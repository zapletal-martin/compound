use serde::Deserialize;

#[derive(Deserialize)]
pub struct Investment {

    #[serde(alias = "Fund name")]
    pub fund_name: String,

    #[serde(alias = "Symbol")]
    pub symbol: String,

    #[serde(alias = "YTD as of 01/12/2024")]
    pub ytd: f64,

    #[serde(alias = "1 year Average annual total returns as of 12/31/2023")]
    pub one_year: f64,

    #[serde(alias = "3 year Average annual total returns as of 12/31/2023")]
    pub three_year: f64,

    #[serde(alias = "5 year Average annual total returns as of 12/31/2023")]
    pub five_year: f64,

    #[serde(alias = "10 year Average annual total returns as of 12/31/2023")]
    pub ten_year: f64,

    #[serde(alias = "Since inception Average annual total returns as of 12/31/2023")]
    pub since_inception: f64,

    #[serde(alias = "Expense ratio")]
    pub expense_ratio: f64,

    #[serde(alias = "SEC yield")]
    pub sec_yield: String,

    #[serde(alias = "Distribution")]
    pub distribution: f64,

    #[serde(alias = "Dividend")]
    pub dividend: f64,

    #[serde(alias = "Benchmark")]
    pub benchmark: String
}
