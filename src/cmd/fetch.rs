use std::str;
use std::process;
use curl::http;


pub fn run(owner: &str, repo: &str, labels: Vec<String>) {
    let url = format!("https://api.github.com/repos/{}/{}/issues", owner, repo);
    // println!("{:?}, {:?}", owner, repo);
    // println!("{:?}", labels);

    let res = http::handle()
                   .get(url)
                   .header("User-Agent", "Github-Issues-CLI")
                   .header("Accept", "application/vnd.github.v3+json")
                   .exec()
                   .unwrap_or_else(|e| process::exit(1));

    let body = match str::from_utf8(res.get_body()) {
        Ok(b) => b,
        Err(..) => "Unable to parse"
    };

    println!("code={}; headers={:?}; body={:?}",
             res.get_code(),
             res.get_headers(),
             body);
}
