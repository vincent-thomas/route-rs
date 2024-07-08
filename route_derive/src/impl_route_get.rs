use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, ItemFn, Lit, Stmt};

fn statements_to_quote(statements: Vec<Stmt>) -> TokenStream {
  // Create a TokenStream to collect all the quote! {} blocks
  let mut quote_tokens = TokenStream::new();

  for statement in statements {
    // Convert the statement into a quote! {} block
    let stmt_quote = quote! {
        #statement
    };

    // Extend the quote_tokens with the generated quote! {} block
    quote_tokens.extend(stmt_quote);
  }

  // Return the final TokenStream containing all quote! {} blocks
  quote_tokens
}

pub(crate) fn impl_route_get(
  ast: ItemFn,
  args: proc_macro::TokenStream,
) -> proc_macro2::TokenStream {
  let lit = syn::parse::<syn::Lit>(args).unwrap();

  let path = match lit {
    Lit::Str(string) => string.value(),
    _ => unimplemented!(),
  };

  let fn_name = quote::format_ident!("{}_fn", ast.sig.ident);

  let mut testt = "".to_string();

  for _ in 0..ast.sig.inputs.len() {
    testt.push_str("request,");
  }
  testt.pop();
  let testt = quote! { #testt };

  let block = statements_to_quote(ast.block.stmts);

  let struct_def = quote! {
    pub struct #fn_name;

    impl route::Service for #fn_name {
      fn method(&self) -> route_router::RouteMethod {
        route_router::RouteMethod::Get
      }

      fn path(&self) -> String {
        #path.to_string()
      }

      fn handler(&self) -> fn() -> Box<dyn route_router::HttpResponse> {
        |#testt| #block
      }
    }
  };
  quote! {
    #struct_def
  }
}
