use std::error::Error;
use std::path::{PathBuf};
use clap::{Parser, Subcommand};

mod core;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
	#[command(subcommand)]
	command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
	Import {
		csv_path: PathBuf
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	let cli = Cli::parse();
	match &cli.command {
		Commands::Import { csv_path: path } => {
			command::import(path)
		}
	}
}

mod command;

