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
use github::entities::{Error, NewIssue};
use github::response::{XRateLimitRemaining};

pub fn create(url: &str, token: &str, issue: &NewIssue) -> Result<Response, Error> {
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
            let err = Error::from_str(&body.content).unwrap();

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
