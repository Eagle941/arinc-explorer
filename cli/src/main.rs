use std::path::PathBuf;

use clap::Parser;

#[derive(Clone, Parser, Debug)]
pub struct Args {
    #[arg(long)]
    pub filename: PathBuf,
}

fn main() {
    println!("Hello, world!");
}
