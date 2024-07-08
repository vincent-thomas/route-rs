
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum HttpMethod {
  Get,
  Post,
  Patch,
  Put,
  Delete,
  Options,
  Head,
  Trace,
}

pub struct InvalidHttpMethod;

impl TryFrom<&str> for HttpMethod {
  type Error = InvalidHttpMethod;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    match value {
      "GET" => Ok(HttpMethod::Get),
      "POST" => Ok(HttpMethod::Post),
      "PATCH" => Ok(HttpMethod::Patch),
      "PUT" => Ok(HttpMethod::Put),
      "DELETE" => Ok(HttpMethod::Delete),
      "OPTIONS" => Ok(HttpMethod::Options),
      "HEAD" => Ok(HttpMethod::Head),
      "TRACE" => Ok(HttpMethod::Trace),
      _ => Err(InvalidHttpMethod),
    }
  }
}

impl HttpMethod {
  pub fn is_idempotent(&self) -> bool {
    match self {
      HttpMethod::Get
      | HttpMethod::Head
      | HttpMethod::Options
      | HttpMethod::Trace
      | HttpMethod::Delete
      | HttpMethod::Patch
      | HttpMethod::Put => true,
      HttpMethod::Post => false,
    }
  }

  pub fn is_safe(&self) -> bool {
    match self {
      HttpMethod::Get | HttpMethod::Head | HttpMethod::Options | HttpMethod::Trace => true,
      HttpMethod::Post | HttpMethod::Delete | HttpMethod::Patch | HttpMethod::Put => false,
    }
  }

  pub fn is_cacheable(&self) -> bool {
    match self {
      HttpMethod::Get | HttpMethod::Head => true,
      HttpMethod::Options
      | HttpMethod::Trace
      | HttpMethod::Post
      | HttpMethod::Delete
      | HttpMethod::Patch
      | HttpMethod::Put => false,
    }
  }
}
