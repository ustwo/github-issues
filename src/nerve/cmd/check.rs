//! Check command
//!
//! It checks if any records in a CSV already exist as issues in Github.

use csv;

use say;
use checker::filter_by_similar;
use github::issues;
use github::entities::{Issue, Issues, NewIssues, issues_from_json};
use github::response::Page;

pub fn run(repopath: String, oauth_token: String, filename: String) {
    let existing_issues = fetch_open_issues(&repopath, &oauth_token);
    let title_list = extract_titles(existing_issues);

    let records = csv::Reader::from_file(filename).unwrap();
    let issues: NewIssues = NewIssues::from(records);


    for new_issue in issues {
        let similars = filter_by_similar(&new_issue.title, &title_list, 0.6);

        println!("{}: {:?}", new_issue.title, similars);

    }
}


fn extract_titles<'a>(issues: Issues) -> Vec<(u32, String)> {
    issues.into_iter()
          .map(|issue| (issue.number, issue.title.unwrap_or("".to_string())))
          .collect()
}

fn fetch_open_issues(repopath: &str, oauth_token: &str) -> Issues {
    let url = format!("https://api.github.com/repos/{repopath}/issues?filter=all&state=open",
                      repopath = repopath);

    let page = Page::new(&url, &oauth_token);

    let mut issues: Issues = issues_from_json(&page.content).unwrap();
    let mut next_url = page.next.clone();

    page.warn();

    while let Some(url) = next_url {
        let page = Page::new(&url, &oauth_token);
        issues.extend(issues_from_json(&page.content).unwrap());
        next_url = page.next.clone();

        page.warn();
    }

    println!("{} {}", say::highlight("Total issues collected:"), issues.len());

    issues
}
