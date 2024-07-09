use std::{collections::HashMap, fmt::Debug};

use route_http::{method::HttpMethod, variable::VariableValue};
pub mod error;
pub mod extractors;

#[derive(Debug, Clone)]
struct ResolvableRoute<H> {
  /// This depends on the hashmap key.
  path_variable_name: Option<String>,
  routes: HashMap<HttpMethod, Route<H>>,
  children: HashMap<RoutePathType, ResolvableRoute<H>>,
}

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
enum RoutePathType {
  Static(String),
  Variable,
}

#[derive(Clone, Debug)]
pub struct Route<H> {
  pub handler: H,
}

impl<H> Route<H> {
  pub fn new(handler: H) -> Self {
    Route { handler }
  }
}

#[derive(Debug, Clone)]
enum RouterMount {
  Root,
  Path(String),
}

impl RouterMount {
  fn as_str<'a>(&'a self) -> &'a str {
    match self {
      RouterMount::Root => "/",
      RouterMount::Path(path) => path.as_str(),
    }
  }
}

#[derive(Debug)]
pub struct Router<H>
// where
//   H: Clone,
{
  mount: RouterMount,
  routes: HashMap<RoutePathType, ResolvableRoute<H>>,
}

#[derive(Debug)]
enum PathPartType {
  Variable(String),
  Static(String),
}

#[derive(Debug)]
pub struct NoRouteFound;
#[derive(Debug)]
pub struct RouteFound<'a, H> {
  pub route: &'a Route<H>,
  pub variables: HashMap<String, VariableValue>,
}
impl<H> Router<H>
// where
//   H: Clone,
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
  ) -> Result<RouteFound<'_, H>, NoRouteFound> {
    let mount_str = self.mount.as_str();
    let path_without_mount: &str = route.strip_prefix(mount_str).ok_or(NoRouteFound)?;
    let path_vec: Vec<&str> = path_without_mount.split('/').collect();

    let mut base = &self.routes;
    let mut current_route: Option<&ResolvableRoute<H>> = None;
    let mut variables = HashMap::new();

    for item in path_vec {
      if let Some(static_output) = base.get(&RoutePathType::Static(item.to_string())) {
        current_route = Some(static_output);
        base = &static_output.children;
      } else if let Some(dynamic_output) = base.get(&RoutePathType::Variable) {
        let variable_name = dynamic_output
          .path_variable_name
          .as_ref()
          .expect("route-rs: Internal error: Dynamic route without path variable name on route");
        let variable_value = VariableValue::new(item.into());
        variables.insert(variable_name.clone(), variable_value);
        current_route = Some(dynamic_output);
        base = &dynamic_output.children;
      } else {
        return Err(NoRouteFound);
      }
    }

    match current_route {
      Some(route) => {
        if let Some(route_for_method) = route.routes.get(&method) {
          Ok(RouteFound { variables, route: route_for_method })
        } else {
          Err(NoRouteFound)
        }
      }
      None => Err(NoRouteFound),
    }
  }
  // pub fn match_route(
  //   &self,
  //   method: HttpMethod,
  //   route: &str,
  // ) -> Result<RouteFound<H>, NoRouteFound> {
  //   let mount_str = self.mount.as_str();
  //   let path_without_mount: &str = route.strip_prefix(mount_str).ok_or(NoRouteFound)?;
  //   let path_vec: Vec<&str> = path_without_mount.split('/').collect();

  //   let mut base = &self.routes;
  //   let mut current_route: Option<&ResolvableRoute<H>> = None;
  //   let mut variables = HashMap::new();

  //   for item in path_vec {
  //     if let Some(static_output) = base.get(&RoutePathType::Static(item.to_string())) {
  //       current_route = Some(static_output);
  //       base = &current_route.unwrap().children;
  //     } else if let Some(dynamic_output) = base.get(&RoutePathType::Variable) {
  //       let variable_name = dynamic_output
  //         .path_variable_name
  //         .as_ref()
  //         .expect("route-rs: Internal error: Dynamic route without path variable name on route");
  //       let variable_value = VariableValue::new(item.into());
  //       variables.insert(variable_name.clone(), variable_value);
  //       current_route = Some(dynamic_output);
  //       base = &current_route.unwrap().children;
  //     } else {
  //       return Err(NoRouteFound);
  //     }
  //   }

  //   match current_route {
  //     Some(route) => {
  //       if let Some(route_for_method) = route.routes.get(&method) {
  //         Ok(RouteFound { variables, route: route_for_method.clone() })
  //       } else {
  //         Err(NoRouteFound)
  //       }
  //     }
  //     None => Err(NoRouteFound),
  //   }
  // }
  // pub fn match_route(
  //   &self,
  //   method: HttpMethod,
  //   route: &str,
  // ) -> Result<RouteFound<H>, NoRouteFound> {
  //   let mount_str = self.mount.as_str();
  //   let path_without_mount: &str = route.strip_prefix(mount_str).ok_or(NoRouteFound)?;
  //   let path_vec: Vec<&str> = path_without_mount.split('/').collect();

  //   let mut base = self.routes.clone();
  //   let mut current_route: Option<ResolvableRoute<H>> = None;
  //   let mut variables = HashMap::new();

  //   for item in path_vec {
  //     let static_output = base.get(&RoutePathType::Static(item.to_string()));

  //     if static_output.is_some() {
  //       current_route = Some(static_output.unwrap().clone());
  //       base = current_route.clone().unwrap().children;
  //     } else {
  //       let dynamic_output = base.get(&RoutePathType::Variable);
  //       if dynamic_output.is_none() {
  //         return Err(NoRouteFound);
  //       }
  //       let dynamic_output = dynamic_output.unwrap();
  //       let variable_name = dynamic_output
  //         .path_variable_name
  //         .clone()
  //         .expect("route-rs: Internal error: Dynamic route without path variable name on route");
  //       let variable_value = VariableValue::new(item.into());
  //       variables.insert(variable_name, variable_value);
  //       current_route = Some(dynamic_output.clone());
  //       base = current_route.clone().unwrap().children;
  //     }
  //   }

  //   match current_route {
  //     Some(route) => {
  //       let route_exists_for_specified_method = route.routes.get(&method);

  //       match route_exists_for_specified_method {
  //         None => Err(NoRouteFound),
  //         Some(route) => Ok(RouteFound { variables, route: route.clone() }),
  //       }
  //     }
  //     None => Err(NoRouteFound),
  //   }
  // }

  pub fn route(&mut self, method: HttpMethod, path: String, route: Route<H>) {
    let path = path.strip_prefix('/').expect("route-rs: route path must start with /");

    let path_vec: Vec<PathPartType> = path
      .split('/')
      .map(|path| {
        if path.starts_with('{') && path.ends_with('}') {
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