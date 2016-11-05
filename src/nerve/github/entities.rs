// Github entities represented as structs.
use std::io;
use csv;
use rustc_serialize::json;

use format::split;

/// An Issue-to-be. It doesn't have number, state or any timestamp because
/// it is not yet created.
///
/// The flow is then:
///
/// create(NewIssue) -> Issue
#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct NewIssue {
    pub assignees: Vec<String>,
    pub body: String,
    pub labels: Vec<String>,
    pub title: String,
    pub milestone: Option<u32>,
}

type Record = (String, String, String, String, Option<u32>);

impl From<Record> for NewIssue {
    fn from(record: Record) -> Self {
        let (title, body, labels, assignees, milestone):
            (String, String, String, String, Option<u32>) = record;

        NewIssue { assignees : split(assignees)
                 , body : body
                 , labels : split(labels)
                 , title : title
                 , milestone : milestone
                 }
    }
}

pub struct NewIssues(Vec<NewIssue>);


impl NewIssues {
    fn new() -> Self {
        NewIssues(Vec::new())
    }

    fn push(&mut self, elem: NewIssue) {
        self.0.push(elem);
    }
}

impl IntoIterator for NewIssues {
    type Item = NewIssue;
    type IntoIter = ::std::vec::IntoIter<NewIssue>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl <T: io::Read>From<csv::Reader<T>> for NewIssues {
    fn from(mut records: csv::Reader<T>) -> Self {
        let mut issues: NewIssues = NewIssues::new();

        for record in records.decode::<Record>() {
            issues.push(NewIssue::from(record.unwrap()));
        }

        issues
    }
}


/// A partial representation of a Github Issue. The represented fields are the
/// only ones transported to the final CSV/Json output. Might be interesting
/// to add some more bits like the milestone.
#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct Issue {
    pub assignee: Option<User>,
    // pub assignees: Vec<User>,
    pub body: Option<String>,
    pub created_at: Option<String>,
    pub closed_at: Option<String>,
    pub labels: Labels,
    pub number: u32,
    // pub id: u32,
    pub state: Option<String>,
    pub title: Option<String>,
    pub user: Option<User>,
}

impl Issue {
    pub fn from_str(data: &str) -> Result<Issue, json::DecoderError> {
        json::decode(data)
    }
}


pub type Issues = Vec<Issue>;

pub fn issues_from_json(data: &str) -> Result<Issues, json::DecoderError> {
    json::decode(data)
}


pub type Labels = Vec<Label>;

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct Label {
    pub name: String,
}

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct User {
    pub login: String,
}
