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
}


pub fn create(url: &str, token: &str, issue: NewIssue) -> Response {
    let client = Client::new();
    let body = json::encode(&issue).unwrap();

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
        e => {
            println!("{} {}", say::error(), e);
            process::exit(1)
        }
    }

    as_response(res)
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
