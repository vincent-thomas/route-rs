use proc_macro::TokenStream;
use quote::quote;
use syn::{Field, Fields, ItemStruct};

#[proc_macro]
pub fn html_tag(item: TokenStream) -> TokenStream {
  let mut item_struct = syn::parse::<ItemStruct>(item).unwrap();
  let struct_name = &item_struct.ident;

  // Define a new field to add
  //let new_field =
  let new_field: Vec<Field> = Vec::from_iter([
    syn::parse_quote! { pub classes: Vec<TagClass> },
    syn::parse_quote! { pub ids: Vec<String> },
  ]);

  // Append the new field to the struct
  match &mut item_struct.fields {
    Fields::Named(fields_named) => {
      for field in new_field {
        fields_named.named.push(field);
      }
    }
    _ => {
      return syn::Error::new_spanned(
        item_struct,
        "Only named fields are supported",
      )
      .to_compile_error()
      .into();
    }
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
          pub fn style(mut self, style: crate::style::StyleRule) -> Self {
              self.add_class(TagClass::Style(style))
          }

          pub fn style_from_iter<'a>(mut self, style: impl IntoIterator<Item = (&'a str, &'a str)>) -> Self {
              self.add_class(TagClass::Style(crate::style::StyleRule::from_iter(style)))
          }
          pub fn styles(mut self, styles: impl IntoIterator<Item = crate::style::StyleRule>) -> Self {
              let mut styles_iter = styles.into_iter();
              while let Some(value) = styles_iter.next() {
                 self = self.style(value.clone());
              };
              self
          }

      }
  };

  TokenStream::from(expanded)
}
