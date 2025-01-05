use proc_macro::TokenStream;
use quote::quote;
use syn::{Field, Fields, ItemStruct};

#[proc_macro]
pub fn html_tag(item: TokenStream) -> TokenStream {
  let mut item_struct = syn::parse::<ItemStruct>(item).unwrap();
  let struct_name = &item_struct.ident;

  let new_field: Vec<Field> = Vec::from_iter([
    syn::parse_quote! { pub classes: Vec<TagClass> },
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
          pub fn add_class(mut self, class: TagClass) -> Self {
              self.classes.push(class);
              self
          }
          pub fn add_id(mut self, id: impl Into<String>) -> Self {
              self.ids.push(id.into());
              self
          }
          pub fn style(mut self, style: crate::stylerule::StyleRule) -> Self {
              self.add_class(TagClass::Style(style))
          }

          pub fn add_attribute(mut self, key: String, value: String) -> Self {
              self.attributes.insert(key, value);
              self
          }

          pub fn style_from_iter<'a>(mut self, style: impl IntoIterator<Item = (&'a str, &'a str)>) -> Self {
              self.add_class(TagClass::Style(crate::stylerule::StyleRule::from_iter(style)))
          }
          pub fn styles(mut self, styles: &str) -> Self {

              use lightningcss::stylesheet::ParserOptions;
              use lightningcss::printer::PrinterOptions;
              use lightningcss::properties::{custom::CustomPropertyName, Property};

              let mut parser_input = cssparser::ParserInput::new(styles);
              let mut parser = cssparser::Parser::new(&mut parser_input);
              let styles = lightningcss::declaration::DeclarationBlock::parse(&mut parser, &ParserOptions::default()).expect("Invalid css");

              let mut nice_styles: Vec<(String,String)> = Vec::default();

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
                      panic!("Invalid css property name: {}", name);
                  }
                  let mut nice_str: String = item.to_css_string(false, PrinterOptions::default()).unwrap();
                  let iter: Vec<&str> = nice_str.split(": ").collect();
                  let key = iter[0];
                  let value = iter[1];

                  nice_styles.push((key.to_string(), value.to_string()));

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
                      panic!("Invalid css property name: {}", name);
                  }

                  let mut nice_str: String = item.to_css_string(false, PrinterOptions::default()).unwrap();
                  let iter: Vec<&str> = nice_str.split(": ").collect();
                  let key = iter[0];
                  let value = iter[1];

                  nice_styles.push((key.to_string(), value.to_string()));
              }

              self.add_class(TagClass::Style(crate::stylerule::StyleRule::from_iter(nice_styles)))
          }
      }
  };

  TokenStream::from(expanded)
}
