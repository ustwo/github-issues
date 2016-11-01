//! Upload command
//!
//! It uploads issues from a CSV.

use csv;
use rustc_serialize::json;

use say;
use github::issues::{self, Issues, NewIssue};
use github::entities::{Issue};


pub fn run(repopath: String,
           oauth_token: String,
           filename: String) {

    let issues: Issues = from_file(filename);

    let url = format!("https://api.github.com/repos/{repopath}/issues",
                      repopath = repopath);


    for new_issue in issues {
        let res = issues::create(&url, &oauth_token, new_issue);
        let issue: Issue = as_issue(&res.content).unwrap();

        println!("{} {} {} {}", say::info(), "Created issue number",
                 issue.number,
                 issue.title.unwrap_or("missing title".to_string()));

    }
}


fn as_issue(data: &str) -> Result<Issue, json::DecoderError> {
    json::decode(data)
}



// CSV lib does not cast Vec<String> so this is a workaround.
fn split(s: String) -> Vec<String> {
    if s.is_empty() {
        vec![]
    } else {
        s.split(",").map(From::from).collect()
    }
}


fn from_file(filename: String) -> Issues {
    let mut records = csv::Reader::from_file(filename).unwrap();
    let mut issues: Issues = vec![];

    for record in records.decode() {
        let (title, body, labels, assignees): (String, String, String, String) =
            record.unwrap();

        issues.push(NewIssue { assignees : split(assignees)
                             , body : body
                             , labels : split(labels)
                             , title : title
                             });
    }

    issues
}
