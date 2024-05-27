#[allow(dead_code,unused_variables)]
pub mod merkle_tree;
pub mod generic_utils;
mod logs_reader;


#[tokio::main]
async fn main() {

    logs_reader::root_reader(Some("https://raw.githubusercontent.com/arkadiyt/bounty-targets-data/main/data/wildcards.txt".to_string())).await;
    // logs_reader::root_reader(None).await;

}
