mod stylerule;
mod utils;
pub use stylerule::StyleRule;

use cssparser::{Parser, ParserInput};
use lightningcss::{
  declaration::DeclarationBlock, printer::PrinterOptions,
  stylesheet::ParserOptions,
};

pub fn parse_css_block(string: impl Into<String>) -> Vec<StyleRule> {
  let string = string.into();
  let mut parser_input = ParserInput::new(&string);
  let mut parser = Parser::new(&mut parser_input);
  let parsed_styles =
    DeclarationBlock::parse(&mut parser, &ParserOptions::default())
      .expect("Invalid css");

  let mut nice = Vec::default();

  for style in parsed_styles.declarations {
    let options = PrinterOptions { minify: true, ..Default::default() };
    let id = style.property_id();
    let key = id.name();

    let value_string = style.value_to_css_string(options).unwrap();
    let value = value_string.as_ref();

    let rule = StyleRule::from_iter([(key, value)]);
    nice.push(rule);
  }
  for style in parsed_styles.important_declarations {
    let options = PrinterOptions { minify: true, ..Default::default() };
    let id = style.property_id();
    let key = id.name();
    let value_string = style.value_to_css_string(options).unwrap();
    let value = value_string.as_ref();

    let rule = StyleRule::from_iter([(key, value)]);
    nice.push(rule);
  }
  nice
}
