//! Fetch command
//!
//! It fetches issues and serialises them based on the defined output format.
//! Currently csv or json.

use csv;
use rustc_serialize::json;
use std::fs::File;
use std::io::prelude::*;
use std::result::Result;

use say;
use format::{OutputFormat};
use github::entities::{Issues};
use github::response::{Page};


fn as_issues(data: &str) -> Result<Issues, json::DecoderError> {
    json::decode(data)
}

pub fn run(repopath: String,
           oauth_token: String,
           labels: Vec<String>,
           state: String,
           format: OutputFormat,
           output_file: String) {

    let labels_pair = if labels.is_empty() { "".to_owned() }
                      else { format!("&labels={}", labels.join(",")) };
    let url = format!("https://api.github.com/repos/{repopath}/issues?filter=all&state={state}{labels_pair}",
                      repopath = repopath,
                      state = state,
                      labels_pair = labels_pair);

    let page = Page::new(&url, &oauth_token);
    let mut issues = as_issues(&page.content).unwrap();
    let mut next_url = page.next.clone();

    page.warn();

    while let Some(url) = next_url {
        let page = Page::new(&url, &oauth_token);
        issues.extend(as_issues(&page.content).unwrap());
        next_url = page.next.clone();

        page.warn();
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
