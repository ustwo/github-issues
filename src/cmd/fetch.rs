use csv;
use regex::Regex;
use std::result::Result;
use std::collections::HashMap;
use std::str;
use std::process;
use curl::http;
use rustc_serialize::json;

use say;

macro_rules! check_repopath {
    ($path:expr) => (
        if $path.len() != 2 {
            println!("{} {}", say::error(), "<repopath> must have the form <owner>/<repo>.  e.g. ustwo/github-issues");
            process::exit(1)
        }
    );
}

fn ratelimit(headers: &HashMap<String, Vec<String>>) -> String {
    headers.get("x-ratelimit-remaining").unwrap()
           .first().unwrap().to_string()
}

fn get_page(url: String, token: &str) -> http::Response {
    println!("{} {} {}", say::info(), "Fetching", url);

    let auth_header = format!("token {}", token);
    let res = http::handle()
                   .get(url)
                   .header("Authorization", &auth_header)
                   .header("User-Agent", "Github-Issues-CLI")
                   .header("Accept", "application/vnd.github.v3+json")
                   .exec()
                   .unwrap_or_else(|e| process::exit(1));

    if res.get_code() != 200 {
        match str::from_utf8(res.get_body()) {
            Ok(b) => {
                println!("{} {:?}", say::error(), json::decode::<GithubError>(b).ok());
                process::exit(1)
            }
            Err(..) => {
                println!("{} {}", say::error(), "Unable to parse the response from Github");
                process::exit(1)
            }
        }
    }

    res
}

fn next_url(link: String) -> Option<String> {
    let re = Regex::new(r"<([^;]+)>;\s*rel=.next.").unwrap();
    match re.captures(&link) {
        None => None,
        Some(cs) => cs.at(1).as_ref().map(|x| x.to_string())
    }
}

fn to_issues(raw: &[u8]) -> Result<Issues, json::DecoderError> {
    match str::from_utf8(raw) {
        Ok(b) => json::decode(b),
        Err(..) => {
            println!("{} {}", say::error(), "Unable to parse the response from Github");
            process::exit(1)
        }
    }
}

fn parse_repopath(path: String) -> (String, String) {
    let list: Vec<&str> = path.split("/").collect();

    check_repopath!(list);

    (list[0].to_string(), list[1].to_string())
}

pub fn run(repopath: String,
           oauth_token: String,
           labels: Vec<String>,
           state: String,
           output_file: String) {

    let (owner, repo) = parse_repopath(repopath);
    let labels_pair = if labels.is_empty() { "".to_string() }
                      else { format!("&labels={}", labels.join(",")) };
    let url = format!("https://api.github.com/repos/{}/{}/issues?filter=all&state={}{}",
                      owner, repo, state, labels_pair);

    let res = get_page(url, &oauth_token);
    let mut issues = to_issues(res.get_body()).unwrap();

    // A Link header is not present if the requested collection has less than
    // _pagesize_.
    match res.get_headers().get("link") {
        Some(links) => {
            let mut nurl = next_url(links.first().unwrap().clone());
            if nurl.is_none() {
                println!("{} {} {}", say::warn(), ratelimit(res.get_headers()), "Remaining requests");
            }

            while let Some(nu) = nurl {
                let r = get_page(nu.to_string(), &oauth_token);
                issues.extend(to_issues(r.get_body()).unwrap());

                let link = r.get_headers().get("link").unwrap()
                                          .first().unwrap()
                                          .clone();
                nurl = next_url(link);
                if nurl.is_none() {
                    println!("{} {} {}", say::warn(), ratelimit(r.get_headers()), "Remaining requests");
                }
            }
        }
        _ => {}
    }

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
    wtr.encode(headers);

    println!("{} {}", say::highlight("Total issues collected:"), issues.len());

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

        wtr.encode(row);
    }
}

#[derive(Debug, RustcDecodable, RustcEncodable)]
struct Issue {
    assignee: Option<User>,
    body: Option<String>,
    created_at: Option<String>,
    closed_at: Option<String>,
    labels: Labels,
    number: u32,
    state: Option<String>,
    title: Option<String>,
    user: Option<User>,
}

type Issues = Vec<Issue>;

type Labels = Vec<Label>;

#[derive(Debug, RustcDecodable, RustcEncodable)]
struct Label {
    color: String,
    name: String,
}

#[derive(Debug, RustcDecodable, RustcEncodable)]
struct User {
    login: String,
}

#[derive(Debug, RustcDecodable, RustcEncodable)]
struct GithubError {
    message: String,
    documentation_url: String,
}
