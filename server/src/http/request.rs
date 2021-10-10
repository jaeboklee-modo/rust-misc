use super::method::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::str;
use std::str::Utf8Error;
// Importing multiple values from same module
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

pub struct Request {
    path: String,
    query_string: Option<String>,
    // There is no none or null in Rust, therefore Option enum plays a role for the null
    // Option : Kind of Enum in Rust standard library. Use <> cause it is generic type.
    method: Method,
}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    // GET /search?name=abc&sort=1 HTTP/1.1
    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        // match str::from_utf8(buf) {
        //     Ok(request) => {}
        //     Err(_) => return Err(ParseError::InvalidEncoding),
        // }

        // or method : return self if it is Ok, but return error if result is err.
        // So it can replace above expression. Common pattern in rust
        // match str::from_utf8(buf).or(Err(ParseError::InvalidEncoding)) {
        //     Ok(request) => {}
        //     Err(e) => return Err(e),
        // }

        // ? mark : result is Ok, return the value that ok wraps, if err, return err.
        // let request = str::from_utf8(buf).or(Err(ParseError::InvalidEncoding))?;

        let request = str::from_utf8(buf)?;

        // match get_next_word(request) {
        //     // The first word would be GET, POST ...
        //     Some((method, request)) => {}
        //     // We have Invalid req in this case.
        //     None => return Err(ParseError::InvalidRequest),
        // }

        // Transforms an options into result which is same with above code
        // If some, return Ok, otherwise none, recept Err param and return err.
        // 'request' is assigned agained, which means shadowing above 'request'.
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvaildProtocol);
        }
        unimplemented!()
    }
}
// Option is for when there is no string at the end.
fn get_next_word(request: &str) -> Option<(&str, &str)> {
    // let mut iter = request.chars();
    // loop {
    //     let item = iter.next();
    //     match item {
    //         Some(c) => {}
    //         None => break,
    //     }
    // }

    // same result above
    // enumberate : yields tuple.
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            // In rust, index is not character, byte.
            return Some((&request[..i], &request[i + 1..]));
        }
    }

    None
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvaildProtocol,
    InvalidMethod,
}

impl ParseError {
    // here, self is enum
    fn message(&self) -> &str {
        // Automatically return string message corresponding to enum.
        match self {
            Self::InvalidRequest => "InvalidRequest",
            Self::InvalidEncoding => "InvalidEncoding",
            Self::InvaildProtocol => "InvaildProtocol",
            Self::InvalidMethod => "InvalidMethod",
        }
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {}
