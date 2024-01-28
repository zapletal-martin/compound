pub fn compounding(
    principal: f64,
    dividend_yield: f64,
    int: f64,
    expense_ratio: f64,
    years: i32,
    compound_per_period: i32) -> f64 {

    if years <= 0 {
        return principal;
    } else {
        let new_principal =
            interest(principal, int, 1, compound_per_period);

        let partial_dividend =
            | portion: f64 |
                ((principal + ((new_principal - principal) * portion))
                * (dividend_yield / 100.0))
                / 4.0;

        // Dividends are paid quarterly, this assumes linear interest throughout the year
        let div = 
            partial_dividend(1.0/4.0) +
            partial_dividend(1.0/2.0) +
            partial_dividend(3.0/4.0) +
            partial_dividend(1.0);

        // TODO: I assume expense ratio should be calculated similarly to the dividend
        let exp = new_principal * expense_ratio / 100.0;

        return compounding(
            new_principal + div - exp,
            dividend_yield,
            int,
            expense_ratio,
            years - 1,
            compound_per_period);
    }
}


fn interest(principal: f64, interest: f64, years: i32, compound_per_period: i32) -> f64 {
    principal * (1.0 + interest / 100.0 / compound_per_period as f64).powi(compound_per_period * years)
}