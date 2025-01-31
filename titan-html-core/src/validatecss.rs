use cssparser::SourceLocation;
use lightningcss::{
  printer::PrinterOptions,
  properties::{custom::CustomPropertyName, Property},
  stylesheet::{ParserOptions, StyleSheet},
};

pub enum CSSValidationError {
  FieldError(String),
  EntireFile(SourceLocation),
}
pub fn validate_css(value: &str) -> Result<(), CSSValidationError> {
  let mut parser_input = cssparser::ParserInput::new(value);
  let mut parser = cssparser::Parser::new(&mut parser_input);

  let styles = match lightningcss::declaration::DeclarationBlock::parse(
    &mut parser,
    &ParserOptions::default(),
  ) {
    Ok(value) => value,
    Err(err) => return Err(CSSValidationError::EntireFile(err.location)),
  };

  for item in styles.declarations {
    if let Property::Custom(custom) = item.clone() {
      use std::ops::Deref;
      let name = match custom.name {
        CustomPropertyName::Custom(dashed_ident) => {
          let deref = Box::new(dashed_ident);
          deref.deref().to_string()
        }
        CustomPropertyName::Unknown(ident) => {
          let deref = Box::new(ident);
          deref.deref().to_string()
        }
      };
      return Err(CSSValidationError::FieldError(name));
    }
  }

  for item in styles.important_declarations {
    if let Property::Custom(custom) = item.clone() {
      use std::ops::Deref;
      let name = match custom.name {
        CustomPropertyName::Custom(dashed_ident) => {
          let deref = Box::new(dashed_ident);
          deref.deref().to_string()
        }
        CustomPropertyName::Unknown(ident) => {
          let deref = Box::new(ident);
          deref.deref().to_string()
        }
      };
      return Err(CSSValidationError::FieldError(name));
    }
  }
  Ok(())
}

pub fn validate_globalcss(value: &str) -> String {
  let styles = match StyleSheet::parse(value, ParserOptions::default()) {
    Ok(value) => value,
    Err(_) => panic!("invalid global css"),
  };

  styles
    .to_css(PrinterOptions { minify: true, ..Default::default() })
    .unwrap()
    .code
}
