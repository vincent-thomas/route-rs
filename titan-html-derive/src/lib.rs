use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Field, Fields, ItemStruct, LitStr};

// This module and crate::prelude::__private_validatecss::* is the biggest hack of the century
mod private {
  include!("../../titan-html/src/prelude/__private_validatecss.rs");
}

#[proc_macro]
pub fn css(input: TokenStream) -> TokenStream {
  // Parse the input into a string literal
  let input = parse_macro_input!(input as LitStr);

  // Convert the string literal into a &str
  let result = input.value();

  if let Err(err) = private::validate_css(&result) {
    match err {
      private::CSSValidationError::FieldError(field) => {
        let span = input.span();

        let error_msg = format!("Invalid css property name: {}", field);

        let err = syn::Error::new(span, error_msg);
        return err.to_compile_error().into(); // Return the error as a TokenStream
      }
      private::CSSValidationError::EntireFile(location) => {
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

  // Generate the code that returns a reference to the string literal (&str)
  let expanded = quote! {
    #result
  };

  // Convert the generated code back into a TokenStream and return it
  TokenStream::from(expanded)
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
          pub fn add_class(mut self, class: crate::tags::TagClass) -> Self {
            self.classes.insert(class);
            self
          }
          pub fn add_id(mut self, id: impl Into<String>) -> Self {
            self.ids.push(id.into());
            self
          }

          pub fn add_attribute(mut self, key: String, value: String) -> Self {
            self.attributes.insert(key, value);
            self
          }

          pub fn styles(mut self, str_styles: &str) -> Self {
            if let Err(err) = crate::prelude::__private_validatecss::validate_css(&str_styles) {
              match err {
                crate::prelude::__private_validatecss::CSSValidationError::FieldError(field) => {
                  panic!("Invalid css property name: {}", field);
                }
                crate::prelude::__private_validatecss::CSSValidationError::EntireFile(location) => {
                 panic!(
                   "Error parsing css at line = {}, col = {}",
                   location.line, location.column
                 );
                }
              }
            };

            self.add_class(crate::tags::TagClass::Style(str_styles.to_string()))
          }
      }
  };

  TokenStream::from(expanded)
}
