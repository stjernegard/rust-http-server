mod request;
mod response;
mod router;
mod server;

use std::{env, fs};

use itertools::Itertools;
use server::Server;
use response::ResponseCode;

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
                let arglist = env::args().collect_vec();
                let mut args = arglist[1..].chunks(2);

                let Some(dir) = args
                .find(|&pair| pair.first() == Some(&"--directory".to_string()))
                .and_then(|pair| pair.last()) else {
                    return req.build_response(
                        ResponseCode::NotFound,
                        None,
                        None,
                        None
                    )
                };

                match fs::read_to_string(format!("{}/{}", &dir, path)) {
                    Ok(file) => {
                        req.build_response(
                            ResponseCode::OK,
                            Some("application/octet-stream".to_string()),
                            None,
                            Some(file)
                        )
                    }

                    Err(_) => {
                        req.build_response(
                            ResponseCode::NotFound,
                            None,
                            None,
                            None
                        )
                    }
                }
            });
        });
    }).listen();
}
