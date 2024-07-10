#[derive(Clone)]
pub struct Address(pub u8, pub u8, pub u8, pub u8);

impl From<Address> for String {
  fn from(val: Address) -> Self {
    format!("{}.{}.{}.{}", val.0, val.1, val.2, val.3)
  }
}
