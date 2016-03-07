use csv;
use std::result::Result;
use std::str;
use std::process;
use curl::http;
use rustc_serialize::json;

// fn get_page(url: String, token: String) -> Result<Vec<Issue>, json::DecoderError> {
//     let auth_header = format!("token {}", token);
//     let res = http::handle()
//                    .get(url)
//                    .header("Authorization", &auth_header)
//                    .header("User-Agent", "Github-Issues-CLI")
//                    .header("Accept", "application/vnd.github.v3+json")
//                    .exec()
//                    .unwrap_or_else(|e| process::exit(1));

//     let body = match str::from_utf8(res.get_body()) {
//         Ok(b) => b,
//         Err(..) => "Unable to parse"
//     };

//     // let issues_result: Result<Vec<Issue>,_> = json::decode(body);
//     // issues_result
//     json::decode(body)
// }

fn get_page(owner: &str,
            repo: &str,
            page: u16,
            labels: Vec<String>,
            token: String) -> http::Response {

    let url = format!("https://api.github.com/repos/{}/{}/issues?state=all&page={}",
                      owner, repo, page);
    let auth_header = format!("token {}", token);
    let res = http::handle()
                   .get(url)
                   .header("Authorization", &auth_header)
                   .header("User-Agent", "Github-Issues-CLI")
                   .header("Accept", "application/vnd.github.v3+json")
                   .exec()
                   .unwrap_or_else(|e| process::exit(1));

    res
}


pub fn run(owner: &str,
           repo: &str,
           oauth_token: String,
           labels: Vec<String>) {

    let page = 1;
    let res = get_page(owner, repo, page, labels, oauth_token);
    let body = match str::from_utf8(res.get_body()) {
        Ok(b) => b,
        Err(..) => "Unable to parse"
    };

    let issues_result: Result<Vec<Issue>,_> = json::decode(body);
    let issues = issues_result.unwrap();
    // println!("{:?}", issues);

    println!("{:?}", res.get_headers().get("link"));
    for (k, v) in res.get_headers() {
        println!("{:?}: {:?}", k, v);
    }
    // println!("code={}; headers={:?}",
    //          res.get_code(),
    //          res.get_headers());

    // let mut wtr = csv::Writer::from_memory();
    let mut wtr = csv::Writer::from_file("foo.csv").unwrap();
    let headers = ("number",
                   "title",
                   "state",
                   "created_at",
                   "closed_at",
                   "user",
                   "labels",
                   "body");
    wtr.encode(headers);

    println!("{:?}", issues.len());

    for issue in issues {
        let labels = issue.labels.iter()
                                 .map(|x| x.name.clone())
                                 .collect::<Vec<String>>()
                                 .join(",");

        let row = (issue.number,
                   issue.title,
                   issue.state,
                   issue.created_at,
                   issue.closed_at,
                   issue.user.login,
                   labels,
                   issue.body);

        // println!("{:?}", row);
        // let result = wtr.encode(row);
        // println!("{:?}", result.is_ok());
        wtr.encode(row);
    }

    // println!("{:?}", wtr.as_string());
}

#[derive(Debug, RustcDecodable, RustcEncodable)]
struct Issue {
    body: Option<String>,
    created_at: Option<String>,
    closed_at: Option<String>,
    labels: Vec<Label>,
    number: u32,
    state: Option<String>,
    title: Option<String>,
    user: User,
}

type Labels = Vec<Label>;

#[derive(Debug, RustcDecodable, RustcEncodable)]
struct Label {
    color: String,
    name: String,
}

#[derive(Debug, RustcDecodable, RustcEncodable)]
struct User {
    login: String,
}
