// Github entities represented as structs.

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

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct GithubError {
    pub message: String,
    pub documentation_url: String,
}
