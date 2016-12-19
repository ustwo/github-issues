//! Check command
//!
//! It checks if any records in a CSV already exist as issues in Github.

use csv;

use say;
use checker::filter_by_similar;
use github::entities::{Issues, NewIssues, issues_from_json};
use github::response::Page;

pub fn run(repopath: String, oauth_token: String, filename: String) {
    let href = format!("https://github.com/{}/issues/", repopath);
    let existing_issues = fetch_open_issues(&repopath, &oauth_token);
    let title_list = extract_titles(existing_issues);

    let records = csv::Reader::from_file(filename).unwrap();
    let issues: NewIssues = NewIssues::from(records);


    for (i, new_issue) in issues.into_iter().enumerate() {
        let similars = filter_by_similar(&new_issue.title, &title_list, 0.6);

        if !similars.is_empty() {
            println!("#{} {}", i, say::highlight(&new_issue.title));

            for (num, title) in similars {
                println!("    {}{} {}", href, say::red(&num.to_string()), say::yellow(&title));
            }
        }

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

    issues
}
