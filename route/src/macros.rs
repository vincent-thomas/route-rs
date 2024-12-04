#[macro_export]
macro_rules! panic_err {
  ($result:expr, $error_str:expr) => {
    match $result {
      Ok(value) => value,
      Err(err) => panic!($error_str, err),
    }
  };
  ($result:expr) => {
    match $result {
      Ok(value) => value,
      Err(err) => panic!($error_str, "Unknown error: {:?}", err),
    }
  };
}
