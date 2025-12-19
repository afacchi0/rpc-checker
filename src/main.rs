mod rpc;
mod types;

use types::CheckResult;

fn main() {
    let result = CheckResult {
        protocol: "tendermint".to_string(),
        rpc: "http://localhost:26657".to_string(),
        reachable: true,
        latest_block: Some(123),
        syncing: Some(false),
        lag: Some(0),
        error: None,
    };

    println!("{}", serde_json::to_string_pretty(&result).unwrap());
}
