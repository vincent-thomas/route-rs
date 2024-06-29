use std::collections::HashMap;

#[derive(Debug)]
pub struct RouteTree {
    route: RouteInTree,
    children: Box<HashMap<String, RouteTree>>,
}

#[derive(Debug)]
pub enum RouteInTree {
    NoRoute,
    RouteExists(Route),
}

#[derive(Debug)]
pub struct Routes {
    routes: HashMap<String, RouteTree>,
    prefix: Option<String>,
}

fn main() {
    let mut routes_in_auth: HashMap<String, RouteTree> = HashMap::new();

    routes_in_auth.insert(
        "register".into(),
        RouteTree {
            children: Box::new(HashMap::new()),
            route: RouteInTree::RouteExists(Route {
                method: RouteMethod::Post,
                handler: Box::new(|| {}),
            }),
        },
    );
    routes_in_auth.insert(
        "login".into(),
        RouteTree {
            children: Box::new(HashMap::new()),
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
            route: RouteInTree::NoRoute,
            children: Box::new(routes_in_auth),
        },
    );

    let tree = Routes {
        prefix: None,
        routes,
    };

    println!("{tree:#?}");
}

#[derive(Debug)]
pub struct Route {
    method: RouteMethod,
    handler: Box<fn()>,
}

#[derive(Debug)]
enum RouteMethod {
    Get,
    Post,
    Delete,
    Patch,
    Put,
}
