use std::{collections::HashMap, io::{BufRead, BufReader}, net::TcpStream};

use crate::response::{Response, ResponseCode};

pub struct Request {
    version: String,
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
}

impl Request {
    pub fn new(stream: &TcpStream) -> Option<Request> {
        let mut line = String::new();
        let mut reader = BufReader::new(stream);
        reader.read_line(&mut line).ok()?;
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

        Some(Request {
            method: parts.next()?.to_string(),
            path: parts.next()?.to_string(),
            version: parts.next()?.to_string(),
            headers: HashMap::from_iter(headers)
        })
    }

    pub fn build_response(&self, code: ResponseCode, headers: Option<HashMap<String, String>>, content: Option<String>) -> Response {
        let mut headers = headers.unwrap_or_else(HashMap::new);
        if let Some(ref content) = content {
            headers.insert("Content-Type".to_string(), "text/plain".to_string());
            headers.insert("Content-Length".to_string(), content.len().to_string());
        }
        Response {
            version: self.version.to_string(),
            code,
            headers,
            content
        }
    }
}