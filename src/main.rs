use std::fs;
use csv::Error;
use std::io::{Error as IoError, ErrorKind as IoErrorKind};

mod raw_investment;
mod investment;
mod potential;
mod compound_investment;

fn parse_percentage(perc: &String) -> Result<f64, IoError> {
    perc
        .split("%")
        .next()
        // TODO: This should clearly return an option
        .map(|s| s.parse::<f64>().unwrap_or_default())
        .ok_or(IoError::new(IoErrorKind::Other, "Percentage parsing failed"))
}

fn main() -> Result<(), Error> {

    let years: i32 = 10;
    let principal: f64 = 100000.0;
    // let interest: fn(&investment::Investment) -> f64 = | i | i.three_year;
    let interest: fn(&investment::Investment) -> f64 =
    |   i | (i.one_year + i.three_year + i.five_year + i.ten_year) / 4.0;  

    let csv_text = fs::read_to_string("./All_Investments_view.csv")?;

    let bytes = csv_text.as_bytes();
    let mut reader = csv::Reader::from_reader(bytes);
    let records_err: Result<Vec<raw_investment::RawInvestment>, csv::Error> = reader.deserialize().collect();
    let records = records_err?;

    let investments_err: Result<Vec<compound_investment::CompoundInvestment>, csv::Error> =
        records.iter().map(
            |ri: &raw_investment::RawInvestment| -> Result<compound_investment::CompoundInvestment, csv::Error> {
                let inv = investment::Investment {
                    fund_name: ri.fund_name.clone(),
                    symbol: ri.symbol.clone(),
                    ytd: parse_percentage(&ri.ytd)?,
                    one_year: parse_percentage(&ri.one_year)?,
                    three_year: parse_percentage(&ri.three_year)?,
                    five_year: parse_percentage(&ri.five_year)?,
                    ten_year: parse_percentage(&ri.ten_year)?,
                    since_inception: parse_percentage(&ri.since_inception)?,
                    expense_ratio: parse_percentage(&ri.expense_ratio)?,
                    sec_yield: ri.sec_yield.clone(),
                    dividend: parse_percentage(&ri.dividend)?,
                    distribution: parse_percentage(&ri.distribution)?,
                    benchmark: ri.benchmark.clone()
                };

                let comp_inv = compound_investment::CompoundInvestment {
                    fund_name: ri.fund_name.clone(),
                    symbol: ri.symbol.clone(),
                    years: years,
                    principal_start: principal,
                    principal_end: potential::compounding(
                        principal,
                        inv.dividend,
                        interest(&inv),
                        inv.expense_ratio,
                        years,
                        1)
                };

        Ok(comp_inv)
    }).collect();

    let mut investments = investments_err?;
    investments.sort_by(
        |a, b| b.principal_end.total_cmp(&a.principal_end));

    for inv in &investments[0..=50] {
        println!(
            "Fund {}, symbol {},  with total {}.",
            inv.fund_name,
            inv.symbol,
            inv.principal_end
        );
    }

    Ok(())
}
