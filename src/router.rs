use std::collections::HashMap;

use itertools::Itertools;

use crate::{request::Request, response::Response};

pub struct Router<'a> {
    paths: HashMap<&'static str, Routable<'a>>
}

struct Route<'a> {
    handler: Box<dyn 'a + Fn(&str, &Request) -> Response>,
}

enum Routable<'a> {
    Router(Router<'a>),
    Route(Route<'a>),
}

impl Router<'_> {
    pub fn new<'a>() -> Router<'a> {
        Router { paths: HashMap::new() }
    }

    pub fn handle(&self, request: &Request) -> Response {
        let path_components = request.path.split("/")
        .filter(|&segment| !segment.is_empty())
        .collect_vec();

        self.handle_recursive(path_components, request)
    }

    fn handle_recursive(&self, path: Vec<&str>, request: &Request) -> Response {
        if path.is_empty() || path == ["/"] {
            if let Some(Routable::Route(route)) = self.paths.get("/") {
                return route.handle("/", request);
            }
        }

        if let Some(Routable::Route(route)) = self.paths.get("*") {
            return route.handle(&path.join("/"), request);
        }

        let Some((&head, tail)) = path.split_first() else {
            return request.not_found();
        };

        let Some(&ref routable) = self.paths.get(head) else {
            return request.not_found()
        };

        match routable {
            Routable::Route(route) => route.handle(head, request),
            Routable::Router(router) => router.handle_recursive(tail.to_vec(), request)
        }
    }
}

impl Route<'_> {
    fn handle(&self, path: &str, request: &Request) -> Response {
        (self.handler)(path, &request)
    }
}

impl<'a> Router<'a> {
    pub fn register_handler(&mut self, path: &'static str, f: impl Fn(&str, &Request) -> Response + 'a) {
        self.paths.insert(path, Routable::Route(Route { handler: Box::new(f) }));
    }

    pub fn register_group(&mut self, path: &'static str, f: impl Fn(&mut Router) -> ()) {
        let mut router = Self::new();
        f(&mut router);
        self.paths.insert(path, Routable::Router(router));
    }
}
