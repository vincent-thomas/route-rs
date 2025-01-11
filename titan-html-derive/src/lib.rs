use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, Field, Fields, ItemStruct, LitStr};
use titan_html_core::StyleRule;
use titan_utils::validatecss::{self, CSSValidationError};

fn from_stylerule_to_tokenstream(rules: StyleRule) -> TokenStream2 {
  let styles_tokens: Vec<TokenStream2> = rules
    .styles
    .iter()
    .map(|(key, value)| {
      quote::quote! { (#key.into(), #value.into()) }
    })
    .collect();
  let rule = rules.rule;

  quote::quote! {
      titan::html::StyleRule {
          rule: #rule.to_string(),
          styles: vec![#(#styles_tokens),*],
      }
  }
}

#[proc_macro]
pub fn global_css(input: TokenStream) -> TokenStream {
  // Parse the input into a string literal

  let input = parse_macro_input!(input as LitStr);
  let result = input.value();

  let err = validatecss::validate_globalcss(&result);

  quote! { titan::html::tags::style::Style::Text(#err.to_string()) }.into()
}

#[proc_macro]
pub fn css(input: TokenStream) -> TokenStream {
  // Parse the input into a string literal

  let input = parse_macro_input!(input as LitStr);
  let result = input.value();

  if let Err(err) = validatecss::validate_css(&result) {
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

  let rules = titan_html_core::parse_css_block(result)
    .into_iter()
    .map(|x| from_stylerule_to_tokenstream(x));

  quote! { vec![ #(#rules),* ] }.into()
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

          pub fn styles(mut self, style_rules: Vec<titan_html_core::StyleRule>) -> Self {
            for style in style_rules {
              self.classes.insert(crate::tags::TagClass::StyleRule(style));
            };
            self
          }
      }
  };

  TokenStream::from(expanded)
}
