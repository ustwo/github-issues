use csv;
use std::str;
use std::process;
use curl::http;
use rustc_serialize::json;


pub fn run(owner: &str,
           repo: &str,
           oauth_token: String,
           labels: Vec<String>) {
    let url = format!("https://api.github.com/repos/{}/{}/issues", owner, repo);
    let auth_header = format!("token {}", oauth_token);
    // println!("{:?}, {:?}", owner, repo);
    // println!("{:?}", labels);

    let res = http::handle()
                   .get(url)
                   .header("Authorization", &auth_header)
                   .header("User-Agent", "Github-Issues-CLI")
                   .header("Accept", "application/vnd.github.v3+json")
                   .exec()
                   .unwrap_or_else(|e| process::exit(1));

    let body = match str::from_utf8(res.get_body()) {
        Ok(b) => b,
        Err(..) => "Unable to parse"
    };

    // let issues: json::Json = body.parse().unwrap();
    let issues: Vec<Issue> = json::decode(body).unwrap();

    println!("code={}; headers={:?}",
             res.get_code(),
             res.get_headers());

    let mut wtr = csv::Writer::from_memory();
    let headers = ("number",
                   "created_at",
                   "title",
                   "body",
                   "labels",
                   "user");
    wtr.encode(headers);

    for issue in issues {
        let labels = issue.labels.iter()
                                 .map(|x| x.name.clone())
                                 .collect::<Vec<String>>()
                                 .join(",");

        let row = (issue.number,
                   issue.created_at,
                   issue.title,
                   issue.body,
                   labels,
                   issue.user.login);

        // println!("{:?}", row);
        let result = wtr.encode(row);

        println!("{:?}", result.is_ok());
    }

    println!("{:?}", wtr.as_string());
}

#[derive(Debug, RustcDecodable, RustcEncodable)]
struct Issue {
    body: String,
    created_at: String,
    labels: Vec<Label>,
    number: u32,
    title: String,
    user: User,
}

#[derive(Debug, RustcDecodable, RustcEncodable)]
struct Label {
    color: String,
    name: String,
}

#[derive(Debug, RustcDecodable, RustcEncodable)]
struct User {
    login: String,
}
