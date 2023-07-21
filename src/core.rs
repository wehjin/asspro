#[derive(Debug)]
pub enum AssetProfile {
	UsStock {
		company_symbol: String,
		company_name: String,
		usd_market_cap: String,
	}
}
