use clap::Parser;
use reticulum_packet_rs::parse_packet;
use std::io::{self, BufRead, BufReader};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(long, short)]
    verbose: bool,

    #[arg(long, default_value_t = 0)]
    ifac_size: usize,

    #[arg(long, default_value = "")]
    ifac: String,

    #[arg(long)]
    announce: bool,

    #[arg(long, short, default_value = "")]
    filter: String,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    let stdin = io::stdin();
    let mut addr_filter: [u8; 16] = [0; 16];
    if !cli.filter.is_empty() {
        hex::decode_to_slice(cli.filter, &mut addr_filter).expect("Invalid Reticulum address. Expected 16 bytes as hex characters.");
    }
    for packet in BufReader::new(stdin).lines() {
        let raw_packet = packet.unwrap();
        let raw = hex::decode(raw_packet).unwrap_or("00".into());
        let a = parse_packet(raw, (&cli.ifac, &cli.ifac_size));
        match a {
            Ok(p) => {
                if addr_filter != [0; 16] {
                    if p.is_hop() {
                        if p.address() != addr_filter && p.hop_address() != addr_filter {
                            continue;
                        }
                    } else if p.address() != addr_filter {
                        continue;
                    }
                }
                eprintln!("{}", &p.debug_packet(cli.verbose));
                if cli.announce {
                    match &p.parse_announce() {
                        Ok(p) => eprintln!("{:?}", p.debug_announcement()),
                        Err(_) => continue,
                    };
                }
            }
            Err(_) => continue,
        }
    }
    Ok(())
}
