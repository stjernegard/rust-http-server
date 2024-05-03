use std::collections::HashMap;

use itertools::Itertools;

use crate::{request::Request, response::{Response, ResponseCode}};

#[derive(Clone)]
pub struct Router {
    root: Option<fn(&Request) -> Response>,
    catchall: Option<fn(String, &Request) -> Response>,
    subrouters: HashMap<&'static str, Router>,
}

impl Router {
    pub fn new() -> Self {
        Router {
            root: None,
            catchall: None,
            subrouters: HashMap::new(),
        }
    }

    pub fn register_handler(&mut self, f: fn(&Request) -> Response) {
        self.root = Some(f);
    }

    pub fn register_catchall(&mut self, f: fn(String, &Request) -> Response) {
        self.catchall = Some(f);
    }

    pub fn register_path(&mut self, route: &'static str, f: fn(&mut Router) -> ()) {
        let mut router = Self::new();
        f(&mut router);
        self.subrouters.insert(route, router);
    }
}

impl Router {
    pub fn handle(&self, request: Request) -> Response {

        let path = request.path.split("/")
        .filter(|segment| !segment.is_empty())
        .collect_vec();

        match self._handle(path.as_slice(), &request) {
            None => return not_found(request),
            Some(response) => return response,
        }
    }

    fn _handle(&self, path: &[&str], request: &Request) -> Option<Response> {
        if let Some(catchall) = self.catchall {
            return Some(catchall(path.join("/"), request));
        }

        match path.split_first() {
            Some((head, tail)) => self.subrouters.get(*head)?._handle(tail, request),
            None => return Some(self.root?(request))
        }
    }
}

fn not_found(request: Request) -> Response {
    request.build_response(
        ResponseCode::NotFound,
        None,
        None
    )
}
