use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs;
use std::io::BufReader;
use std::path::PathBuf;
use directories::ProjectDirs;
use crate::core::state::State;

pub fn read_state() -> Result<State, Box<dyn Error>> {
	let json_path = state_file_path()?;
	let state = if json_path.is_file() {
		let reader = BufReader::new(fs::File::open(json_path)?);
		let state = serde_json::from_reader(reader)?;
		state
	} else {
		State::default()
	};
	Ok(state)
}

pub fn write_state(state: &State) -> Result<(), Box<dyn Error>> {
	let state_file_path = state_file_path()?;
	let json = serde_json::to_string_pretty(state)?;
	fs::write(state_file_path, json)?;
	Ok(())
}

fn state_file_path() -> Result<PathBuf, Box<dyn Error>> {
	let data_path = data_path()?;
	let state_file_path = data_path.join("STATE");
	Ok(state_file_path)
}

fn project_dirs() -> Result<ProjectDirs, Box<dyn Error>> {
	let project_dirs = ProjectDirs::from("com", "acme", "AssetProfiles")
		.ok_or(Box::new(StorageError::ProjectDirs))?;
	Ok(project_dirs)
}

fn data_path() -> Result<PathBuf, Box<dyn Error>> {
	let project_dirs = project_dirs()?;
	let data_dir = project_dirs.data_dir();
	fs::create_dir_all(data_dir)?;
	Ok(data_dir.to_path_buf())
}


#[derive(Debug)]
pub enum StorageError {
	ProjectDirs
}

impl Display for StorageError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:?}", self)
	}
}

impl Error for StorageError {}
