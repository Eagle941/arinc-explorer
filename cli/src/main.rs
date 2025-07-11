use std::path::PathBuf;
use std::process;

use arinc_explorer::loads::LoadsLum;
use clap::Parser;
use exitcode::{OK, SOFTWARE};

#[derive(Clone, Parser, Debug)]
pub struct Args {
    #[arg(long)]
    pub filename: PathBuf,
}

fn main() {
    let args = Args::parse();

    match run(args) {
        Ok(()) => process::exit(OK),
        Err(e) => {
            eprintln!("Internal software error: {e}");
            process::exit(SOFTWARE);
        }
    }
}

fn run(args: Args) -> anyhow::Result<()> {
    let filename = args.filename;

    let loads_lum = LoadsLum::new(&filename)?;
    println!("{loads_lum}");

    Ok(())
}
