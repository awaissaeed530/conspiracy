use clap::{Command, Arg, ArgAction, ArgMatches};
use twitter_dl::downloader::{TwitterVideoOptions, TwitterDownloader};

fn main() {
    let matches = Command::new("Twitter DL")
        .args([
              Arg::new("url"),
              Arg::new("output_path").short('o').long("output_path"),
              Arg::new("cookies_browser").short('b').long("cookies_browser").required(true),
              Arg::new("verbose").short('v').long("verbose").action(ArgAction::SetTrue),
        ])
        .get_matches();

    let options = build_options(&matches);
    TwitterDownloader::new().download(&options);
}

fn build_options(matches: &ArgMatches) -> TwitterVideoOptions {
    let mut options = TwitterVideoOptions::default();
    if let Some(url) = matches.get_one::<String>("url") {
        options.url = url.to_owned();
    }
    if let Some(output_path) = matches.get_one::<String>("output_path") {
        options.output_path = Some(output_path.to_owned());
    }
    if let Some(cookies_browser) = matches.get_one::<String>("cookies_browser") {
        options.cookies_browser = cookies_browser.to_owned();
    }
    options.verbose = Some(matches.get_flag("verbose"));
    options
}
