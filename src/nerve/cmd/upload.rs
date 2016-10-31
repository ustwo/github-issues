//! Upload command
//!
//! It uploads issues from a CSV.

use csv;

pub type Issues = Vec<NewIssue>;

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct NewIssue {
    pub assignees: Option<Vec<String>>,
    pub body: Option<String>,
    pub labels: Option<Vec<String>>,
    pub title: String,
}


pub fn run(repopath: String,
           oauth_token: String,
           filename: String) {

    let issues: Issues = from_file(filename);

    println!("{:?}", issues);
}


// CSV lib does not cast Vec<String> so this is a workaround.
fn split(s: String) -> Vec<String> {
   s.split(",").map(From::from).collect()
}


fn from_file(filename: String) -> Issues {
    let mut records = csv::Reader::from_file(filename).unwrap();
    let mut issues: Issues = vec![];

    for record in records.decode() {
        let (title, body, labels, assignees): (String, Option<String>, Option<String>, Option<String>) =
            record.unwrap();

        issues.push(NewIssue { assignees : assignees.map(split)
                             , body : body
                             , labels : labels.map(split)
                             , title : title
                             });
    }

    issues
}
