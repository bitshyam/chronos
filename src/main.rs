mod converter;
mod parser;

use clap::Parser;
use std::process;

#[derive(Parser)]
#[command(name = "chronos")]
#[command(about = "A CLI tool for converting between ISO timestamps, epoch timestamps, and local time")]
#[command(version = "0.1.0")]
struct Cli {
    /// Timestamp to convert (epoch number or ISO string). If no argument provided, shows current time.
    timestamp: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    match cli.timestamp {
        Some(timestamp) => {
            if let Err(e) = converter::auto_convert(&timestamp) {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        }
        None => {
            // No argument provided, show current time
            let (iso, epoch, local) = converter::current_time();
            
            println!("Current time:");
            println!("ISO: {}", iso);
            println!("Epoch: {}", epoch);
            println!("Local: {}", local);
        }
    }
}
