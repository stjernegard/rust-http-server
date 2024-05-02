use std::{io::{BufRead, BufReader, BufWriter, Error, Write}, net::{TcpListener, TcpStream}};

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

fn parse(stream: &TcpStream) -> Result<Request, Error> {
    let mut line = String::new();
    let mut reader = BufReader::new(stream);
    reader.read_line(&mut line)?;
    let mut parts = line.split_whitespace();

    Ok(Request {
        method: parts.next().unwrap().to_string(),
        path: parts.next().unwrap().to_string(),
        version: parts.next().unwrap().to_string(),
    })
}

fn respond(request: Request) -> Response {
    let mut code = "404 Not Found";
    let mut headers: Vec<(String, String)> = Vec::new();
    let mut content: Option<String> = None;

    if request.method == "GET" {
        if request.path == "/" {
            code = "200 OK"
        } else if request.path.starts_with("/echo/") {
            code = "200 OK";
            let message = request.path.trim_start_matches("/echo/");
            content = Some(message.to_string());
            headers.push(("Content-Type".to_string(), "text/plain".to_string()));
            headers.push(("Content-Length".to_string(), message.len().to_string()));
        }
    }

    Response {
        version: request.version,
        code: code.to_string(),
        headers,
        content,
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
    headers: Vec<(String, String)>,
    content: Option<String>,
}
