use std::fmt;
use hyper::Client;
use hyper::status::StatusCode;
use hyper::client::Response as HyperResponse;
use hyper::header::{Headers, Accept, Authorization, Connection, UserAgent, qitem};
use hyper;
use rustc_serialize::json;
use std::io::Read;
use std::process;

use say;
use github::mime;

header! { (XRateLimitRemaining, "X-RateLimit-Remaining") => [u32] }

pub type Issues = Vec<NewIssue>;

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct NewIssue {
    pub assignees: Vec<String>,
    pub body: String,
    pub labels: Vec<String>,
    pub title: String,
    pub milestone: Option<u32>,
}

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct RemoteError {
    pub message: String,
    pub errors: Vec<ErrorResource>,
}

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub enum ErrorName {
    Invalid,
    Missing,
    MissingField,
    AlreadyExists,
}

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct ErrorResource {
    pub code: String,
    pub resource: String,
    pub field: String,
}

impl fmt::Display for RemoteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let error = self.errors.first().unwrap();

        write!(f, "the field '{}' {}", error.field, "has an invalid value.")
    }
}


fn as_remote_error(data: &str) -> Result<RemoteError, json::DecoderError> {
    json::decode(data)
}


pub fn create(url: &str, token: &str, issue: &NewIssue) -> Result<Response, RemoteError> {
    let client = Client::new();
    let body = json::encode(issue).unwrap();

    let res = client.post(&*url.clone())
                    .body(&body)
                    .header(UserAgent(format!("nerve/{}", env!("CARGO_PKG_VERSION"))))
                    .header(Authorization(format!("token {}", token)))
                    .header(Accept(vec![qitem(mime())]))
                    .header(Connection::close())
                    .send().unwrap_or_else(|_| process::exit(1));

    match res.status {
        hyper::Ok => {}
        StatusCode::Created => {}
        StatusCode::UnprocessableEntity => {
            let body = as_response(res);
            let err = as_remote_error(&body.content).unwrap();

            return Err(err);
        }
        e => {
            println!("{} {}", say::error(), e);
            process::exit(1)
        }
    }

    Ok(as_response(res))
}


/// The result of processing a response.
#[derive(Debug)]
pub struct Response {
    pub content: String,
    pub ratelimit: u32,
}


pub fn as_response(mut res: HyperResponse) -> Response {
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    Response { content: body
             , ratelimit: ratelimit(&res.headers)
             }
}



pub fn warn_ratelimit(ratelimit: u32) {
    println!("{} {} {}", say::warn(), ratelimit, "Remaining requests");
}

pub fn ratelimit(headers: &Headers) -> u32 {
    match headers.get() {
        Some(&XRateLimitRemaining(x)) => x,
        None => 0
    }
}
