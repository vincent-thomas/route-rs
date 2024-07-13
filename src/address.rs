#[derive(Clone, Debug)]
pub struct Address(pub u8, pub u8, pub u8, pub u8);

impl Default for Address {
  fn default() -> Self {
    Address(0, 0, 0, 0)
  }
}

impl From<(u8, u8, u8, u8)> for Address {
  fn from(value: (u8, u8, u8, u8)) -> Self {
    Address(value.0, value.1, value.2, value.3)
  }
}

impl From<Address> for String {
  fn from(val: Address) -> Self {
    format!("{}.{}.{}.{}", val.0, val.1, val.2, val.3)
  }
}
