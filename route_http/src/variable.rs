#[derive(Debug, Clone)]
pub struct ExtractVariableError;
#[derive(Debug, Clone)]
pub struct VariableValue {
  raw_value: String,
}

impl VariableValue {
  pub fn new(value: String) -> Self {
    VariableValue { raw_value: value }
  }
}

pub trait ExtractVariable<Output> {
  fn extract(self) -> Result<Output, ExtractVariableError>;
}

macro_rules! extract_int {
    ($($t:ty)*) => {
        $(
            impl ExtractVariable<$t> for VariableValue {
                fn extract(self) -> Result<$t, ExtractVariableError> {
                    let test = self
                        .raw_value
                        .parse::<$t>()
                        .map_err(|_| ExtractVariableError);
                    return test;
                }
            }
        )*
    }
}

extract_int!(i8 i16 i32 i64 i128 u8 u16 u32 u64 u128);

impl ExtractVariable<String> for VariableValue {
  fn extract(self) -> Result<String, ExtractVariableError> {
    return Ok(self.raw_value);
  }
}
impl ExtractVariable<Vec<String>> for VariableValue {
  fn extract(self) -> Result<Vec<String>, ExtractVariableError> {
    let variables: Vec<String> = self.raw_value.split("/").map(|v| v.to_string()).collect();
    return Ok(variables);
  }
}
