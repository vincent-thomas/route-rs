use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemFn, Lit, Stmt};

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

  let ident = &ast.sig.ident;

  let struct_name = quote::format_ident!("{}_struct", ast.sig.ident);

  let fn_name = quote::format_ident!("{}_fn", ast.sig.ident);

  dbg!(ast.sig.inputs);

  // Get the function so that inputs can be extracted with the FromRequest trait
  let fn_block = &ast.block;
  let fn_statements = &fn_block.stmts;

  let fn_statements_quote = statements_to_quote(fn_statements.clone());

  // Generate the function that will be called by the route
  let gen_fn = quote! {
    fn #fn_name(req: route_http::HttpRequest) -> String {
      #fn_statements_quote
    }
  };

  let gen_struct = quote! {
    struct #struct_name;
  };

  let gen_struct_impl = quote! {
    impl route::Service for #ident {
      fn method(&self) -> route_http::method::HttpMethod {
        route_http::method::HttpMethod::Get
      }

      fn path(&self) -> String {
        #path.to_string()
      }
    }
  };

  quote! {
    #gen_struct
    #gen_struct_impl
    #gen_fn
  }
}
