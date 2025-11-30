use std::collections::HashMap;
use std::fs;
use clap::Parser;

mod freq;

#[derive(Parser)]
#[command(about = "A simple file compressor, wip, only does frequency analysis for now")]
struct Cli {
    /// The input file to analyze
    input: String,
}

fn main() -> anyhow::Result<()> {
    let args= Cli::parse();
    let data = fs::read(args.input)?;
    let freq_count: HashMap<u8, u128> = freq::freq_analysis(data);
    println!("{:?}", freq_count);
    Ok(())
}
