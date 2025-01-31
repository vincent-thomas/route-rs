use std::collections::HashMap;

use super::segment::{CompareSegment, CompareSegmentOut, Segment};

#[derive(Default, Hash, Clone, Debug, Eq, PartialEq)]
pub struct Segments(pub(crate) Vec<Segment>);

#[derive(PartialEq, Debug)]
pub enum FindSegmentResult {
  NoMatch,
  Match(HashMap<String, String>),
}

impl std::fmt::Display for Segments {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for item in self.0.iter() {
      f.write_str(item.to_string().as_str())?;
    }
    Ok(())
  }
}

impl Segments {
  pub fn num_params(&self) -> usize {
    let mut count = 0;
    for item in &self.0 {
      if let Segment::Param(_) = item {
        count += 1;
      }

      if let Segment::Rest = item {
        count += 1;
      }
    }
    count
  }
  pub(crate) fn find(&self, _extern: &[Segment]) -> FindSegmentResult {
    let mut iter = self.0.iter().enumerate();

    fn compare_segment(
      contract: &Segment,
      request: &Segment,
      ctx: &mut FindSegmentResult,
    ) {
      match CompareSegment::eq(contract, request) {
        CompareSegmentOut::NoMatch => *ctx = FindSegmentResult::NoMatch,
        CompareSegmentOut::Match(None) => {
          if FindSegmentResult::NoMatch == *ctx {
            *ctx = FindSegmentResult::Match(HashMap::default());
          }
        }
        CompareSegmentOut::Match(Some(value)) => match ctx {
          FindSegmentResult::NoMatch => {
            *ctx = FindSegmentResult::Match(HashMap::from_iter([value]));
          }
          FindSegmentResult::Match(ref mut value1) => {
            value1.insert(value.0, value.1);
          }
        },
        CompareSegmentOut::MatchRestPartValue(rest_value) => {
          if FindSegmentResult::NoMatch == *ctx {
            *ctx = FindSegmentResult::Match(HashMap::default());
          }

          match ctx {
            FindSegmentResult::Match(hashmap) => {
              if let Some(thing) = hashmap.get_mut("_") {
                thing.push_str(&rest_value)
              } else {
                hashmap.insert("_".to_string(), rest_value);
              }
            }
            _ => unreachable!(),
          }
        }
      }
    }

    let mut ctx = FindSegmentResult::NoMatch;

    while let Some((index, contract)) = iter.next() {
      if *contract == Segment::Rest {
        let mut iter = _extern.iter().skip(index);
        while let Some(request) = iter.next() {
          compare_segment(contract, request, &mut ctx);
        }
        break;
      }

      let request = match _extern.get(index) {
        Some(v) => v,
        None => break,
      };

      compare_segment(contract, request, &mut ctx);
    }

    ctx
  }
}

impl From<&'_ str> for Segments {
  fn from(value: &'_ str) -> Self {
    Segments::from(value.to_string())
  }
}

impl From<String> for Segments {
  fn from(value: String) -> Self {
    let mut vec = Vec::new();

    let mut iter = value.chars().peekable();

    if let Some(test) = iter.peek() {
      if *test != '/' {
        vec.push(Segment::Slash);
      }
    }

    while let Some(this_byte) = iter.next() {
      if this_byte == '/' {
        vec.push(Segment::Slash);
        continue;
      }

      if this_byte == '*' {
        if iter.peek().is_some_and(|v| *v != '*') || iter.peek().is_none() {
          panic!("False syntax, '**' signalises catch all");
        }

        iter.next().unwrap();

        vec.push(Segment::Rest);

        if iter.next() != None {
          panic!("False router syntax: ** must at the very end of the path");
        }

        break;
      }

      if this_byte == ':' {
        let mut var_name = String::new();
        loop {
          if iter.peek().is_some_and(|v| *v == '/') || iter.peek().is_none() {
            // End of dynamic string
            break;
          }

          if let Some(test) = iter.next() {
            var_name.push(test);
          };
        }
        vec.push(Segment::Param(var_name));
        continue;
      }

      let mut str = String::from(this_byte);
      while let Some(this_byte) = iter.next() {
        str.push(this_byte);

        if iter.peek().is_some_and(|v| *v == '/') {
          // End of dynamic string
          break;
        }
      }

      vec.push(Segment::Exact(str));
    }

    Segments(vec)
  }
}

#[cfg(test)]
mod tests_segment_parsing {
  use super::*;
  #[test]
  fn test_static() {
    let test = "/test/**";

    let contract = Segments::from(test);

    let request = Segments::from("/test/very/nice/indeed");

    let result = contract.find(&request.0);

    if let FindSegmentResult::Match(result) = result {
      assert_eq!(
        result,
        HashMap::from_iter([("_".to_string(), "very/nice/indeed".to_string())])
      );
    } else {
      panic!("aaaww");
    }
  }

  #[test]
  fn test_dynamic() {
    let test = "/test/:dynamic";

    let result = Segments::from(test);

    let expected_result = Vec::from_iter([
      Segment::Slash,
      Segment::Exact("test".to_string()),
      Segment::Slash,
      Segment::Param("dynamic".to_string()),
    ]);

    assert_eq!(result.0, expected_result);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_test() {
    let registered = Segments(Vec::from_iter([
      Segment::Slash,
      Segment::Param("test".to_string()),
    ]));

    let req =
      Vec::from_iter([Segment::Slash, Segment::Exact("test".to_string())]);

    let result = registered.find(&req);

    let FindSegmentResult::Match(testing) = result else {
      panic!();
    };

    let hash: HashMap<String, String> =
      HashMap::from_iter([("test".to_string(), "test".to_string())]);

    assert_eq!(testing, hash);
  }
}
