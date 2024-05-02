use std::{collections::HashMap, io::{BufRead, BufReader, BufWriter, Error, Write}, net::{TcpListener, TcpStream}};

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
    let response = respond(&request);

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
    let mut headers: HashMap<String, String> = HashMap::new();

    let mut lines = reader.lines();
    while let Some(Ok(line)) = lines.next() {
        if let Some((key, value)) = line.split_once(": ") {
            headers.insert(key.to_string(), value.to_string());
        } else {
            break;
        }
    }

    Ok(Request {
        method: parts.next().unwrap().to_string(),
        path: parts.next().unwrap().to_string(),
        version: parts.next().unwrap().to_string(),
        headers: HashMap::from_iter(headers)
    })
}

fn respond(request: &Request) -> Response {
    let mut code = "404 Not Found";
    let mut headers: HashMap<String, String> = HashMap::new();
    let mut content: Option<String> = None;

    if request.method == "GET" {
        if request.path == "/" {
            code = "200 OK";
        } else if request.path == "/user-agent" {
            code = "200 OK";
            content = request.headers.get("User-Agent").cloned();

        } else if request.path.starts_with("/echo/") {
            code = "200 OK";
            let message = request.path.trim_start_matches("/echo/");
            content = Some(message.to_string());
        }

        if let Some(ref content) = content {
            headers.insert("Content-Type".to_string(), "text/plain".to_string());
            headers.insert("Content-Length".to_string(), content.len().to_string());
        }
    }

    Response {
        version: request.version.to_owned(),
        code: code.to_string(),
        headers,
        content,
    }
}

struct Request {
    method: String,
    path: String,
    version: String,
    headers: HashMap<String, String>,
}

struct Response {
    version: String,
    code: String,
    headers: HashMap<String, String>,
    content: Option<String>,
}
