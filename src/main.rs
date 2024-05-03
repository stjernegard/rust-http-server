mod request;
mod response;
mod router;
mod server;

use router::Router;
use server::Server;
use response::ResponseCode;

use std::net::TcpListener;

fn main() {
    let listener = match TcpListener::bind("127.0.0.1:4221") {
        Ok(listener) => listener,
        Err(error) => {
            println!("Failed binding to port: {}", error);
            return
        },
    };

    let mut server = Server {
        listener,
        router: Router::new(),
    };

    server.router.register_handler(|req| {
        req.build_response(
            ResponseCode::OK,
            None,
            None
        )
    });

    server.router.register_path("user-agent", |router| {
        router.register_handler(|req| {
            req.build_response(
                ResponseCode::OK,
                None,
                req.headers.get("User-Agent").cloned()
            )
        });
    });

    server.router.register_path("echo", |router| {
        router.register_catchall(|path, req| {
            req.build_response(
                ResponseCode::OK,
                None,
                Some(path)
            )
        });
    });

    server.listen()
}
