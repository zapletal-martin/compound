use serde::Deserialize;

#[derive(Deserialize)]
pub struct RawInvestment {

    #[serde(alias = "Fund name")]
    pub fund_name: String,

    #[serde(alias = "Symbol")]
    pub symbol: String,

    #[serde(alias = "YTD as of 01/12/2024")]
    pub ytd: String,

    #[serde(alias = "1 year Average annual total returns as of 12/31/2023")]
    pub one_year: String,

    #[serde(alias = "3 year Average annual total returns as of 12/31/2023")]
    pub three_year: String,

    #[serde(alias = "5 year Average annual total returns as of 12/31/2023")]
    pub five_year: String,

    #[serde(alias = "10 year Average annual total returns as of 12/31/2023")]
    pub ten_year: String,

    #[serde(alias = "Since inception Average annual total returns as of 12/31/2023")]
    pub since_inception: String,

    #[serde(alias = "Expense ratio")]
    pub expense_ratio: String,

    #[serde(alias = "SEC yield")]
    pub sec_yield: String,

    #[serde(alias = "Distribution")]
    pub distribution: String,

    #[serde(alias = "Dividend")]
    pub dividend: String,

    #[serde(alias = "Benchmark")]
    pub benchmark: String
}
