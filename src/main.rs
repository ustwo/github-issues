#[macro_use] extern crate clap;
#[macro_use] extern crate log;
extern crate env_logger;

extern crate nerve;

use clap::{Arg, App, SubCommand};

use nerve::cmd;
use nerve::format::{OutputFormat};
use nerve::validators::{is_repopath};

fn main() {
    env_logger::init().unwrap();

    let upload_about = "Upload issues from a CSV file. If the first line is
detected to be a header it will be ignored. The fields are identified and
consumed in order:

1. `title`
2. `body`
3. `labels`
4. `assignees`
5. `milestone_id`";

    let matches = App::new("github-issues")
                      .version(env!("CARGO_PKG_VERSION"))
                      .author("Arnau Siches <arnau@ustwo.com>")
                      .about("Github issues consumer.")
                      .subcommand(SubCommand::with_name("upload-template")
                                             .about("Generates a CSV example as an easy start for the upload command."))
                      .subcommand(SubCommand::with_name("upload")
                                             .about(upload_about)
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
                                             .arg(Arg::with_name("input")
                                                      .help("Read input from <file>")
                                                      .long("input")
                                                      .value_name("file")
                                                      .required(true))
                                             .arg(Arg::with_name("check")
                                                      .help("Check if any records in the provided CSV have potential collisions in existing Issues. This flag makes the command noop.")
                                                      .long("check")))
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


    if let Some(_) = matches.subcommand_matches("upload-template") {
        println!(r#"title,body,labels,assignees,milestone_id
"A nice title","A descriptive body","in_backlog,feature",arnau,1"#);
    }

    if let Some(ref matches) = matches.subcommand_matches("upload") {
        let repopath = matches.value_of("repopath").unwrap().to_owned();
        let oauth_token = matches.value_of("oauth-token").unwrap().to_owned();
        let input_file = matches.value_of("input").unwrap().to_owned();

        if matches.is_present("check") {
            cmd::check::run(repopath,
                            oauth_token,
                            input_file);
        } else {
            cmd::upload::run(repopath,
                             oauth_token,
                             input_file);
        }
    }

}
