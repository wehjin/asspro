use std::error::Error;
use clap::{Parser};
use crate::command::Commands;

mod core;
mod command;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
	#[command(subcommand)]
	command: Commands,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let cli = Cli::parse();
	cli.command.run().await
}


