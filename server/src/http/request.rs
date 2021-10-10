use super::method::Method;
use std::convert::TryFrom;
use std::error::Error;

pub struct Request {
    path: String,
    query_string: Option<String>,
    // There is no none or null in Rust, therefore Option enum plays a role for the null
    // Option : Kind of Enum in Rust standard library. Use <> cause it is generic type.
    method: Method,
}

impl TryFrom<&[u8]> for Request {
    type Error = String;

    // GET /search?name=abc&sort=1 HTTP/1.1
    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        let string = String::from("asd");
        unimplemented!()
    }
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvaildProtocol,
    InvalidMethod,
}

impl Error for ParseError {}
