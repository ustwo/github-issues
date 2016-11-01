use std::fmt;
use hyper;
use hyper::Client;
use hyper::client::Response;
use hyper::header::{Headers, Accept, Authorization, Connection, UserAgent, qitem};
use regex::Regex;
use std::io::prelude::*;
use std::process;

use say;
use github;

header! { (XRateLimitRemaining, "X-RateLimit-Remaining") => [u32] }

header! { (Link, "Link") => [String] }

/// The result of processing a response.
pub struct Page {
    pub content: String,
    pub next: Option<String>,
    pub ratelimit: u32,
}

impl Page {
    pub fn new(url: &str, token: &str) -> Page {
        let mut res = get_page(url.to_string(), token);
        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();

        Page {content: body,
              next: next_url(link(&res.headers)),
              ratelimit: ratelimit(&res.headers)}
    }

    pub fn warn(&self) {
        if self.next.is_none() {
            println!("{} {} {}", say::warn(), self.ratelimit, "Remaining requests");
        }
    }
}

impl fmt::Display for Page {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{next: {:?}, ratelimit: {}}}", self.next, self.ratelimit)
    }
}

fn get_page(url: String, token: &str) -> Response {
    println!("{} {} {}", say::info(), "Fetching", url);

    let client = Client::new();
    let res = client.get(&*url.clone())
                    .header(UserAgent(format!("nerve/{}", env!("CARGO_PKG_VERSION"))))
                    .header(Authorization(format!("token {}", token)))
                    .header(Accept(vec![qitem(github::mime())]))
                    .header(Connection::close())
                    .send().unwrap_or_else(|_| process::exit(1));

    match res.status {
        hyper::Ok => {}
        _ => {
            println!("{} {}", say::error(), "Unable to parse the response from Github");
            process::exit(1)
        }
    }

    res
}

pub fn ratelimit(headers: &Headers) -> u32 {
    match headers.get() {
        Some(&XRateLimitRemaining(x)) => x,
        None => 0
    }
}

// A Link header is not present if the requested collection has less than
// _pagesize_.
pub fn link(headers: &Headers) -> String {
    match headers.get() {
        Some(&Link(ref x)) => x.to_string(),
        None => "".to_string()
    }
}

fn next_url(link: String) -> Option<String> {
    let re = Regex::new(r"<([^;]+)>;\s*rel=.next.").unwrap();
    match re.captures(&link) {
        None => None,
        Some(cs) => cs.at(1).as_ref().map(|x| x.to_string())
    }
}
