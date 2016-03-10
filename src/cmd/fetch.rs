use std::io::prelude::*;
use csv;
use curl::http;
use regex::Regex;
use rustc_serialize::json;
use std::collections::HashMap;
use std::fs::File;
use std::process;
use std::result::Result;
use std::str;

use say;
use format::{OutputFormat};
use github::entities::{Issues, GithubError};

fn ratelimit(headers: &HashMap<String, Vec<String>>) -> u32 {
    let rate = headers.get("x-ratelimit-remaining").unwrap()
                      .first().unwrap();

    rate.parse().unwrap()
}
#[test]
fn valid_ratelimit() {
    let mut headers: HashMap<String, Vec<String>> = HashMap::new();
    headers.insert("x-ratelimit-remaining".to_owned(), vec!["1".to_owned()]);

    assert_eq!(ratelimit(&headers), 1);
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
                   .unwrap_or_else(|_| process::exit(1));

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

fn as_issues(raw: &[u8]) -> Result<Issues, json::DecoderError> {
    match str::from_utf8(raw) {
        Ok(b) => json::decode(b),
        Err(..) => {
            println!("{} {}", say::error(), "Unable to parse the response from Github");
            process::exit(1)
        }
    }
}

// TODO: Review its need
// fn parse_repopath(path: String) -> (String, String) {
//     let list: Vec<&str> = path.split("/").collect();

//     (list[0].to_string(), list[1].to_string())
// }

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

    let res = get_page(url, &oauth_token);
    let mut issues = as_issues(res.get_body()).unwrap();

    // A Link header is not present if the requested collection has less than
    // _pagesize_.
    match res.get_headers().get("link") {
        Some(links) => {
            let mut nurl = next_url(links.first().unwrap().clone());

            while let Some(nu) = nurl {
                let r = get_page(nu.to_string(), &oauth_token);
                issues.extend(as_issues(r.get_body()).unwrap());

                let link = r.get_headers().get("link").unwrap()
                                          .first().unwrap()
                                          .clone();
                nurl = next_url(link);
                if nurl.is_none() {
                    println!("{} {} {}", say::warn(), ratelimit(r.get_headers()), "Remaining requests");
                }
            }
        }
        _ => {
            println!("{} {} {}", say::warn(), ratelimit(res.get_headers()), "Remaining requests");
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
