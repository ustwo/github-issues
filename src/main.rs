extern crate ansi_term;

#[macro_use]
extern crate clap;
extern crate csv;
extern crate curl;
extern crate env_logger;
extern crate regex;
extern crate rustc_serialize;

#[macro_use]
extern crate log;

use clap::{Arg, App, SubCommand};

mod say;
mod format;
mod cmd;

use format::{OutputFormat};

fn main() {
    env_logger::init().unwrap();

    let matches = App::new("github-issues")
                      .version(crate_version!())
                      .author("Arnau Siches <arnau@ustwo.com>")
                      .about("Github issues consumer.")
                      .subcommand(SubCommand::with_name("fetch")
                                             .about("Fetch issues from Github.")
                                             .arg(Arg::with_name("repopath")
                                                      .help("Repo path (e.g. ustwo/mastermind)")
                                                      .index(1)
                                                      .validator(is_repopath)
                                                      .required(true))
                                             .arg(Arg::with_name("oauth-token")
                                                      .help("Github OAuth authorisation token")
                                                      .long("oauth-token")
                                                      .value_name("oauth_token")
                                                      .required(true))
                                             .arg(Arg::with_name("format")
                                                      .help("Output format")
                                                      .long("format")
                                                      .value_name("format")
                                                      .possible_values(&["csv", "json"])
                                                      .required(true))
                                             .arg(Arg::with_name("output")
                                                      .help("Write output to <file>")
                                                      .long("output")
                                                      .value_name("file")
                                                      .required(true))
                                             .arg(Arg::with_name("state")
                                                      .help("Issue state. Defaults to \"all\"")
                                                      .long("state")
                                                      .value_name("state")
                                                      .possible_values(&["all", "open", "closed"]))
                                             .arg(Arg::with_name("label")
                                                      .help("Github label to filter with")
                                                      .long("label")
                                                      .value_name("label")
                                                      .multiple(true)))
                      .get_matches();

    if let Some(ref matches) = matches.subcommand_matches("fetch") {
        let labels: Vec<String> = values_t!(matches, "label", String).unwrap_or(vec![]);
        let state = matches.value_of("state").unwrap_or("all").to_owned();
        let repopath = matches.value_of("repopath").unwrap().to_owned();
        let oauth_token = matches.value_of("oauth-token").unwrap().to_owned();
        let format = value_t!(matches, "format", OutputFormat).unwrap_or_else(|e| e.exit());
        let output = matches.value_of("output").unwrap().to_owned();

        cmd::fetch::run(repopath,
                        oauth_token,
                        labels,
                        state,
                        format,
                        output);
    }
}

// Validators

fn is_repopath(value: String) -> Result<(), String> {
    if value.split("/").collect::<Vec<&str>>().len() < 2 {
        return Err(String::from("<repopath> must have the form <owner>/<repo>.  e.g. ustwo/github-issues"));
    }

    Ok(())
}
