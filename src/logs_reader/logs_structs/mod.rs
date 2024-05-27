use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    #[serde(rename = "is_all_logs")]
    pub is_all_logs: bool,
    pub version: String,
    #[serde(rename = "log_list_timestamp")]
    pub log_list_timestamp: String,
    pub operators: Vec<Operator>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Operator {
    pub name: String,
    pub email: Vec<String>,
    pub logs: Vec<Log>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Log {
    pub description: String,
    #[serde(rename = "log_id")]
    pub log_id: String,
    pub key: String,
    pub url: String,
    pub mmd: i64,
    pub state: Option<State>,
    #[serde(rename = "temporal_interval")]
    pub temporal_interval: Option<TemporalInterval>,
    #[serde(rename = "log_type")]
    pub log_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct State {
    pub rejected: Option<Rejected>,
    pub usable: Option<Usable>,
    pub readonly: Option<Readonly>,
    pub retired: Option<Retired>,
    pub pending: Option<Pending>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rejected {
    pub timestamp: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Usable {
    pub timestamp: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Readonly {
    pub timestamp: String,
    #[serde(rename = "final_tree_head")]
    pub final_tree_head: FinalTreeHead,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FinalTreeHead {
    #[serde(rename = "sha256_root_hash")]
    pub sha256_root_hash: String,
    #[serde(rename = "tree_size")]
    pub tree_size: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Retired {
    pub timestamp: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pending {
    pub timestamp: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TemporalInterval {
    #[serde(rename = "start_inclusive")]
    pub start_inclusive: String,
    #[serde(rename = "end_exclusive")]
    pub end_exclusive: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tree {
    #[serde(rename = "tree_size")]
    pub tree_size: i64,
    pub timestamp: i64,
    #[serde(rename = "sha256_root_hash")]
    pub sha256_root_hash: String,
    #[serde(rename = "tree_head_signature")]
    pub tree_head_signature: String,
}
