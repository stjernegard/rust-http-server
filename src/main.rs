mod request;
mod response;
mod router;
mod server;

use server::Server;
use response::ResponseCode;

fn main() {
    Server::new(|router| {
        router.register_handler(|req| {
            req.build_response(
                ResponseCode::OK,
                None,
                None
            )
        });

        router.register_path("user-agent", |router| {
            router.register_handler(|req| {
                req.build_response(
                    ResponseCode::OK,
                    None,
                    req.headers.get("User-Agent").cloned()
                )
            });
        });

        router.register_path("echo", |router| {
            router.register_catchall(|path, req| {
                req.build_response(
                    ResponseCode::OK,
                    None,
                    Some(path)
                )
            });
        });

    }).listen();
}
