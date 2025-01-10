#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub(crate) enum Segment {
  Slash,
  Exact(String),
  Param(String),
}

pub(crate) trait CompareSegment {
  fn eq(&self, may_be_dynamic: &Self) -> CompareSegmentOut;
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum CompareSegmentOut {
  NoMatch,
  Match(Option<(String, String)>),
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
}
