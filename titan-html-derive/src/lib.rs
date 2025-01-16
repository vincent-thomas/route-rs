use lightningcss::{printer::PrinterOptions, properties::Property};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, Field, Fields, Ident, ItemStruct, LitStr};
use titan_utils::validatecss::{
  validate_css, validate_globalcss, CSSValidationError,
};

fn from_stylerules_to_tokenstream(
  prop: Vec<(String, Property<'_>)>,
) -> TokenStream2 {
  let styles_tokens: Vec<TokenStream2> = prop
    .iter()
    .map(|(hash, prop)| {
      let prop_id = prop.property_id();
      let key = prop_id.name();

      let value = prop.value_to_css_string(PrinterOptions::default()).unwrap();

      quote::quote! {
          titan::html::StyleRule {
              rule: #hash,
              styles: &[(#key, #value)],
          }
      }
    })
    .collect();

  quote::quote! {
      #(#styles_tokens),*
  }
}
//
//// Helper function to find and extract variables from CSS values
//fn extract_variables(css: &str) -> Vec<(usize, usize, String)> {
//  let mut variables = Vec::new();
//  let mut start = 0;
//  while let Some(var_start) = css[start..].find("{") {
//    if let Some(var_end) = css[start + var_start..].find('}') {
//      let abs_start = start + var_start;
//      let abs_end = start + var_start + var_end + 1;
//      let var_name = css[abs_start + 1..abs_end - 1].to_string();
//      variables.push((abs_start, abs_end, var_name));
//      start = abs_end;
//    } else {
//      break;
//    }
//  }
//  variables
//}
//
//#[derive(Debug)]
//enum StringBit {
//  Var(Ident),
//  Text(String),
//}

#[proc_macro]
pub fn global_css(input: TokenStream) -> TokenStream {
  // Parse the input into a string literal

  let input = parse_macro_input!(input as LitStr);
  let result = input.value();

  let err = validate_globalcss(&result);

  quote! { titan::html::tags::Style::Text(#err.to_string()) }.into()
}

#[proc_macro]
pub fn css(input: TokenStream) -> TokenStream {
  // Parse the input into a string literal

  let input = parse_macro_input!(input as LitStr);
  let result = input.value();

  if let Err(err) = validate_css(&result) {
    match err {
      CSSValidationError::FieldError(field) => {
        let span = input.span();

        let error_msg = format!("Invalid css property name: {}", field);

        let err = syn::Error::new(span, error_msg);
        return err.to_compile_error().into(); // Return the error as a TokenStream
      }
      CSSValidationError::EntireFile(location) => {
        let span = input.span();

        let error_msg = format!(
          "Error parsing css at line = {}, col = {}",
          location.line, location.column
        );

        let err = syn::Error::new(span, error_msg);
        return err.to_compile_error().into(); // Return the error as a TokenStream
      }
    }
  };

  let rules = titan_html_core::parse_css_block(&result);

  let rules = from_stylerules_to_tokenstream(rules);

  quote! { &[#rules] }.into()
}

#[proc_macro]
pub fn html_tag(item: TokenStream) -> TokenStream {
  let mut item_struct = syn::parse::<ItemStruct>(item).unwrap();
  let struct_name = &item_struct.ident;

  let new_field: Vec<Field> = Vec::from_iter([
    syn::parse_quote! { pub classes: std::collections::HashSet<crate::tags::TagClass> },
    syn::parse_quote! { pub ids: Vec<String> },
    syn::parse_quote! { pub attributes: HashMap<String, String> },
  ]);

  let Fields::Named(fields_named) = &mut item_struct.fields else {
    return syn::Error::new_spanned(
      item_struct,
      "Only named fields are supported",
    )
    .to_compile_error()
    .into();
  };

  // Append the new field to the struct
  for field in new_field {
    fields_named.named.push(field);
  }

  let expanded = quote! {
      #[derive(Clone)]
      #item_struct

      impl #struct_name {
          pub fn class(mut self, class: impl Into<String>) -> Self {
            let classes: Vec<String> = class.into().split(' ').map(|x| x.to_string()).collect();

            for class in classes {
              let key = crate::tags::TagClass::text(class.clone());
              if self.classes.contains(&key) {
                eprintln!("warning: class '{class}' already is defined in element: {}", stringify!(#struct_name));
              }
              self.classes.insert(key);
            }
            self
          }

          pub fn id(mut self, id: impl Into<String>) -> Self {
            self.ids.push(id.into());
            self
          }

          pub fn add_attribute(mut self, key: String, value: String) -> Self {
            self.attributes.insert(key, value);
            self
          }

          pub fn styles(mut self, style_rules: &[titan_html_core::StyleRule]) -> Self {
            for style in style_rules {
              self.classes.insert(crate::tags::TagClass::StyleRule(style.clone()));
            };
            self
          }
      }
  };

  TokenStream::from(expanded)
}
