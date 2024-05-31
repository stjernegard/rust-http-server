use std::io::{Result, Write};

use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};

use crate::router::Router;
use crate::{request::Request, response::Response};

pub struct Server<'a> {
    pub router: Router<'a>
}

impl Server<'_> {
    pub fn new<'a>(f: fn(&mut Router) -> ()) -> Server<'a> {
        let mut router = Router::new();
        f(&mut router);
        Server { router }
    }

    pub async fn listen(&self) {
        let listener = match TcpListener::bind("127.0.0.1:4221").await {
            Ok(listener) => listener,
            Err(error) => {
                println!("Failed binding to port: {}", error);
                return
            },
        };

        loop {
            let (mut stream, _) = match listener.accept().await {
                Ok(result) => result,
                Err(error) => {
                    println!("Failed to accept tcp stream: {}", error);
                    return
                }
            };

            let Some(request) = Request::new(&mut stream).await else {
                println!("Error: parsing request failed");
                return
            };

            let response = self.router.handle(&request);

            if let Err(error) = write_response(&mut stream, response).await {
                println!("Error: {}", error);
                return
            }
        }
    }
}

async fn write_response(stream: &mut TcpStream, response: Response) -> Result<()> {
    let mut buf = Vec::<u8>::new();

    writeln!(buf, "{} {}", response.version, response.code)?;
    for (key, value) in &response.headers {
        writeln!(buf, "{}: {}", key, value)?;
    }
    if !response.headers.is_empty() {
        write!(buf, "\r\n")?;
    }
    if let Some(content) = response.content {
        writeln!(buf, "{}\r\n", content)?;
    }

    stream.write_all(&buf).await
}
