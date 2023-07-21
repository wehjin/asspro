use std::error::Error;
use std::path::{Path, PathBuf};
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
	#[command(subcommand)]
	command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
	Import {
		#[clap(short, long)]
		path: PathBuf
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	let cli = Cli::parse();
	match &cli.command {
		Commands::Import { path } => {
			import(path)
		}
	}
}

fn import(path: &Path) -> Result<(), Box<dyn Error>> {
	println!("Importing {}", path.to_str().expect("Path to_str"));

	#[derive(Debug, serde::Deserialize)]
	struct Record {
		rank: u64,
		us_symbol: String,
		company_name: String,
		usd_market_cap: String,
		usd_stock_price: String,
		usd_revenue: String,
	}

	let mut rdr = csv::Reader::from_path(path.clone())?;
	for result in rdr.deserialize() {
		let record: Record = result?;
		println!("{}: {:?}", record.rank, record);
	}
	Ok(())
}