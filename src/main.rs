use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct RouteTree {
    route: RouteInTree,
    children: HashMap<String, RouteTree>,
    dynamic_children: Option<Box<(DynamicRouteType, RouteTree)>>,
    
}

#[derive(Debug, Clone)]
pub enum RouteInTree {
    NoRoute,
    RouteExists(Route),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DynamicRouteType {
    Single,
    Many,
}

#[derive(Debug)]
pub struct Routes {
    routes: RouteTree,
    prefix: RoutePrefix,
}

#[derive(Debug)]
pub enum RoutePrefix {
    /// Root == "/"
    Root,
    /// This would be a path where the router gets "mounted" on to, for example /api
    Mount(String)
}

fn main() {
    let mut routes_in_auth: HashMap<String, RouteTree> = HashMap::new();

    routes_in_auth.insert(
        "register".into(),
        RouteTree {
            dynamic_children: None,
            children: HashMap::new(),
            route: RouteInTree::RouteExists(Route {
                method: RouteMethod::Post,
                handler: Box::new(|| {}),
            }),
        },
    );
    routes_in_auth.insert(
        "login".into(),
        RouteTree {
            children: HashMap::new(),
            dynamic_children: None,
            route: RouteInTree::RouteExists(Route {
                method: RouteMethod::Patch,
                handler: Box::new(|| {}),
            }),
        },
    );
    let mut routes: HashMap<String, RouteTree> = HashMap::new();

    routes.insert(
        "auth".into(),
        RouteTree {
            dynamic_children: None,
            route: RouteInTree::NoRoute,
            children: routes_in_auth,
        },
    );


    routes.insert(
        "testing".into(),
        RouteTree {
            dynamic_children: Some(Box::new((
                DynamicRouteType::Single,
                RouteTree {
                    children: HashMap::new(),
                    dynamic_children: None,
                    route: RouteInTree::RouteExists(
                        Route {
                            method: RouteMethod::Post,
                            handler: Box::new(|| {})
                        }
                    )
                }
            ))),
            route: RouteInTree::RouteExists(Route {
                method: RouteMethod::Get,
                handler: Box::new(|| {}),
            }),
            children: HashMap::new(),
        },
    );

    let tree = Routes {
        prefix: RoutePrefix::Root,
        routes: RouteTree {
            route: RouteInTree::NoRoute,
            children: routes,
            dynamic_children: None
        },
    };

    println!("{tree:#?}");

    let query: Vec<&str> = "/auth/login".trim_matches('/').split('/').collect();

    let mut index = 0;
    let mut current_route = None;

    let query_length = query.len();

    let mut routes_to_check = tree.routes;

    loop {
        if index > query_length - 1 {
            break;
        }

        if let Some(next_routes) = routes_to_check.children.get(query[index]).cloned() {
            current_route = Some(next_routes.clone());
            routes_to_check = next_routes.children.get(query[index]);
            index += 1;
        } else {
            break;
        }
    }

    println!("{:?}", &current_route);
}

#[derive(Debug, Clone)]
pub struct Route {
    method: RouteMethod,
    handler: Box<fn() -> ()>,
}

#[derive(Debug, Clone)]
enum RouteMethod {
    Get,
    Post,
    Delete,
    Patch,
    Put,
}
