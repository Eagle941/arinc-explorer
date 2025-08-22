use std::path::PathBuf;
use std::process;

use anyhow::anyhow;
use arinc_explorer::files::FilesLum;
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
    let path = args.filename;
    let filename = path
        .file_name()
        .ok_or(anyhow!("Valid filename from {path:?} not found."))?;

    match filename.to_str() {
        Some("LOADS.LUM") => {
            let loads_lum = LoadsLum::new(&path)?;
            println!("{loads_lum}");
        }
        Some("FILES.LUM") => {
            let files_lum = FilesLum::new(&path)?;
            println!("{files_lum}");
        }
        Some(file_name) => return Err(anyhow!("{file_name} not supported.")),
        None => return Err(anyhow!("Filename not supported.")),
    }

    Ok(())
}
