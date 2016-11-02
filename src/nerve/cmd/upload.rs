//! Upload command
//!
//! It uploads issues from a CSV.
use csv;

use say;
use format::{split};
use github::issues;
use github::entities::{Issue, NewIssues, NewIssue};


pub fn run(repopath: String,
           oauth_token: String,
           filename: String) {

    let issues: NewIssues = from_file(filename);

    let url = format!("https://api.github.com/repos/{repopath}/issues",
                      repopath = repopath);


    for new_issue in issues {
        let res = issues::create(&url, &oauth_token, &new_issue);

        match res {
            Ok(r) => {
                let issue = Issue::from_str(&r.content).unwrap();

                println!("{} {} {} {}", say::info(), "Created issue number",
                         issue.number,
                         issue.title.unwrap_or("missing title".to_string()));
            }

            Err(e) => {
                println!("{} {} {} {} {}", say::error(),
                         "Couldn't create an issue for", new_issue.title,
                         "because", e);
            }
        }
    }
}


fn from_file(filename: String) -> NewIssues {
    let mut records = csv::Reader::from_file(filename).unwrap();
    let mut issues: NewIssues = vec![];

    for record in records.decode() {
        let (title, body, labels, assignees, milestone):
            (String, String, String, String, Option<u32>) = record.unwrap();

        issues.push(NewIssue { assignees : split(assignees)
                             , body : body
                             , labels : split(labels)
                             , title : title
                             , milestone : milestone
                             });
    }

    issues
}
