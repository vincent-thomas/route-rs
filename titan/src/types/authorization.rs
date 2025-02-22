use titan_core::{FromRequestParts, Respondable};
use titan_http::{Parts, StatusCode};

pub enum AuthorizationError {
  Invalid,
  DoesntExist,
}

impl Respondable for AuthorizationError {
  fn respond(self) -> titan_http::Response<titan_http::body::Body> {
    match self {
      AuthorizationError::Invalid => {
        (StatusCode::BAD_REQUEST, "Invalid Authorization format").respond()
      }
      AuthorizationError::DoesntExist => {
        (StatusCode::UNAUTHORIZED, "Unauthorized").respond()
      }
    }
  }
}

impl TryFrom<&str> for AuthorizationType {
  type Error = AuthorizationError;
  fn try_from(value: &str) -> Result<Self, Self::Error> {
    match value.to_lowercase().as_str() {
      "bearer" => Ok(Self::Bearer),
      "basic" => Ok(Self::Basic),
      _ => Err(AuthorizationError::Invalid),
    }
  }
}

#[derive(Debug)]
pub struct Authorization {
  pub _type: AuthorizationType,
  pub value: String,
}
impl FromRequestParts for Authorization {
  type Error = AuthorizationError;
  fn from_request_parts(parts: &mut Parts) -> Result<Self, Self::Error> {
    let auth_header = match parts.headers.get("authorization") {
      Some(v) => v,
      None => return Err(AuthorizationError::DoesntExist),
    }
    .to_str()
    .ok();

    let Some(header) = auth_header else {
      return Err(AuthorizationError::Invalid);
    };

    let mut trimed = header.split_whitespace();

    let _type = AuthorizationType::try_from(trimed.next().unwrap())?;
    let value = trimed.next().unwrap();

    if trimed.next().is_some() {
      return Err(AuthorizationError::Invalid);
    }

    Ok(Authorization { _type, value: value.to_string() })
  }
}
#[derive(Debug, PartialEq)]
pub enum AuthorizationType {
  Bearer,
  Basic,
}

macro_rules! diff_auth_types {
  ($($type:ident)*) => {
    $(
        pub struct $type(pub String);

        impl FromRequestParts for $type {
          type Error = AuthorizationError;
          fn from_request_parts(req: &mut Parts) -> Result<Self, Self::Error> {
            let test = Authorization::from_request_parts(req);

            let Ok(result) = test else {
              let err = test.unwrap_err();

              return Err(err);
            };

            if AuthorizationType::$type != result._type {
              return Err(AuthorizationError::Invalid);
            };

            Ok($type(result.value))
          }
        }
    )*
  };
}

diff_auth_types! { Basic Bearer }
