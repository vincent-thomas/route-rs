use route_http::request::Request;

pub trait FromRequest: Sized {
  type Error: Into<()>;
  fn from_request(req: Request) -> Result<Self, Self::Error>;
  fn extract(req: Request) -> Result<Self, Self::Error> {
    Self::from_request(req)
  }
}

impl FromRequest for String {
  type Error = ();
  fn from_request(req: Request) -> Result<Self, Self::Error> {
    let test = req.into_body();
    Ok(String::from_utf8_lossy(&test).to_string())
  }
}

macro_rules! impl_fromrequest_tuple {
  ($($name:ident),*) => {
    impl<$($name: FromRequest),*> FromRequest for ($($name,)*) {
      type Error = ();
      #[allow(unused_variables)]
      fn from_request(req: Request) -> Result<Self, Self::Error> {
            let args = ($(match $name::from_request(req.clone()) {Ok(v) => v, Err(_) => unimplemented!(),},)*);
            Ok(args)
      }
    }
  };
}

impl_fromrequest_tuple!();
impl_fromrequest_tuple!(A);
impl_fromrequest_tuple!(A, B);
impl_fromrequest_tuple!(A, B, C);
impl_fromrequest_tuple!(A, B, C, D);
impl_fromrequest_tuple!(A, B, C, D, E);
impl_fromrequest_tuple!(A, B, C, D, E, F);
impl_fromrequest_tuple!(A, B, C, D, E, F, G);
impl_fromrequest_tuple!(A, B, C, D, E, F, G, H);
impl_fromrequest_tuple!(A, B, C, D, E, F, G, H, I);
impl_fromrequest_tuple!(A, B, C, D, E, F, G, H, I, J);
impl_fromrequest_tuple!(A, B, C, D, E, F, G, H, I, J, K);
impl_fromrequest_tuple!(A, B, C, D, E, F, G, H, I, J, K, L);
