mod impl_route_get;
use impl_route_get::impl_route_get;

extern crate proc_macro;
use proc_macro::TokenStream;
use syn::ItemFn;

#[proc_macro_attribute]
pub fn get(args: TokenStream, input: TokenStream) -> TokenStream {
  let ast: ItemFn = syn::parse2(input.into()).unwrap();

  impl_route_get(ast, args).into()
}
