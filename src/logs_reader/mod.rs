mod logs_structs;
use std::collections::HashMap;
use reqwest::Client;
use logs_structs::{Root,Tree};
use std::boxed::Box;

use crate::generic_utils::{read_base64_entries, Entries};
use crate::merkle_tree::utils::read_entry;
use regex::RegexSet;





pub async fn root_reader(bounty_target:Option<String>){
    let client:Client = Client::new();
    const ROOT_URL:&str =  "https://www.gstatic.com/ct/log_list/v3/all_logs_list.json";
    let data =  send_request(&ROOT_URL.to_string(), client.clone()).await.unwrap_or_else(|error| panic!("Unable to send request. {error}"));
    let parsed_root : Root = serde_json::from_str(&data).unwrap_or_else(|error| panic!("Unable to parse root {error}"));
    let operators = get_log_sources(parsed_root).await;
    let mut tasks = Vec::new();
    let mut regex_set:RegexSet = RegexSet::empty();
    if bounty_target.is_some(){
        let targets = send_request(&bounty_target.unwrap(), client.clone()).await.unwrap();
        let targets:String = "amazonaws".to_string();
        let fix1 = targets.replace("\n", "$\n").replace(".", r#"\."#).replace("*", ".*");
        let targets_vec = fix1.lines().collect::<Vec<_>>();
        println!("{:?}",targets_vec);
        regex_set = RegexSet::new(&targets_vec).expect("Failed to compile regex set");
    }

    for (name, url) in operators {
        // tasks.spawn(process_operator(name, url,client.clone()));
        tasks.push(process_operator(name, url,client.clone(),regex_set.clone()));    
    }


    // while let Some(_) = tasks.join_next().await{
    // }
    let _ = futures::future::join_all(tasks).await;
}


async fn send_request(url: &String, client: Client) -> Result<String, Box<dyn std::error::Error>> {
    let response = client.get(url).send().await?.error_for_status()?;
    let body = response.text().await?;
    return Ok(body);   
}


async fn get_log_sources(root: Root) -> HashMap<String, String> {
    let mut operators: HashMap<String, String> = HashMap::new();
    for op in root.operators {
        for log in op.logs {
            operators.insert(log.description.to_string(), log.url.to_string());
        }
    }
    return operators;
}
async fn process_operator(_name:String,url:String,client:Client,regex_set:RegexSet)->Result<String, Box<dyn std::error::Error>>{
    let mut info_endpoint = url.clone();
    info_endpoint.push_str("ct/v1/get-sth");
    let response = send_request(&info_endpoint, client.clone()).await?;
    let tree:Tree = serde_json::from_str(&response)?;
    let tree_size = tree.tree_size;
    let mut size_request = url.clone();
    size_request.push_str("ct/v1/get-entries?start=0&end=10000");
    let mut response:String = String::new();
    loop {
     match send_request(&size_request, client.clone()).await{
        Ok(resp)=>{
            response.push_str(&resp);
            break
        }
        Err(_)=>continue
     };
    }
    
    let batch_size = read_base64_entries(response).await.unwrap().entries.len();
    let mut start = tree_size;
    loop{
        let mut watch_request = url.clone();
        let end = start+(batch_size as i64);
        watch_request.push_str(&format!("ct/v1/get-entries?start={start}&end={end}")[..]);
        let mut response : Result<String,std::io::Error> ;
        loop {
            response = match send_request(&watch_request, client.clone()).await{
                Ok(res)=>{Ok(res)},
                Err(_)=>Err(std::io::Error::new(std::io::ErrorKind::Other,"Req failed"))
            };
            match response{
                Ok(_)=> break,
                Err(_)=>continue
            };
        }
        let response = response.unwrap_or_else(|_e| "".to_string());
        let entries : Entries = read_base64_entries(response).await.unwrap_or_else(|_err| Entries{entries:vec![]});
        start = start + (entries.entries.len() as i64);
        for entry in entries.entries{
            let results = read_entry(&entry).await;
            if regex_set.is_empty() {
                println!("{:?}",results);
            }
            else if !regex_set.is_empty() {
                let all_domains = results.get("all_domains");
                if all_domains.is_some(){
                    let all_domains = results.get("all_domains").unwrap();
                    for domain in all_domains{
                        let matches: Vec<_> = regex_set.matches(&domain).into_iter().collect();
                        if !matches.is_empty(){
                            println!("{domain}");
                        }
                    }
                }
            }
        }
    }
}
