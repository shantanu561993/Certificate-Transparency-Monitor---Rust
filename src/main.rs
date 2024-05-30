#[allow(dead_code,unused_variables,dropping_copy_types)]
pub mod merkle_tree;
pub mod generic_utils;
mod logs_reader;
use clap::{Arg, ArgGroup, Command};

#[tokio::main]
async fn main() {
    let matches = Command::new("Certificate Transparency Lists Monitor")
    .version("0.1")
    .author("Shantanu Khandelwal<shantanukhandelwal@protonmail.com>")
    .about("Tool to monitor and dump Certificate Transparency Lists")
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
    .arg(
        Arg::new("dump")
            .long("dump")
            .value_name("dump")
            .help("Dump a single Certificate Transparency List.")
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
    else if let Some(dump) = matches.get_one::<String>("dump"){
        println!("Please wait while we fetch the list of all the operators. This may take a while.");
        let operators = logs_reader::get_all_working_operators().await;
        let main_list: Vec<(String,String)> = operators.iter().map(|(key, value)| (key.clone(),value.clone())).collect();
        drop(operators);
        println!("Select the Log Source to parse:");
        for (index, (name,url)) in main_list.iter().enumerate() {
            println!("{index}: {name}=>{url}");
        }
        println!("Enter the index of the Log Source to parse:");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let index = input.trim().parse::<usize>().unwrap();
        let (name,url) = main_list[index].clone();
        drop(index);
        drop(main_list);
        drop(input);
        let mut client : reqwest::Client = reqwest::Client::new();
        logs_reader::dump_single_operator(&url).await;
    }
    else{
        logs_reader::root_reader(None).await;
    }
}
