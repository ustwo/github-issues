use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};

pub mod entities;
pub mod response;

/// Hyper Mime for application/vnd.github.v3+json
pub fn mime() -> Mime {
    Mime(TopLevel::Application,
         SubLevel::Ext("vnd.github.v3+json".to_owned()),
         vec![(Attr::Charset, Value::Utf8)])
}
