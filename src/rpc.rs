use crate::types::{
    BitcoinMethod, CheckResult, Command, EthereumMethod, ResultData, TendermintMethod,
};

pub fn check(cmd: Command, rpc: &str) -> CheckResult {
    match cmd {
        Command::Tendermint(m) => check_tendermint(m, rpc),
        Command::Ethereum(m) => check_ethereum(m, rpc),
        Command::Bitcoin(m) => check_bitcoin(m, rpc),
    }
}

fn check_tendermint(method: TendermintMethod, rpc: &str) -> CheckResult {
    match method {
        TendermintMethod::Status => check_tendermint_status(rpc),
        TendermintMethod::Health => check_tendermint_health(rpc),
        TendermintMethod::Block { height } => check_tendermint_block(rpc, height),
    }
}

fn check_ethereum(method: EthereumMethod, rpc: &str) -> CheckResult {
    match method {
        EthereumMethod::Status => check_ethereum_status(rpc),
        EthereumMethod::Health => check_ethereum_health(rpc),
        EthereumMethod::Block { height } => check_ethereum_block(rpc, height),
    }
}

fn check_bitcoin(method: BitcoinMethod, rpc: &str) -> CheckResult {
    match method {
        BitcoinMethod::Status => check_bitcoin_status(rpc),
        BitcoinMethod::Health => check_bitcoin_health(rpc),
        BitcoinMethod::Block { height } => check_bitcoin_block(rpc, height),
    }
}

fn check_tendermint_status(rpc: &str) -> CheckResult {
    let status_url = format!("{}/status", rpc);

    let resp = reqwest::blocking::get(&status_url);

    match resp {
        Ok(response) => {
            if !response.status().is_success() {
                return CheckResult {
                    protocol: "tendermint".to_string(),
                    rpc: rpc.to_string(),
                    reachable: false,
                    result: None,
                    error: Some(format!("HTTP {}", response.status())),
                };
            }

            let json: Result<serde_json::Value, _> = response.json();
            match json {
                Ok(v) => {
                    let sync_info = &v["result"]["sync_info"];

                    let latest_block = sync_info["latest_block_height"]
                        .as_str()
                        .and_then(|s| s.parse::<u64>().ok());

                    let catching_up = sync_info["catching_up"].as_bool();

                    CheckResult {
                        protocol: "tendermint".to_string(),
                        rpc: rpc.to_string(),
                        reachable: true,
                        result: Some(ResultData::Status {
                            latest_block,
                            syncing: catching_up,
                        }),
                        error: None,
                    }
                }
                Err(e) => CheckResult {
                    protocol: "tendermint".to_string(),
                    rpc: rpc.to_string(),
                    reachable: false,
                    result: None,
                    error: Some(format!("Invalid JSON: {}", e)),
                },
            }
        }
        Err(e) => CheckResult {
            protocol: "tendermint".to_string(),
            rpc: rpc.to_string(),
            reachable: false,
            result: None,
            error: Some(e.to_string()),
        },
    }
}

fn check_tendermint_health(rpc: &str) -> CheckResult {
    let url = format!("{}/health", rpc);

    let resp = reqwest::blocking::get(&url);

    match resp {
        Ok(response) => {
            if response.status().is_success() {
                CheckResult {
                    protocol: "tendermint".to_string(),
                    rpc: rpc.to_string(),
                    reachable: true,
                    result: Some(ResultData::Health { healthy: true }),
                    error: None,
                }
            } else {
                CheckResult {
                    protocol: "tendermint".to_string(),
                    rpc: rpc.to_string(),
                    reachable: false,
                    result: Some(ResultData::Health { healthy: false }),
                    error: Some(format!("HTTP {}", response.status())),
                }
            }
        }
        Err(e) => CheckResult {
            protocol: "tendermint".to_string(),
            rpc: rpc.to_string(),
            reachable: false,
            result: Some(ResultData::Health { healthy: false }),
            error: Some(e.to_string()),
        },
    }
}

fn check_tendermint_block(rpc: &str, height: Option<u64>) -> CheckResult {
    let url = match height {
        Some(h) => format!("{}/block?height={}", rpc, h),
        None => format!("{}/block", rpc),
    };

    let resp = reqwest::blocking::get(&url);

    match resp {
        Ok(response) => {
            if !response.status().is_success() {
                return CheckResult {
                    protocol: "tendermint".to_string(),
                    rpc: rpc.to_string(),
                    reachable: false,
                    result: None,
                    error: Some(format!("HTTP {}", response.status())),
                };
            }

            let json: Result<serde_json::Value, _> = response.json();
            match json {
                Ok(v) => {
                    let height = v["result"]["block"]["header"]["height"]
                        .as_str()
                        .and_then(|s| s.parse::<u64>().ok());

                    CheckResult {
                        protocol: "tendermint".to_string(),
                        rpc: rpc.to_string(),
                        reachable: true,
                        result: Some(ResultData::Block { height }),
                        error: None,
                    }
                }
                Err(e) => CheckResult {
                    protocol: "tendermint".to_string(),
                    rpc: rpc.to_string(),
                    reachable: false,
                    result: None,
                    error: Some(format!("Invalid JSON: {}", e)),
                },
            }
        }
        Err(e) => CheckResult {
            protocol: "tendermint".to_string(),
            rpc: rpc.to_string(),
            reachable: false,
            result: None,
            error: Some(e.to_string()),
        },
    }
}

fn check_ethereum_status(rpc: &str) -> CheckResult {
    let client = reqwest::blocking::Client::new();

    let syncing_req = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "eth_syncing",
        "params": [],
        "id": 1
    });

    let syncing_resp = client.post(rpc).json(&syncing_req).send();
    let syncing_resp = match syncing_resp {
        Ok(r) => r,
        Err(e) => {
            return CheckResult {
                protocol: "ethereum".to_string(),
                rpc: rpc.to_string(),
                reachable: false,
                result: None,
                error: Some(e.to_string()),
            };
        }
    };

    let syncing_json: serde_json::Value = match syncing_resp.json() {
        Ok(v) => v,
        Err(e) => {
            return CheckResult {
                protocol: "ethereum".to_string(),
                rpc: rpc.to_string(),
                reachable: false,
                result: None,
                error: Some(format!("Invalid JSON: {}", e)),
            };
        }
    };

    let syncing = syncing_json["result"].is_object();

    let block_req = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "eth_blockNumber",
        "params": [],
        "id": 2
    });

    let block_resp = client.post(rpc).json(&block_req).send();
    let block_resp = match block_resp {
        Ok(r) => r,
        Err(e) => {
            return CheckResult {
                protocol: "ethereum".to_string(),
                rpc: rpc.to_string(),
                reachable: false,
                result: None,
                error: Some(e.to_string()),
            };
        }
    };

    let block_json: serde_json::Value = match block_resp.json() {
        Ok(v) => v,
        Err(e) => {
            return CheckResult {
                protocol: "ethereum".to_string(),
                rpc: rpc.to_string(),
                reachable: false,
                result: None,
                error: Some(format!("Invalid JSON: {}", e)),
            };
        }
    };

    let latest_block = block_json["result"]
        .as_str()
        .and_then(|s| u64::from_str_radix(s.trim_start_matches("0x"), 16).ok());

    CheckResult {
        protocol: "ethereum".to_string(),
        rpc: rpc.to_string(),
        reachable: true,
        result: Some(ResultData::Status {
            latest_block,
            syncing: Some(syncing),
        }),
        error: None,
    }
}

fn check_ethereum_health(rpc: &str) -> CheckResult {
    let client = reqwest::blocking::Client::new();

    let req = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "eth_chainId",
        "params": [],
        "id": 1
    });

    let resp = client.post(rpc).json(&req).send();

    match resp {
        Ok(response) => {
            if !response.status().is_success() {
                return CheckResult {
                    protocol: "ethereum".to_string(),
                    rpc: rpc.to_string(),
                    reachable: false,
                    result: Some(ResultData::Health { healthy: false }),
                    error: Some(format!("HTTP {}", response.status())),
                };
            }

            let json: Result<serde_json::Value, _> = response.json();
            match json {
                Ok(v) => {
                    let healthy = v.get("result").is_some();

                    CheckResult {
                        protocol: "ethereum".to_string(),
                        rpc: rpc.to_string(),
                        reachable: healthy,
                        result: Some(ResultData::Health { healthy }),
                        error: None,
                    }
                }
                Err(e) => CheckResult {
                    protocol: "ethereum".to_string(),
                    rpc: rpc.to_string(),
                    reachable: false,
                    result: Some(ResultData::Health { healthy: false }),
                    error: Some(format!("Invalid JSON: {}", e)),
                },
            }
        }
        Err(e) => CheckResult {
            protocol: "ethereum".to_string(),
            rpc: rpc.to_string(),
            reachable: false,
            result: Some(ResultData::Health { healthy: false }),
            error: Some(e.to_string()),
        },
    }
}

fn check_ethereum_block(rpc: &str, height: Option<u64>) -> CheckResult {
    let client = reqwest::blocking::Client::new();

    let block_param = match height {
        Some(n) => format!("0x{:x}", n),
        None => "latest".to_string(),
    };

    let req = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "eth_getBlockByNumber",
        "params": [block_param, false],
        "id": 1
    });

    let resp = client.post(rpc).json(&req).send();

    match resp {
        Ok(response) => {
            if !response.status().is_success() {
                return CheckResult {
                    protocol: "ethereum".to_string(),
                    rpc: rpc.to_string(),
                    reachable: false,
                    result: None,
                    error: Some(format!("HTTP {}", response.status())),
                };
            }

            let json: Result<serde_json::Value, _> = response.json();
            match json {
                Ok(v) => {
                    let height = v["result"]["number"]
                        .as_str()
                        .and_then(|s| u64::from_str_radix(s.trim_start_matches("0x"), 16).ok());

                    CheckResult {
                        protocol: "ethereum".to_string(),
                        rpc: rpc.to_string(),
                        reachable: height.is_some(),
                        result: Some(ResultData::Block { height }),
                        error: None,
                    }
                }
                Err(e) => CheckResult {
                    protocol: "ethereum".to_string(),
                    rpc: rpc.to_string(),
                    reachable: false,
                    result: None,
                    error: Some(format!("Invalid JSON: {}", e)),
                },
            }
        }
        Err(e) => CheckResult {
            protocol: "ethereum".to_string(),
            rpc: rpc.to_string(),
            reachable: false,
            result: None,
            error: Some(e.to_string()),
        },
    }
}

fn check_bitcoin_status(rpc: &str) -> CheckResult {
    let client = reqwest::blocking::Client::new();

    let req = serde_json::json!({
        "jsonrpc": "1.0",
        "id": "rpc-checker",
        "method": "getblockchaininfo",
        "params": []
    });

    let resp = client
        .post(rpc)
        .basic_auth("rpcuser", Some("rpcpass"))
        .json(&req)
        .send();

    match resp {
        Ok(response) => {
            if !response.status().is_success() {
                return CheckResult {
                    protocol: "bitcoin".to_string(),
                    rpc: rpc.to_string(),
                    reachable: false,
                    result: None,
                    error: Some(format!("HTTP {}", response.status())),
                };
            }

            let json: Result<serde_json::Value, _> = response.json();
            match json {
                Ok(v) => {
                    let result = &v["result"];

                    let blocks = result["blocks"].as_u64();
                    let headers = result["headers"].as_u64();
                    let syncing = match (blocks, headers) {
                        (Some(b), Some(h)) => Some(b < h),
                        _ => None,
                    };

                    CheckResult {
                        protocol: "bitcoin".to_string(),
                        rpc: rpc.to_string(),
                        reachable: true,
                        result: Some(ResultData::Status {
                            latest_block: blocks,
                            syncing,
                        }),
                        error: None,
                    }
                }
                Err(e) => CheckResult {
                    protocol: "bitcoin".to_string(),
                    rpc: rpc.to_string(),
                    reachable: false,
                    result: None,
                    error: Some(format!("Invalid JSON: {}", e)),
                },
            }
        }
        Err(e) => CheckResult {
            protocol: "bitcoin".to_string(),
            rpc: rpc.to_string(),
            reachable: false,
            result: None,
            error: Some(e.to_string()),
        },
    }
}

fn check_bitcoin_health(rpc: &str) -> CheckResult {
    let client = reqwest::blocking::Client::new();

    let req = serde_json::json!({
        "jsonrpc": "1.0",
        "id": "rpc-checker",
        "method": "getnetworkinfo",
        "params": []
    });

    let resp = client
        .post(rpc)
        .basic_auth("rpcuser", Some("rpcpass"))
        .json(&req)
        .send();

    match resp {
        Ok(response) => {
            if !response.status().is_success() {
                return CheckResult {
                    protocol: "bitcoin".to_string(),
                    rpc: rpc.to_string(),
                    reachable: false,
                    result: Some(ResultData::Health { healthy: false }),
                    error: Some(format!("HTTP {}", response.status())),
                };
            }

            let json: Result<serde_json::Value, _> = response.json();
            match json {
                Ok(v) => {
                    let healthy = v.get("result").is_some();

                    CheckResult {
                        protocol: "bitcoin".to_string(),
                        rpc: rpc.to_string(),
                        reachable: healthy,
                        result: Some(ResultData::Health { healthy }),
                        error: None,
                    }
                }
                Err(e) => CheckResult {
                    protocol: "bitcoin".to_string(),
                    rpc: rpc.to_string(),
                    reachable: false,
                    result: Some(ResultData::Health { healthy: false }),
                    error: Some(format!("Invalid JSON: {}", e)),
                },
            }
        }
        Err(e) => CheckResult {
            protocol: "bitcoin".to_string(),
            rpc: rpc.to_string(),
            reachable: false,
            result: Some(ResultData::Health { healthy: false }),
            error: Some(e.to_string()),
        },
    }
}

fn check_bitcoin_block(rpc: &str, height: Option<u64>) -> CheckResult {
    let client = reqwest::blocking::Client::new();

    let height = match height {
        Some(h) => h,
        None => {
            let info_req = serde_json::json!({
                "jsonrpc": "1.0",
                "id": "rpc-checker",
                "method": "getblockchaininfo",
                "params": []
            });

            let info_resp = client
                .post(rpc)
                .basic_auth("rpcuser", Some("rpcpass"))
                .json(&info_req)
                .send();

            let info_resp = match info_resp {
                Ok(r) if r.status().is_success() => r,
                Ok(r) => {
                    return CheckResult {
                        protocol: "bitcoin".to_string(),
                        rpc: rpc.to_string(),
                        reachable: false,
                        result: None,
                        error: Some(format!("HTTP {}", r.status())),
                    };
                }
                Err(e) => {
                    return CheckResult {
                        protocol: "bitcoin".to_string(),
                        rpc: rpc.to_string(),
                        reachable: false,
                        result: None,
                        error: Some(e.to_string()),
                    };
                }
            };

            let info_json: serde_json::Value = match info_resp.json() {
                Ok(v) => v,
                Err(e) => {
                    return CheckResult {
                        protocol: "bitcoin".to_string(),
                        rpc: rpc.to_string(),
                        reachable: false,
                        result: None,
                        error: Some(format!("Invalid JSON: {}", e)),
                    };
                }
            };

            match info_json["result"]["blocks"].as_u64() {
                Some(h) => h,
                None => {
                    return CheckResult {
                        protocol: "bitcoin".to_string(),
                        rpc: rpc.to_string(),
                        reachable: false,
                        result: None,
                        error: Some("Missing latest block height".to_string()),
                    };
                }
            }
        }
    };

    let hash_req = serde_json::json!({
        "jsonrpc": "1.0",
        "id": "rpc-checker",
        "method": "getblockhash",
        "params": [height]
    });

    let hash_resp = client
        .post(rpc)
        .basic_auth("rpcuser", Some("rpcpass"))
        .json(&hash_req)
        .send();

    let hash_resp = match hash_resp {
        Ok(r) if r.status().is_success() => r,
        Ok(r) => {
            return CheckResult {
                protocol: "bitcoin".to_string(),
                rpc: rpc.to_string(),
                reachable: false,
                result: None,
                error: Some(format!("HTTP {}", r.status())),
            };
        }
        Err(e) => {
            return CheckResult {
                protocol: "bitcoin".to_string(),
                rpc: rpc.to_string(),
                reachable: false,
                result: None,
                error: Some(e.to_string()),
            };
        }
    };

    let hash_json: serde_json::Value = match hash_resp.json() {
        Ok(v) => v,
        Err(e) => {
            return CheckResult {
                protocol: "bitcoin".to_string(),
                rpc: rpc.to_string(),
                reachable: false,
                result: None,
                error: Some(format!("Invalid JSON: {}", e)),
            };
        }
    };

    let block_hash = match hash_json["result"].as_str() {
        Some(h) => h.to_string(),
        None => {
            return CheckResult {
                protocol: "bitcoin".to_string(),
                rpc: rpc.to_string(),
                reachable: false,
                result: None,
                error: Some("Missing block hash".to_string()),
            };
        }
    };

    let header_req = serde_json::json!({
        "jsonrpc": "1.0",
        "id": "rpc-checker",
        "method": "getblockheader",
        "params": [block_hash]
    });

    let header_resp = client
        .post(rpc)
        .basic_auth("rpcuser", Some("rpcpass"))
        .json(&header_req)
        .send();

    match header_resp {
        Ok(response) => {
            if !response.status().is_success() {
                return CheckResult {
                    protocol: "bitcoin".to_string(),
                    rpc: rpc.to_string(),
                    reachable: false,
                    result: None,
                    error: Some(format!("HTTP {}", response.status())),
                };
            }

            let json: Result<serde_json::Value, _> = response.json();
            match json {
                Ok(v) => {
                    let height = v["result"]["height"].as_u64();

                    CheckResult {
                        protocol: "bitcoin".to_string(),
                        rpc: rpc.to_string(),
                        reachable: height.is_some(),
                        result: Some(ResultData::Block { height }),
                        error: None,
                    }
                }
                Err(e) => CheckResult {
                    protocol: "bitcoin".to_string(),
                    rpc: rpc.to_string(),
                    reachable: false,
                    result: None,
                    error: Some(format!("Invalid JSON: {}", e)),
                },
            }
        }
        Err(e) => CheckResult {
            protocol: "bitcoin".to_string(),
            rpc: rpc.to_string(),
            reachable: false,
            result: None,
            error: Some(e.to_string()),
        },
    }
}
