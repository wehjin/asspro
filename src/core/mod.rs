use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AssetProfile {
	UsStock {
		company_symbol: String,
		company_name: String,
		usd_market_cap: String,
	}
}

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

