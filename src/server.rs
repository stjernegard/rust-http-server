use std::{io::{BufWriter, Error, Write}, net::{TcpListener, TcpStream}, thread};

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

    pub fn listen(&self) {
        let listener = match TcpListener::bind("127.0.0.1:4221") {
            Ok(listener) => listener,
            Err(error) => {
                println!("Failed binding to port: {}", error);
                return
            },
        };

        for stream in listener.incoming() {
            let router = self.router.clone();
            thread::spawn(move || {
                let mut stream = match stream {
                    Ok(stream) => stream,
                    Err(error) => {
                        println!("Error: {}", error);
                        return
                    },
                };

                let Some(request) = Request::new(&stream) else {
                    println!("Error: parsing request failed");
                    return
                };

                let response = router.handle(request);

                if let Err(error) = write_response(&mut stream, response) {
                    println!("Error: {}", error);
                    return
                }
            });
        }
    }
}

fn write_response(stream: &mut TcpStream, response: Response) -> Result<(), Error> {
    let mut writer = BufWriter::new(stream);
    writer.write_fmt(format_args!("{} {}\r\n", response.version, response.code))?;
    for (key, value) in response.headers {
        writer.write_fmt(format_args!("{}: {}\r\n", key, value))?;
    }
    if let Some(content) = response.content {
        writer.write_fmt(format_args!("\r\n{}", content))?;
    }
    writer.write(b"\r\n\r\n")?;

    writer.flush()
}
