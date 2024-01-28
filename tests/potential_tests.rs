
#[cfg(test)]
mod potential_tests {
    
    #[test]
    fn test_compounding_single_year_yes_int_no_div_no_exp() {
        assert_eq!(
            compound::potential::compounding(100.0, 0.0, 10.0, 0.0, 1, 1).floor(), 110.0
        );
    }

    #[test]
    fn test_compounding_single_year_no_int_yes_div_no_exp() {
        assert_eq!(
            compound::potential::compounding(100.0, 10.0, 0.0, 0.0, 1, 1).floor(), 110.0
        );
    }

    #[test]
    fn test_compounding_single_year_no_int_no_div_yes_exp() {
        assert_eq!(
            compound::potential::compounding(100.0, 0.0, 0.0, 10.0, 1, 1).floor(), 90.0
        );
    }

    #[test]
    fn test_compounding_two_years_yes_int_no_div_no_exp() {
        assert_eq!(
            compound::potential::compounding(100.0, 0.0, 10.0, 0.0, 2, 1).floor(), 121.0
        );
    }

    #[test]
    fn test_compounding_two_years_no_int_yes_div_no_exp() {
        assert_eq!(
            compound::potential::compounding(100.0, 10.0, 0.0, 0.0, 2, 1).floor(), 121.0
        );
    }

    #[test]
    fn test_compounding_two_years_no_int_no_div_yes_exp() {
        assert_eq!(
            compound::potential::compounding(100.0, 0.0, 0.0, 10.0, 2, 1).floor(), 81.0
        );
    }

    #[test]
    fn test_compounding_three_years_yes_int_yes_div_yes_exp() {
        let principal = 100.0;
        let interest = 10.0;
        let dividend = 2.0;
        let expense = 0.3;

        // These numbers do not include the partial dividends bus close enough :)
        let first_year = (111.7f64).floor();
        let second_year = (124.07f64).floor();
        let third_year = (139.33f64).floor();

        assert_eq!(
            compound::potential::compounding(principal, dividend, interest, expense, 1, 1).floor(), first_year
        );

        assert_eq!(
            compound::potential::compounding(principal, dividend, interest, expense, 2, 1).floor(), second_year
        );

        assert_eq!(
            compound::potential::compounding(principal, dividend, interest, expense, 3, 1).floor(), third_year
        );
    }
}   