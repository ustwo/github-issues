use hyper::Client;
use hyper::status::StatusCode;
use hyper::header::{ Accept, Authorization, Connection, UserAgent, qitem };
use hyper;
use rustc_serialize::json;
use std::process;

use say;
use github::mime;
use github::entities::{ NewIssue };
use github::response::{ Response, ResponseError };

pub fn create(url: &str, token: &str, issue: &NewIssue) -> Result<Response, ResponseError> {
    let client = Client::new();
    let body = json::encode(issue).unwrap();

    let res = client.post(&*url.clone())
                    .body(&body)
                    .header(UserAgent(format!("nerve/{}", env!("CARGO_PKG_VERSION"))))
                    .header(Authorization(format!("token {}", token)))
                    .header(Accept(vec![qitem(mime())]))
                    .header(Connection::close())
                    .send().unwrap_or_else(|_| process::exit(1));

    match res.status {
        hyper::Ok => {}
        StatusCode::Created => {}
        StatusCode::UnprocessableEntity => {
            let body: Response = From::from(res);
            let err = ResponseError::from_str(&body.content).unwrap();

            return Err(err);
        }
        e => {
            println!("{} {}", say::error(), e);
            process::exit(1)
        }
    }

    Ok(From::from(res))
}
