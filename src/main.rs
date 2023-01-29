use clap::Parser;
use reticulum_packet_rs::parse_packet;
use std::io::{self, BufRead, BufReader};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(long, short)]
    verbose: bool,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    let stdin = io::stdin();
    for packet in BufReader::new(stdin).split(0x7e) {
        let packet = packet.unwrap();
        if packet.len() < 36 {
            continue;
        }
        let a = parse_packet(packet);
        a.unwrap().debug_packet(cli.verbose);
    }
    Ok(())
}
