mod request;
mod response;
mod router;
mod server;

use std::{env, fs};

use itertools::Itertools;
use server::Server;
use response::ResponseCode;

fn get_arg(arg: &'static str) -> Option<String> {
    let arglist = env::args().collect_vec();
    let mut args = arglist[1..].chunks(2);

    args
    .find(|&pair| pair.first() == Some(&arg.to_string()))
    .and_then(|pair| pair.last())
    .cloned()
}

fn main() {
    Server::new(|router| {
        router.register_handler(|req| {
            req.build_response(
                ResponseCode::OK,
                None,
                None,
                None
            )
        });

        router.register_path("user-agent", |router| {
            router.register_handler(|req| {
                req.build_response(
                    ResponseCode::OK,
                    None,
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
                    None,
                    Some(path)
                )
            });
        });

        router.register_path("files", |router| {
            router.register_catchall(|path, req| {
                let Some(dir) = get_arg("--directory") else {
                    return req.not_found()
                };

                let Ok(file) = fs::read_to_string(format!("{}/{}", &dir, path)) else {
                    return req.not_found()
                };

                req.build_response(
                    ResponseCode::OK,
                    Some("application/octet-stream".to_string()),
                    None,
                    Some(file)
                )
            });
        });
    }).listen();
}
