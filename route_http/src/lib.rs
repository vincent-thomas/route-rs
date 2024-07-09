use std::collections::HashMap;

mod macros;
pub mod method;
pub mod request;
pub mod response;
pub mod status;
pub mod variable;

#[derive(Debug, Clone)]
pub struct HttpRequest {
  pub variables: HashMap<String, variable::VariableValue>,
}
