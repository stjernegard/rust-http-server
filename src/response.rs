use std::fmt;
use std::collections::HashMap;

pub struct Response {
    pub version: String,
    pub code: ResponseCode,
    pub headers: HashMap<String, String>,
    pub content: Option<String>,
}

pub enum ResponseCode {
    OK,
    NotFound,
}

impl ResponseCode {
    fn as_u16(&self) -> u16 {
        match self {
            Self::OK => 200,
            Self::NotFound => 404,
        }
    }

    fn reason(&self) -> &str {
        match self {
            Self::OK => "OK",
            Self::NotFound => "Not Found",
        }
    }
}

impl fmt::Display for ResponseCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.as_u16(), self.reason())
    }
}
