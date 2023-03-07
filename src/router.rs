use super::{HttpRequest, HttpResponse};
use std::collections::HashMap;

pub struct Router {
    routes: HashMap<String, Route>,
}

impl Router {
    pub fn new(routes: Vec<Route>) -> Router {
        let routes = Self::import_routes(routes);
        Router { routes }
    }

    fn import_routes(routes: Vec<Route>) -> HashMap<String, Route> {
        let mut map = HashMap::new();
        for route in routes {
            map.insert(route.path.to_string(), route);
        }
        map
    }

    pub fn add_route(
        &mut self,
        path: String,
        handler: Box<dyn Fn(HttpRequest) -> HttpResponse + Send + Sync>,
    ) {
        self.routes
            .insert(path.to_string(), Route::new(path, handler));
    }

    pub fn get_handler(&self, path: &str) -> Option<&Route> {
        self.routes.get(path)
    }
}

pub struct Route {
    pub path: String,
    pub handler: Box<dyn Fn(HttpRequest) -> HttpResponse + Send + Sync>,
}

impl Route {
    pub fn new(
        path: String,
        handler: Box<dyn Fn(HttpRequest) -> HttpResponse + Send + Sync>,
    ) -> Route {
        Route {
            path: path.to_string(),
            handler,
        }
    }

    pub fn handle(&self, request: HttpRequest) -> HttpResponse {
        (self.handler)(request)
    }
}
