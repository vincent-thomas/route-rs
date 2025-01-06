use std::collections::HashMap;

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
pub(crate) enum Segment {
  Slash,
  Exact(String),
  Param(String),
}

#[derive(Default, Hash, Eq, PartialEq, Clone)]
pub(crate) struct Segments(pub(crate) Vec<Segment>);

pub(crate) struct Params(pub(crate) HashMap<String, String>);

impl Segments {
  pub(crate) fn num_params(&self) -> usize {
    let mut count = 0;
    for item in &self.0 {
      if let Segment::Param(_) = item {
        count += 1;
      }
    }
    count
  }
  pub(crate) fn find(&self, _extern: &[Segment]) -> Option<Params> {
    let mut params = HashMap::new();

    if self.0.len() != _extern.len() {
      return None;
    }

    for (index, segment) in self.0.iter().enumerate() {
      match _extern.get(index)? {
        Segment::Param(_) => unreachable!("Doesnt show here"),
        Segment::Exact(segment_str_request) => {
          if let Segment::Param(key) = segment {
            params.insert(key.clone(), segment_str_request.clone());
            continue;
          }
          if let Segment::Exact(segment_str_route) = segment {
            if segment_str_request == segment_str_route {
              Segment::Exact(segment_str_request.to_string())
            } else {
              break;
            }
          } else {
            break;
          }
        }
        Segment::Slash => {
          if *segment == Segment::Slash {
            Segment::Slash
          } else {
            break;
          }
        }
      };
    }
    Some(Params(params))
  }
}

#[cfg(test)]
mod test2 {
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

    let Some(testing) = result else {
      panic!();
    };

    let hash: HashMap<String, String> =
      HashMap::from_iter([("test".to_string(), "test".to_string())]);

    assert_eq!(testing.0, hash);
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
mod tests {
  use super::*;
  #[test]
  fn static_routes() {
    let test = "/test";

    let result = Segments::from(test);

    let expected_result =
      Vec::from_iter([Segment::Slash, Segment::Exact("test".to_string())]);

    assert_eq!(result.0, expected_result);
  }

  #[test]
  fn dynamic_routes() {
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
