use crate::http::Error;
use crate::http::Result;
use std::str::FromStr;

#[derive(std::fmt::Debug)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
}

impl FromStr for Method {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "GET" => Ok(Method::GET),
            "POST" => Ok(Method::POST),
            "PUT" => Ok(Method::PUT),
            "DELETE" => Ok(Method::DELETE),
            _ => Err(Error::InvalidMethod),
        }
    }
}
