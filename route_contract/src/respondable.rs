pub trait Respondable: Clone
where
  Self: Sized,
{
  fn body(self) -> String;
  fn headers(self) -> Vec<String>;
}

impl Respondable for String {
  fn body(self) -> String {
    self
  }
  fn headers(self) -> Vec<String> {
    vec![]
  }
}
macro_rules! impl_respondable_tostring {
  ($($t:ty)*) => {
    $(
      impl Respondable for $t {
        fn body(self) -> String {
          self.to_string()
        }
        fn headers(self) -> Vec<String> {
          vec![]
        }
      }
    )*
  }
}

impl_respondable_tostring!(&str i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize f32 f64 bool char);
