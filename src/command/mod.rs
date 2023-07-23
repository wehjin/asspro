use std::error::Error;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use clap::Subcommand;
use ipfs_api_backend_hyper::{IpfsApi, IpfsClient};
use ipfs_api_backend_hyper::response::AddResponse;
use crate::core::{Event, EventData, storage};
use futures::TryStreamExt;

mod csv_imports;

#[derive(Subcommand, Debug)]
pub enum Commands {
    Import { csv_path: PathBuf },
    State,
    Profiles {
        #[clap(short, long)]
        max_count: Option<usize>
    },
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
            Commands::Profiles { max_count: n } => {
                eprintln!("Profiles");
                let state = storage::read_state()?;
                let mut profiles = if let Some(head) = &state.head {
                    match head {
                        Event::Add { hash } => {
                            let string = get_ipfs(hash).await?;
                            let event = serde_json::from_str::<EventData>(&string)?;
                            match event {
                                EventData::Add { profiles } => profiles,
                            }
                        }
                    }
                } else {
                    Vec::new()
                };
                if let Some(n) = n.clone() {
                    profiles.truncate(n);
                }
                for item in profiles {
                    println!("{:?}", item);
                }
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

async fn get_ipfs<T: AsRef<str>>(hash: T) -> Result<String, Box<dyn Error>> {
    let client = IpfsClient::default();
    let bytes = client.cat(hash.as_ref())
        .map_ok(|chunk| chunk.to_vec())
        .try_concat().await?;
    let string = String::from_utf8(bytes)?;
    Ok(string)
}

