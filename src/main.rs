extern crate csv;
extern crate curl;
extern crate docopt;
extern crate regex;
extern crate rustc_serialize;
extern crate ansi_term;

#[macro_use]
extern crate log;

use docopt::Docopt;

mod say;
mod cmd;

const USAGE: &'static str = "
Github issue consumer.

Usage:
    github-issues fetch <repopath> --oauth-token=<oauth_token> --csv --output=<file> [--state=<state>] [--label=<label>...]
    github-issues --version
    github-issues (-h | --help)

Options:
    -h, --help                        Display this message
    --version                         Display the current version
    --oauth-token=<oauth_token>       Github OAuth authorisation token
    --csv                             Output CSV
    --output=<file>                   File where to store the data
    --state=<state>                   Issue state [default: all]. Values: all | open | closed
    --label=<label>                   Github label to filter with
";

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_version: bool,
    cmd_fetch: bool,
    arg_repopath: String,
    flag_label: Vec<String>,
    flag_oauth_token: String,
    flag_csv: bool,
    flag_output: String,
    flag_state: String,
}

pub fn version() -> String {
    let (maj, min, pat) = (option_env!("CARGO_PKG_VERSION_MAJOR"),
                           option_env!("CARGO_PKG_VERSION_MINOR"),
                           option_env!("CARGO_PKG_VERSION_PATCH"));
    match (maj, min, pat) {
        (Some(maj), Some(min), Some(pat)) =>
            format!("{}.{}.{}", maj, min, pat),
        _ => "".to_owned(),
    }
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());

    if args.flag_version {
        println!("github-issues version {}", version());
        return;
    }

    if args.cmd_fetch {
        cmd::fetch::run(args.arg_repopath,
                        args.flag_oauth_token,
                        args.flag_label,
                        args.flag_state,
                        args.flag_output);
    }
}
