# rpc-checker

`rpc-checker` is a lightweight, protocol-aware CLI for inspecting blockchain RPC
endpoints. It provides a unified way to query **status**, **health**, and
**block information** across multiple blockchain protocols.

This is the Rust implementation, focused on correctness, explicitness, and
easy integration into automation and monitoring workflows.

---

## Supported protocols

- **Tendermint / Cosmos SDK**
- **Ethereum (JSON-RPC)**
- **Bitcoin Core (JSON-RPC)**

---

## What this tool is (and isn’t)

**rpc-checker is:**
- a small, explicit inspection tool
- protocol-aware but CLI-consistent
- designed for monitoring, scripting, and validation

**rpc-checker is not:**
- a full client or SDK
- an indexer
- a replacement for node-specific tooling

---

## Installation

Clone the repository and build with Cargo:

```bash
cargo build --release
```

## Usage

Syntax

```bash
./target/release/rpc-checker  --protocol <protocol> --method <method> --rpc <rpc-url>
```

Example

```bash
./target/release/rpc-checker   --protocol tendermint   --method status   --rpc https://rpc.cosmos.directory/cosmoshu
b
{
  "protocol": "tendermint",
  "rpc": "https://rpc.cosmos.directory/cosmoshub",
  "reachable": true,
  "result": {
    "type": "status",
    "latest_block": 28979229,
    "syncing": false
  },
  "error": null
}

./target/release/rpc-checker   --protocol ethereum   --method health   --rpc https://eth.llamarpc.com
{
  "protocol": "ethereum",
  "rpc": "https://eth.llamarpc.com",
  "reachable": true,
  "result": {
    "type": "health",
    "healthy": true
  },
  "error": null
}

./target/debug/rpc-checker   --protocol bitcoin   --method block   --rpc http://127.0.0.1:8332
{
  "protocol": "bitcoin",
  "rpc": "http://127.0.0.1:8332",
  "reachable": true,
  "result": {
    "type": "block",
    "height": 495553
  },
  "error": null
}
```