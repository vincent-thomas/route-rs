use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream as TokenStream2;
use syn::{
  braced,
  parse::{Parse, ParseStream},
  parse_macro_input, Expr, LitStr, Token,
};

// Define a struct to represent a single "/path" => ident entry
struct PathMapping {
  path: LitStr,
  _arrow: Token![=>],
  ident: Expr,
}

// Define a struct to represent the entire block
struct PathMappings {
  mappings: Vec<PathMapping>,
}

impl Parse for PathMapping {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    let path = input.parse()?; // Parse the string literal ("/path")
    let _arrow = input.parse()?; // Parse the => token
    let ident = input.parse()?; // Parse the single identifier
    Ok(PathMapping { path, _arrow, ident })
  }
}

impl Parse for PathMappings {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    let content;
    let _braces = braced!(content in input); // Parse the enclosing {}

    let mut mappings = Vec::new();
    while !content.is_empty() {
      mappings.push(content.parse()?); // Parse each "/path" => ident entry
      if content.peek(Token![,]) {
        content.parse::<Token![,]>()?; // Consume the comma, if present
      }
    }

    Ok(PathMappings { mappings })
  }
}

#[proc_macro]
pub fn define_routes(input: TokenStream1) -> TokenStream1 {
  let path_mappings = parse_macro_input!(input as PathMappings);

  impl_define_routes(path_mappings).into()
}

fn impl_define_routes(mappings: PathMappings) -> TokenStream2 {
  let mut base = Vec::from_iter([quote::quote! {
      let mut router = Router::default();
  }]);

  for mapping in mappings.mappings {
    let key = mapping.path;
    let value = mapping.ident;
    base.push(quote::quote! {
        router.at(#key, #value);
    });
  }

  quote::quote! {{
     #(#base)*

     router
  }}
}
