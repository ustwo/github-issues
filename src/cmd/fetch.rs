use std::io::prelude::*;
use csv;
use regex::Regex;
use rustc_serialize::json;
use std::fs::File;
use std::process;
use std::result::Result;

use hyper;
use hyper::Client;
use hyper::header::{Accept, Authorization, Connection, UserAgent, qitem};
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};

use say;
use format::{OutputFormat};
use github::entities::{Issues};

header! { (XRateLimitRemaining, "X-RateLimit-Remaining") => [u32] }

header! { (Link, "Link") => [String] }

fn ratelimit(headers: &hyper::header::Headers) -> u32 {
    match headers.get() {
        Some(&XRateLimitRemaining(x)) => x,
        None => 0
    }
}

fn get_page(url: String, token: String) -> hyper::client::Response {
    println!("{} {} {}", say::info(), "Fetching", url);

    let client = Client::new();
    let res = client.get(&*url.clone())
                    .header(UserAgent(format!("nerve/{}", crate_version!())))
                    .header(Authorization(format!("token {}", token)))
                    .header(Accept(vec![qitem(Mime(TopLevel::Application,
                                                   SubLevel::Ext("vnd.github.v3+json".to_owned()),
                                                   vec![(Attr::Charset, Value::Utf8)]))]))
                    .header(Connection::close())
                    .send().unwrap_or_else(|_| process::exit(1));

    match res.status {
        hyper::Ok => {
        
        }
        _ => {
            println!("{} {}", say::error(), "Unable to parse the response from Github");
            process::exit(1)
        }
    }

    // Read the Response.
    // let mut body = String::new();
    // res.read_to_string(&mut body).unwrap();
    // println!("Response: {}", body);

    res
}

fn link(headers: &hyper::header::Headers) -> String {
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

// TODO: Review its need
// fn parse_repopath(path: String) -> (String, String) {
//     let list: Vec<&str> = path.split("/").collect();

//     (list[0].to_string(), list[1].to_string())
// }

fn as_issues(res: &mut hyper::client::Response) -> Result<Issues, json::DecoderError> {
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    json::decode(&body)
}

pub fn run(repopath: String,
           oauth_token: String,
           labels: Vec<String>,
           state: String,
           format: OutputFormat,
           output_file: String) {

    // let (owner, repo) = parse_repopath(repopath);
    let labels_pair = if labels.is_empty() { "".to_owned() }
                      else { format!("&labels={}", labels.join(",")) };
    let url = format!("https://api.github.com/repos/{}/issues?filter=all&state={}{}",
                      repopath, state, labels_pair);

    let mut res = get_page(url, oauth_token.to_string());
    let mut issues = as_issues(&mut res).unwrap();


    // A Link header is not present if the requested collection has less than
    // _pagesize_.
    let mut nurl = next_url(link(&res.headers));

    if nurl.is_none() {
        println!("{} {} {}", say::warn(), ratelimit(&res.headers), "Remaining requests");
    }

    while let Some(nu) = nurl {
        let mut r = get_page(nu.to_string(), oauth_token.to_string());
        issues.extend(as_issues(&mut r).unwrap());

        nurl = next_url(link(&r.headers));
        if nurl.is_none() {
            println!("{} {} {}", say::warn(), ratelimit(&r.headers), "Remaining requests");
        }
    }

    println!("{} {}", say::highlight("Total issues collected:"), issues.len());


    match format {
        OutputFormat::CSV => write_csv(issues, output_file),
        OutputFormat::JSON => write_json(issues, output_file),
    }
}


fn write_json(issues: Issues, output_file: String) {
    let mut f = File::create(output_file).unwrap();
    let string: String = json::encode(&issues).unwrap();
    let _ = f.write_all(string.as_bytes());
}

fn write_csv(issues: Issues, output_file: String) {
    let mut wtr = csv::Writer::from_file(output_file).unwrap();

    let headers = ("number",
                   "title",
                   "state",
                   "created_at",
                   "closed_at",
                   "assignee",
                   "user",
                   "labels",
                   "body");
    let _ = wtr.encode(headers);

    for issue in issues {
        let labels = issue.labels.iter()
                                 .map(|x| x.name.clone())
                                 .collect::<Vec<String>>()
                                 .join(",");
        let user = issue.user;
        let assignee = issue.assignee;

        let row = (issue.number,
                   issue.title,
                   issue.state,
                   issue.created_at,
                   issue.closed_at,
                   assignee,
                   user,
                   labels,
                   issue.body);

        let _ = wtr.encode(row);
    }
}
