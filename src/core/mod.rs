use serde::{Deserialize, Serialize};
pub use asset_profile::*;

mod asset_profile;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum EventData {
    Add { profiles: Vec<AssetProfile> }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Event {
    Add { hash: String }
}

pub mod state;

pub mod storage;

