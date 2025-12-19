use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct CheckResult {
    pub protocol: String,
    pub rpc: String,
    pub reachable: bool,
    pub latest_block: Option<u64>,
    pub syncing: Option<bool>,
    pub lag: Option<u64>,
    pub error: Option<String>,
}
