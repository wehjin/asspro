use std::error::Error;
use std::path::Path;
use crate::core::AssetProfile;

#[derive(Debug, serde::Deserialize)]
struct Record {
	us_symbol: String,
	company_name: String,
	usd_market_cap: String,
}

impl From<&Record> for AssetProfile {
	fn from(value: &Record) -> Self {
		AssetProfile::UsStock {
			company_symbol: value.us_symbol.clone(),
			company_name: value.company_name.clone(),
			usd_market_cap: value.usd_market_cap.clone(),
		}
	}
}

pub fn read_profiles(path: &Path) -> Result<Vec<AssetProfile>, Box<dyn Error>> {
	let mut profiles = Vec::new();

	let mut rdr = csv::Reader::from_path(path.clone())?;
	for result in rdr.deserialize() {
		let record: Record = result?;
		let profile = AssetProfile::from(&record);
		profiles.push(profile);
	}
	Ok(profiles)
}
