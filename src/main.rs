use std::collections::HashMap;
use std::fs;
use clap::{Parser, Subcommand};
use huffman::freq;

mod huffman;

// Main CLI structure
#[derive(Parser)]
#[command(about = "A simple file compressor, wip, only does frequency analysis for now")]
struct Cli {
    // Subcommand to execute
    #[clap(subcommand)]
    command: Command,
}

// Subcommand enum
#[derive(Subcommand, Debug)]
enum Command {
    // Frequency analysis command
    #[command(about = "Perform frequency analysis on a file", name = "freq")]
    FreqCommand {
        #[arg(help = "Input file to analyze")]
        input: String,
    },
    #[command(about = "Compress a file. wip", name = "compress")]
    CompressCommand {
        #[arg(help = "Input file to compress")]
        input: String,
        #[arg(help = "Output compressed file")]
        output: String,
    }
}

fn main() -> anyhow::Result<()> {
    let args= Cli::parse();
    match &args.command {
        Command::FreqCommand { input } => {
            println!("Performing frequency analysis on file: {}", input);
            let data: Vec<u8> = fs::read(input)?;
            let freq: HashMap<u8, u32> = freq::freq_analysis(data);
            println!("{:?}", freq);
        }
        Command::CompressCommand { input, output } => {
            println!("Compressing file: {} to {}", input, output);
        }
    }
    Ok(())
}
