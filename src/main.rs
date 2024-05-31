mod request;
mod response;
mod router;
mod server;

use std::{env, fs};

use itertools::Itertools;
use server::Server;
use response::ResponseCode;

fn get_arg(arg: &str) -> Option<String> {
    let arglist = env::args().collect_vec();
    let mut args = arglist[1..].chunks(2);

    args
    .find(|&pair| pair.first() == Some(&arg.to_string()))
    .and_then(|pair| pair.last())
    .cloned()
}

#[tokio::main]
async fn main() {
    Server::new(|router| {
        router.register_handler("/", |_, req| {
            req.build_response(
                ResponseCode::OK,
                None,
                None,
                None
            )
        });

        router.register_handler("user-agent", |_, req| {
            req.build_response(
                ResponseCode::OK,
                None,
                None,
                req.headers.get("User-Agent").cloned()
            )
        });

        router.register_group("echo", |group| {
            group.register_handler("*", |path, req| {
                req.build_response(
                    ResponseCode::OK,
                    None,
                    None,
                    Some(path.to_string())
                )
            });
        });

        if let Some(dir) = get_arg("--directory") {
            router.register_group("files", |group| {
                let dir = dir.clone();
                group.register_handler("*", move |path, req| {
                    let Ok(file) = fs::read_to_string(format!("{}/{}", dir, path)) else {
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
        }
    }).listen().await;
}
