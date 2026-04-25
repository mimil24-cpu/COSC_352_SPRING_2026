use csvprof::{stream_csv};
use anyhow::{Context, Result};
use clap::Parser;

#[derive(Parser)]
#[command(name = "csvprof")]
#[command(about = "CSV Data Profiler - COSC 352 Project 8")]
struct Cli {
    file: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let cols = stream_csv(&cli.file).with_context(|| format!("Failed to read '{}'", cli.file))?;
    println!("=== csvprof: {} ===", cli.file);
    println!("Columns: {}
", cols.len());
    for (name, analyzer) in &cols {
        analyzer.report(&name);
    }
    Ok(())
}
