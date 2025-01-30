#![forbid(unsafe_code)]
mod stylerule;
use std::hash::{DefaultHasher, Hasher};

pub use stylerule::StyleRule;

use cssparser::{Parser, ParserInput};
use lightningcss::{
  declaration::DeclarationBlock, printer::PrinterOptions, properties::Property,
  stylesheet::ParserOptions,
};

pub fn parse_css_block(string: &str) -> Vec<(String, Property<'_>)> {
  let mut parser_input = ParserInput::new(string);
  let mut parser = Parser::new(&mut parser_input);
  let parsed_styles =
    DeclarationBlock::parse(&mut parser, &ParserOptions::default())
      .expect("Invalid css");

  let mut nice = Vec::default();

  for style in parsed_styles.declarations {
    let mut hasher = DefaultHasher::default();
    hasher.write(style.property_id().name().as_bytes());
    let options = PrinterOptions { minify: true, ..Default::default() };
    hasher.write(style.value_to_css_string(options).unwrap().as_bytes());
    let hash = encode_base62(hasher.finish());
    nice.push((format!("r{hash}"), style));
  }
  for style in parsed_styles.important_declarations {
    let mut hasher = DefaultHasher::default();
    hasher.write(style.property_id().name().as_bytes());
    let options = PrinterOptions { minify: true, ..Default::default() };
    hasher.write(style.value_to_css_string(options).unwrap().as_bytes());
    let hash = encode_base62(hasher.finish());
    nice.push((hash, style));
  }

  nice
}

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
