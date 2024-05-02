use std::{io::{BufRead, BufReader, Error, Write}, net::{TcpListener, TcpStream}};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        println!("accepted new connection");

        if let Err(e) = stream.and_then(|mut s| handle(&mut s)) {
            println!("Error: {}", e);
            break;
        }

        println!("Request completed");
    }
}

fn handle(stream: &mut TcpStream) -> Result<(), Error> {
    let request = parse(stream)?;
    let response = respond(request);
    return write!(stream, "{} {}\r\n\r\n", response.version, response.code);
}

fn parse(stream: &TcpStream) -> Result<Request, Error> {
    let mut line = String::new();
    let mut reader = BufReader::new(stream);
    reader.read_line(&mut line)?;
    let mut parts = line.split_whitespace();
    return Ok(Request {
        method: parts.next().unwrap().to_string(),
        path: parts.next().unwrap().to_string(),
        version: parts.next().unwrap().to_string(),
    });
}

fn respond(request: Request) -> Response {
    let code = if request.method == "GET" && request.path == "/" {
        "200 OK"
    } else {
        "404 Not Found"
    };

    return Response {
        version: request.version,
        code: code.to_string(),
    }
}

struct Request {
    method: String,
    path: String,
    version: String,
}

struct Response {
    version: String,
    code: String,
}
