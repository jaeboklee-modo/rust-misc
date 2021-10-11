use super::method::{Method, MethodError};
use std::convert::TryFrom;
use std::error::Error;
use std::str;
use std::str::Utf8Error;
// Importing multiple values from same module
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

// request should have longer lifetime than Buffer.
// lifetime is explicit feat in Rust
// Generally, we dont have to consider lifetime in other language.
// The lifetime for request is same for the buffer, so we can name its lifetime 'buf'
pub struct Request<'buf> {
    path: &'buf str, //a lifetime
    query_string: Option<&'buf str>,
    // There is no none or null in Rust, therefore Option enum plays a role for the null
    // Option : Kind of Enum in Rust standard library. Use <> cause it is generic type.
    method: Method,
}

// declare that we will have lifetime name buf.
impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    // GET /search?name=abc&sort=1 HTTP/1.1
    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
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
        //     // We have Invalid re q in this case.
        //     None => return Err(ParseError::InvalidRequest),
        // }

        // Transforms an options into result which is same with above code
        // If some, return Ok, otherwise none, recept Err param and return err.
        // 'request' is assigned agained, which means shadowing above 'request'.
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvaildProtocol);
        }

        let method: Method = method.parse()?;

        let mut query_string = None;
        // match path.find("?") {
        //     // i + 1 for excluding ? mark.
        //     // Some => string parse.
        //     Some(i) => {
        //         query_string = Some(&path[i + 1..]);
        //         path = &path[..i];
        //     }
        //     None => {} //In this case, None is not necessary
        // }

        // let q = path.find('?'); //In this case, q is not necessary
        // if q.is_some() {
        //     let i = q.unwrap();

        //     query_string = Some(&path[i + 1..]);
        //     path = &path[..i];
        // }

        if let Some(i) = path.find('?') {
            query_string = Some(path[i + 1..]);
            path = &path[..i];
        }

        Ok(Self {
            path,
            query_string,
            method,
        })
    }
}
// Option is for when there is no string at the end.
// In this case, there is no need for explicit lifetime since
// Option tuple's lifetime would be same as request's
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

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
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
