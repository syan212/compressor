use std::collections::HashMap;
use std::fs;
use clap::{Parser, Subcommand};

mod freq;

#[derive(Parser)]
#[command(about = "A simple file compressor, wip, only does frequency analysis for now")]
pub struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    #[command(about = "Perform frequency analysis on a file", name = "freq")]
    FreqCommand {
        #[arg(help = "Input file to analyze")]
        input: String,
    }
}

fn main() -> anyhow::Result<()> {
    let args= Cli::parse();
    match &args.command {
        Command::FreqCommand { input } => {
            println!("Performing frequency analysis on file: {}", input);
            let data: Vec<u8> = fs::read(input)?;
            let freq: HashMap<u8, u128> = freq::freq_analysis(data);
            println!("{:?}", freq);
        }
    }
    Ok(())
}
