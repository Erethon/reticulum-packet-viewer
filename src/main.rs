use clap::Parser;
use reticulum_packet_rs::parse_packet;
use std::io::{self, BufRead, BufReader};
use hex;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(long, short)]
    verbose: bool,

    #[arg(long, short)]
    tcp: bool,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    let stdin = io::stdin();
    if cli.tcp {
        for packet in BufReader::new(stdin).lines() {
            let raw_packet = packet.unwrap();
            if raw_packet.len() > 35 {
                let raw = hex::decode(raw_packet).unwrap_or("00".into());
                if raw.len() > 35 {
                    let a = parse_packet(raw);
                    a.unwrap().debug_packet(cli.verbose);
                }
            }
        }
    }
    Ok(())
}
