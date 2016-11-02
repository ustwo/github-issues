// Github entities represented as structs.
use rustc_serialize::json;

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

pub type NewIssues = Vec<NewIssue>;


/// A partial representation of a Github Issue. The represented fields are the
/// only ones transported to the final CSV/Json output. Might be interesting
/// to add some more bits like the milestone.
#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct Issue {
    pub assignee: Option<User>,
    pub body: Option<String>,
    pub created_at: Option<String>,
    pub closed_at: Option<String>,
    pub labels: Labels,
    pub number: u32,
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

pub type Labels = Vec<Label>;

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct Label {
    pub name: String,
}

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct User {
    pub login: String,
}
