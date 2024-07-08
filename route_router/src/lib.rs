use std::{collections::HashMap, fmt::Debug};

use route_contract::{method::HttpMethod, variable::VariableValue};

pub mod error;

#[derive(Debug, Clone)]
struct ResolvableRoute<Req: Clone, Res: Clone> {
  /// This depends on the hashmap key.
  path_variable_name: Option<String>,
  routes: HashMap<HttpMethod, Route<Req, Res>>,
  children: HashMap<RoutePathType, ResolvableRoute<Req, Res>>,
}

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
enum RoutePathType {
  Static(String),
  Variable,
}

// pub trait HttpResponse {
//   fn headers<'a>(&'a self) -> &'a [String];
//   fn body(&self) -> String;
// }

// impl HttpResponse for String {
//   fn body(&self) -> String {
//     self.clone()
//   }
//   fn headers<'a>(&'a self) -> &'a [String] {
//     &[]
//   }
// }

// impl HttpResponse for &str {
//   fn body(&self) -> String {
//     self.to_string()
//   }
//   fn headers<'a>(&'a self) -> &'a [String] {
//     &[]
//   }
// }

// macro_rules! impl_http_response {
//   ($($t:ty)*) => {
//     $(
//       impl HttpResponse for $t {
//         fn body(&self) -> String {
//           self.to_string()
//         }
//         fn headers<'a>(&'a self) -> &'a [String] {
//           &[]
//         }
//       }
//     )*
//   }
// }

// impl_http_response! {
// i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize f32 f64 bool char
// }

#[derive(Debug, Clone)]
pub struct Route<Req: Clone, Res: Clone> {
  pub handler: fn(req: Req) -> Res,
}

pub trait FromRequest: Clone {}

impl<Req: Clone, Res: Clone> Route<Req, Res> {
  pub fn new(handler: fn(req: Req) -> Res) -> Self {
    Route { handler }
  }
}

#[derive(Debug, Clone)]
enum RouterMount {
  Root,
  Path(String),
}

impl Into<String> for RouterMount {
  fn into(self) -> String {
    match self {
      RouterMount::Root => "/".into(),
      RouterMount::Path(string) => string,
    }
  }
}

#[derive(Debug)]
pub struct Router<Req, Res>
where
  Req: Clone + Debug,
  Res: Clone + Debug,
{
  mount: RouterMount,
  routes: HashMap<RoutePathType, ResolvableRoute<Req, Res>>,
}

#[derive(Debug)]
enum PathPartType {
  Variable(String),
  Static(String),
}

#[derive(Debug)]
pub struct NoRouteFound;
#[derive(Debug)]
pub struct RouteFound<Req: Clone, Res: Clone> {
  pub route: Route<Req, Res>,
  pub variables: HashMap<String, VariableValue>,
}
impl<Req, Res> Router<Req, Res>
where
  Req: Clone + Debug,
  Res: Clone + Debug,
{
  pub fn mount_root() -> Self {
    Router { mount: RouterMount::Root, routes: HashMap::new() }
  }
  pub fn mount_at(mount_path: impl Into<String>) -> Self {
    Router { mount: RouterMount::Path(mount_path.into()), routes: HashMap::new() }
  }

  pub fn match_route(
    &self,
    method: HttpMethod,
    route: &str,
  ) -> Result<RouteFound<Req, Res>, NoRouteFound> {
    let mount_str: String = self.mount.clone().into();
    let path_without_mount: &str = route.strip_prefix(&mount_str).ok_or(NoRouteFound)?;
    let path_vec: Vec<&str> = path_without_mount.split('/').collect();

    let mut base = self.routes.clone();
    let mut current_route: Option<ResolvableRoute<Req, Res>> = None;
    let mut variables = HashMap::new();

    for item in path_vec {
      let static_output = base.get(&RoutePathType::Static(item.to_string()));

      if static_output.is_some() {
        current_route = Some(static_output.unwrap().clone());
        base = current_route.clone().unwrap().children;
      } else {
        let dynamic_output = base.get(&RoutePathType::Variable);
        if dynamic_output.is_none() {
          return dbg!(Err(NoRouteFound));
        }
        let dynamic_output = dynamic_output.unwrap();
        let variable_name = dynamic_output
          .path_variable_name
          .clone()
          .expect("route-rs: Internal error: Dynamic route without path variable name on route");
        let variable_value = VariableValue::new(item.into());
        variables.insert(variable_name, variable_value);
        current_route = Some(dynamic_output.clone());
        base = current_route.clone().unwrap().children;
      }
    }

    match current_route {
      Some(route) => {
        let route_exists_for_specified_method = route.routes.get(&method);

        match route_exists_for_specified_method {
          None => dbg!(Err(NoRouteFound)),
          Some(route) => dbg!(Ok(RouteFound { variables, route: route.clone() })),
        }
      }
      None => dbg!(Err(NoRouteFound)),
    }
  }

  pub fn route(&mut self, method: HttpMethod, path: String, route: Route<Req, Res>) {
    let path = path.strip_prefix('/').expect("route-rs: route path must start with /");

    let path_vec: Vec<PathPartType> = path
      .split("/")
      .map(|path| {
        if path.starts_with("{") && path.ends_with("}") {
          PathPartType::Variable(path[1..path.len() - 1].into())
        } else {
          PathPartType::Static(path.into())
        }
      })
      .collect();
    let mut base = &mut self.routes;

    for (index, path) in path_vec.iter().enumerate() {
      match path {
        PathPartType::Static(path) => {
          let result = base.entry(RoutePathType::Static(path.clone()));

          let test = result.or_insert(ResolvableRoute {
            path_variable_name: None,
            routes: HashMap::new(),
            children: HashMap::new(),
          });

          if index == path_vec.len() - 1 {
            test.routes.insert(method, route);
            break;
          } else {
            base = &mut test.children;
          }
        }
        PathPartType::Variable(path) => {
          let result = base.entry(RoutePathType::Variable);

          let test = result.or_insert(ResolvableRoute {
            path_variable_name: Some(path.clone()),
            routes: HashMap::new(),
            children: HashMap::new(),
          });
          if index == path_vec.len() - 1 {
            test.routes.insert(method, route);
            break;
          } else {
            base = &mut test.children;
          }
        }
      }
    }
  }
}
