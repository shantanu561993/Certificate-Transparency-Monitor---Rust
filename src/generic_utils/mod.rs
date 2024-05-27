use std::io::ErrorKind;

use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize,Debug)]
pub struct Entries {
    pub entries: Vec<Entry>
}

#[derive(Serialize, Deserialize,Debug)]
pub struct Entry {
    pub leaf_input: String,
    pub extra_data: String,
}

pub async fn read_base64_entries(base64_entries:String)->Result<Entries, std::io::Error>{
    match serde_json::from_str(&base64_entries){
        Ok(entries) => Ok(entries),
        Err(e) => {
           if base64_entries == r#"{"entries":null}"#{
            return Ok(Entries{entries:vec![]});
           }
           Err(std::io::Error::new(ErrorKind::Other, format!("{}",e)))
        }
    }
}