mod rpc;
mod types;

use clap::Parser;
use rpc::check;
use types::{BitcoinMethod, Command, EthereumMethod, Protocol, TendermintMethod};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(long)]
    protocol: String,

    #[arg(long)]
    method: String,

    #[arg(long)]
    rpc: String,
}

fn parse_protocol(p: &str) -> Protocol {
    match p {
        "tendermint" => Protocol::Tendermint,
        "ethereum" => Protocol::Ethereum,
        "bitcoin" => Protocol::Bitcoin,
        _ => {
            eprintln!("Unsupported protocol: {}", p);
            std::process::exit(1);
        }
    }
}

fn main() {
    let args = Args::parse();

    let _protocol = parse_protocol(&args.protocol);

    // let protocol = parse_protocol(&args.protocol);

    let cmd = match (args.protocol.as_str(), args.method.as_str()) {
        ("tendermint", "status") => Command::Tendermint(TendermintMethod::Status),
        ("tendermint", "health") => Command::Tendermint(TendermintMethod::Health),
        ("tendermint", "block") => Command::Tendermint(TendermintMethod::Block { height: None }),

        ("ethereum", "status") => Command::Ethereum(EthereumMethod::Status),
        ("ethereum", "health") => Command::Ethereum(EthereumMethod::Health),
        ("ethereum", "block") => Command::Ethereum(EthereumMethod::Block { height: None }),

        ("bitcoin", "status") => Command::Bitcoin(BitcoinMethod::Status),
        ("bitcoin", "health") => Command::Bitcoin(BitcoinMethod::Health),
        ("bitcoin", "block") => Command::Bitcoin(BitcoinMethod::Block { height: None }),

        _ => {
            eprintln!(
                "Unsupported combination: protocol={} method={}",
                args.protocol, args.method
            );
            std::process::exit(1);
        }
    };

    let result = check(cmd, &args.rpc);

    println!("{}", serde_json::to_string_pretty(&result).unwrap());
}
