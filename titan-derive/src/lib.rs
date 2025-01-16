use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::parse_macro_input;
use syn::ItemFn;

#[proc_macro_attribute]
pub fn ssg(_: TokenStream, input: TokenStream) -> TokenStream {
  let item = parse_macro_input!(input as ItemFn);

  if !item.sig.inputs.is_empty() {
    return syn::Error::new(
      item.sig.ident.span(),
      "Error: SSG routes cannot have arguments",
    )
    .into_compile_error()
    .into();
  }
  impl_ssg(item).into()
}

fn impl_ssg(item: ItemFn) -> TokenStream2 {
  let struct_ident = item.sig.ident.clone();

  let ident_cache_str =
    format!("{}_CACHE", item.sig.ident.to_string().to_uppercase());
  let ident_cache = syn::Ident::new(&ident_cache_str, item.sig.ident.span());

  let nice_fn = item.block;

  quote::quote! {
    titan::lazy_static! {
      static ref #ident_cache: std::sync::RwLock<Option<Vec<u8>>> = std::sync::RwLock::new(None);
    }

    #[allow(non_camel_case_types)]
    #[derive(Clone)]
    pub struct #struct_ident;

    impl titan::Handler<()> for #struct_ident {
      type Output = titan::http::Response;
      type Future =
        std::pin::Pin<Box<dyn std::future::Future<Output = Self::Output> + Send>>;
      fn call(&self, _: ()) -> Self::Future {
        let _lock = match #ident_cache.read() {
          Ok(v) => v,
          Err(_) => panic!("oh no"),
        };

        if let Some(cache) = _lock.as_ref() {
          let body =
            titan::http::body::Body::Full(cache.clone().into_boxed_slice());
          return Box::pin(async move {
            titan::http::ResponseBuilder::new().status(200).body(body).unwrap()
          });
        };

        Box::pin(async move {
          let response = titan::FutureExt::map(async #nice_fn, |x| x.respond()).await;


          match response.body() {
            titan::http::body::Body::Full(ref body) => {
              let mut refs = #ident_cache.write().unwrap();
              *refs = Some(body.clone().to_vec());
            }
            titan::http::body::Body::Stream(_) => {
              panic!(
                "Body::Stream is not available in a cached request response :("
              )
            }
          };
          response
        })
      }
    }
  }
}
