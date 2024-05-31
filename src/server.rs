use std::io::Error;
use std::io::Write;

use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};

use crate::{request::Request, response::Response, router::Router};

pub struct Server {
    pub router: Router,
}

impl Server {
    pub fn new(f: fn(&mut Router) -> ()) -> Server {
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

            let response = self.router.handle(request);

            if let Err(error) = write_response(&mut stream, response).await {
                println!("Error: {}", error);
                return
            }
        }
    }
}

async fn write_response(stream: &mut TcpStream, response: Response) -> Result<(), Error> {
    let mut buf = Vec::<u8>::new();

    writeln!(buf, "{} {}", response.version, response.code)?;
    for (key, value) in response.headers {
        writeln!(buf, "{}: {}", key, value)?;
    }
    write!(buf, "\r\n")?;
    if let Some(content) = response.content {
        write!(buf, "{}", content)?;
    }
    write!(buf, "\r\n\r\n")?;

    stream.write_all(&buf).await
}
