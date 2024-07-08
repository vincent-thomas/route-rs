use std::collections::HashMap;

pub mod method;
pub mod respondable;
pub mod variable;

#[derive(Debug, Clone)]
pub struct HttpRequest {
  pub variables: HashMap<String, variable::VariableValue>,
}
