#[allow(dead_code,unused_variables)]
pub mod merkle_tree;
pub mod generic_utils;
mod logs_reader;
use clap::{Arg, ArgGroup, Command};


#[tokio::main]
async fn main() {
    let matches = Command::new("Certificate Transparency Lists Monitor")
    .version("0.1")
    .author("Shantanu Khandelwal<shantanukhandelwal@protonmail.com>")
    .about("Cyber Security Researcher")
    .arg(
        Arg::new("regex")
            .long("regex")
            .value_name("REGEX")
            .help("A regex pattern to monitor. Example .*\\.com for monitoring all .com domains")    
        )
    .arg(
        Arg::new("url")
            .long("url")
            .value_name("URL")
            .help("A URL to list of wildcards of bugbounty.")
    )
    .group(
        ArgGroup::new("input")
            .args(&["regex", "url"])
            .required(false)
            .multiple(false),
    )
    .get_matches();
    if let Some(regex) = matches.get_one::<String>("regex") {
        logs_reader::root_reader(Some(vec!["REGEX".to_string(),regex.to_string()])).await
    }
    else if let Some(url) = matches.get_one::<String>("url") {
        logs_reader::root_reader(Some(vec!["URL".to_string(),url.to_string()])).await
    }
    else{
        logs_reader::root_reader(None).await;
    }
}
