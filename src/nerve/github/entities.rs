// Github entities represented as structs.
use std::fmt;
use rustc_serialize::json;

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
pub struct Error {
    pub message: String,
    pub errors: Vec<ErrorResource>,
}

impl Error {
    pub fn from_str(data: &str) -> Result<Error, json::DecoderError> {
        json::decode(data)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let error = self.errors.first().unwrap();

        write!(f, "the field '{}' {}", error.field, "has an invalid value.")
    }
}

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct ErrorResource {
    pub code: String,
    pub resource: String,
    pub field: String,
}

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub enum ErrorName {
    Invalid,
    Missing,
    MissingField,
    AlreadyExists,
}
