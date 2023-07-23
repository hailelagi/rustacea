use clap::Parser;
use std::{fs, error};
use anyhow::{Context, Result};

#[derive(Debug, Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let content = fs::read_to_string(&args.path)?;

    for line in content.lines(){
        if line.contains(&args.pattern) {
            println!("{}", line)
        }
    };

    Ok(())
}
