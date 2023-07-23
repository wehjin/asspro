use std::fmt::{Display, Formatter};
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

impl Display for AssetProfile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AssetProfile::UsStock { company_symbol, company_name, .. } => {
                write!(f, "{}:{}", company_symbol, company_name)
            }
        }
    }
}