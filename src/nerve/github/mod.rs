use hyper::mime;

pub mod entities;
pub mod response;


/// Hyper Mime for application/vnd.github.v3+json
pub fn mime() -> mime::Mime {
    mime::Mime(mime::TopLevel::Application,
               mime::SubLevel::Ext("vnd.github.v3+json".to_owned()),
               vec![(mime::Attr::Charset, mime::Value::Utf8)])
}
