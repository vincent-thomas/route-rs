#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub(crate) enum Segment {
  Slash,
  Exact(String),
  Param(String),
  Rest,
}

impl std::fmt::Display for Segment {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Segment::Slash => f.write_str("/"),
      Segment::Exact(string) => f.write_str(string),
      Segment::Param(param) => f.write_str(format!(":{}", param).as_str()),
      Segment::Rest => f.write_str("**"),
    }
  }
}

pub(crate) trait CompareSegment {
  fn eq(&self, may_be_dynamic: &Self) -> CompareSegmentOut;
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum CompareSegmentOut {
  NoMatch,
  Match(Option<(String, String)>),
  MatchRestPartValue(String),
}

impl CompareSegment for Segment {
  fn eq(&self, from_request: &Self) -> CompareSegmentOut {
    match self {
      Segment::Slash => {
        if matches!(from_request, Segment::Slash) {
          CompareSegmentOut::Match(None)
        } else {
          CompareSegmentOut::NoMatch
        }
      }

      Segment::Exact(str) => match from_request {
        Segment::Exact(str_contract) if str_contract == str => {
          CompareSegmentOut::Match(None)
        }

        _ => CompareSegmentOut::NoMatch,
      },
      Segment::Param(param_name) => match from_request {
        Segment::Exact(param_value) => CompareSegmentOut::Match(Some((
          param_name.clone(),
          param_value.clone(),
        ))),
        _ => CompareSegmentOut::NoMatch,
      },
      Segment::Rest => match from_request {
        Segment::Exact(value) => {
          CompareSegmentOut::MatchRestPartValue(value.clone())
        }
        Segment::Slash => {
          CompareSegmentOut::MatchRestPartValue("/".to_string())
        }
        _ => CompareSegmentOut::NoMatch,
      },
    }
  }
}

#[cfg(test)]
mod testing {
  use super::CompareSegmentOut;

  use super::{CompareSegment, Segment};

  #[test]
  fn test_compare_segment_param_value() {
    let contract = Segment::Param("variable".to_string());
    let request = Segment::Exact("value".to_string());
    let request2 = Segment::Slash;

    assert_eq!(
      CompareSegment::eq(&contract, &request),
      CompareSegmentOut::Match(Some((
        "variable".to_string(),
        "value".to_string(),
      )))
    );

    assert_eq!(
      CompareSegment::eq(&contract, &request2),
      CompareSegmentOut::NoMatch
    );
  }

  #[test]
  fn test_compare_segment_exact() {
    let contract = Segment::Exact("value".to_string());
    let request = Segment::Exact("value".to_string());

    assert_eq!(
      CompareSegment::eq(&contract, &request),
      CompareSegmentOut::Match(None)
    );

    let contract = Segment::Exact("value1".to_string());
    let request = Segment::Exact("value".to_string());

    assert_eq!(
      CompareSegment::eq(&contract, &request),
      CompareSegmentOut::NoMatch,
    );
  }

  #[test]
  fn test_compare_segment_slash() {
    let contract = Segment::Slash;
    let request = Segment::Slash;

    assert_eq!(
      CompareSegment::eq(&contract, &request),
      CompareSegmentOut::Match(None)
    );

    let request2 = Segment::Exact("something".to_string());

    assert_eq!(
      CompareSegment::eq(&contract, &request2),
      CompareSegmentOut::NoMatch,
    );
  }
}
