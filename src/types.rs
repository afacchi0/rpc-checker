use serde::Serialize;

#[derive(Debug, Clone)]
pub enum Protocol {
    Tendermint,
    Ethereum,
    Bitcoin,
}

#[derive(Debug, Clone)]
pub enum TendermintMethod {
    Status,
    Health,
    Block { height: Option<u64> },
}

#[derive(Debug, Clone)]
pub enum EthereumMethod {
    Status,
    Health,
    Block { height: Option<u64> },
}

#[derive(Debug, Clone)]
pub enum BitcoinMethod {
    Status,
    Health,
    Block { height: Option<u64> },
}

#[derive(Debug, Clone)]
pub enum Command {
    Tendermint(TendermintMethod),
    Ethereum(EthereumMethod),
    Bitcoin(BitcoinMethod),
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ResultData {
    Status {
        latest_block: Option<u64>,
        syncing: Option<bool>,
    },
    Health {
        healthy: bool,
    },
    Block {
        height: Option<u64>,
    },
}

#[derive(Debug, Serialize)]
pub struct CheckResult {
    pub protocol: String,
    pub rpc: String,
    pub reachable: bool,
    pub result: Option<ResultData>,
    pub error: Option<String>,
}
