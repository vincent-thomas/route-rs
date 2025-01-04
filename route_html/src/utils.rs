const BASE62: &[u8] =
  b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
pub fn encode_base62(mut num: u64) -> String {
  let mut result = String::new();
  while num > 0 {
    let remainder = (num % 62) as usize;
    result.push(BASE62[remainder] as char);
    num /= 62;
  }
  result.chars().rev().collect()
}
