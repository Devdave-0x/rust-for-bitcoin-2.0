use clap::Parser;
use decodetrx::decode_transaction;

/// Decode a raw Bitcoin transaction from its hex representation
#[derive(Parser)]
#[command(
    name = "decodetrx",
    version = "1.0",
    about = "Bitcoin transaction decoder"
)]
struct Cli {
    /// Raw transaction hex string
    #[arg(short, long, help = "Hex-encoded raw Bitcoin transaction")]
    tx: String,
}

fn main() {
    let cli = Cli::parse();

    match decode_transaction(&cli.tx) {
        Ok(json) => println!("{}", json),
        Err(e) => {
            eprintln!("Error decoding transaction: {}", e);
            std::process::exit(1);
        }
    }
}
