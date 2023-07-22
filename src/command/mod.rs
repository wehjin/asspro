use std::error::Error;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use clap::Subcommand;
use ipfs_api_backend_hyper::{IpfsApi, IpfsClient};
use ipfs_api_backend_hyper::response::AddResponse;
use crate::core::{Event, EventData, storage};

mod csv_imports;

#[derive(Subcommand, Debug)]
pub enum Commands {
	Import { csv_path: PathBuf },
	State,
}

impl Commands {
	pub async fn run(&self) -> Result<(), Box<dyn Error>> {
		match self {
			Commands::Import { csv_path } => import(csv_path).await,
			Commands::State => {
				let state = storage::read_state()?;
				println!("{:?}", state);
				Ok(())
			}
		}
	}
}

async fn import(path: &Path) -> Result<(), Box<dyn Error>> {
	println!("Importing {}", path.to_str().expect("Path to_str"));
	let profiles = csv_imports::read_profiles(path)?;
	let json = serde_json::to_string_pretty(&EventData::Add { profiles })?;
	let ipfs = add_ipfs(json).await?;
	let state = storage::read_state()?
		.add_pin(&ipfs.hash)
		.set_head(Event::Add { hash: ipfs.hash });
	storage::write_state(&state)?;
	println!("Imported");
	Ok(())
}

async fn add_ipfs(content: String) -> Result<AddResponse, Box<dyn Error>> {
	let client = IpfsClient::default();
	let data = Cursor::new(content);
	let res = client.add(data).await?;
	Ok(res)
}

