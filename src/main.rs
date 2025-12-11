use clap::{Parser, Subcommand};
use compressor::{compress, freq};
use std::collections::HashMap;
use std::fs;

mod compressor;

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
    // Compression command
    #[command(about = "Compress a file. wip", name = "compress")]
    CompressCommand {
        #[arg(help = "Input file to compress")]
        input: String,
        #[arg(help = "Output compressed file")]
        output: String,
    },
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    match &args.command {
        Command::FreqCommand { input } => {
            println!("Performing frequency analysis on file: {}", input);
            let data: Vec<u8> = fs::read(input)?;
            let freq: HashMap<u8, u32> = freq::freq_analysis(data);
            println!("{:?}", freq);
        }
        Command::CompressCommand { input, output } => {
            println!("Compressing file: {} to {}", input, output);
            let tree = compress::compress(fs::read(input)?);
            match tree {
                Some(t) => {
                    println!("Huffman codes: {:?}", t);
                    println!("Writing to file: {}", output);
                    fs::write(output, t)?;
                },
                None => {
                    println!("No data to compress")
                },
            }
        }
    }
    Ok(())
}
