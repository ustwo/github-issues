//! Upload command
//!
//! It uploads issues from a CSV.
use csv;

use say;
use github::issues;
use github::entities::{Issue, NewIssues};


pub fn run(repopath: String,
           oauth_token: String,
           filename: String) {

    let records = csv::Reader::from_file(filename).unwrap();
    let issues: NewIssues = NewIssues::from(records);

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
