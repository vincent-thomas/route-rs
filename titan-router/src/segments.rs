use std::collections::HashMap;

use crate::segment::{CompareSegment, CompareSegmentOut, Segment};

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
    }
    count
  }
  pub(crate) fn find(&self, _extern: &[Segment]) -> FindSegmentResult {
    if self.0.len() != _extern.len() {
      return FindSegmentResult::NoMatch;
    }
    let mut ctx = FindSegmentResult::NoMatch;

    for (contract, request) in self.0.iter().zip(_extern) {
      match CompareSegment::eq(contract, request) {
        CompareSegmentOut::NoMatch => return FindSegmentResult::NoMatch,
        CompareSegmentOut::Match(None) => {
          if FindSegmentResult::NoMatch == ctx {
            ctx = FindSegmentResult::Match(HashMap::default());
          }
        }
        CompareSegmentOut::Match(Some(value)) => match ctx {
          FindSegmentResult::NoMatch => {
            ctx = FindSegmentResult::Match(HashMap::from_iter([value]));
          }
          FindSegmentResult::Match(ref mut value1) => {
            value1.insert(value.0, value.1);
          }
        },
      }
    }
    ctx
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

impl From<&'static str> for Segments {
  fn from(value: &'static str) -> Self {
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

      if this_byte == ':' {
        let mut var_name = String::new();
        loop {
          if iter.peek().is_some_and(|v| *v == '/') {
            // End of dynamic string
            break;
          }

          if iter.peek().is_none() {
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
    let test = "/test";

    let result = Segments::from(test);

    let expected_result =
      Vec::from_iter([Segment::Slash, Segment::Exact("test".to_string())]);

    assert_eq!(result.0, expected_result);
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
