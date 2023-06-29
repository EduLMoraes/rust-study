#[cfg(test)]
mod tests {

    #[test]
    fn test_rate() {
        let value = "100".to_string();
        let dividend = "10".to_string();

        let expected_rate = 9.0;

        assert_eq!(simulate_input_rate(&value, &dividend), expected_rate);
    }

    #[test]
    fn test_profit() {
        let dividend = "10".to_string();
        let time = "5".to_string();

        let expected_profit = 50.0;

        assert_eq!(simulate_input_profit(&dividend, &time), expected_profit);
    }

    #[test]
    fn test_value_finish() {
        let value = "100".to_string();
        let dividend = "10".to_string();
        let time = "5".to_string();

        let expected_value_finish = 150.0;

        assert_eq!(simulate_input_value_finish(&value, &dividend, &time), expected_value_finish);
    }

    #[test]
    fn test_dividend() {
        let value = "100".to_string();
        let div_yield = "5".to_string();

        let expected_dividend = 0.5;

        assert_eq!(simulate_input_dividend(&value, &div_yield), expected_dividend);
    }

    fn simulate_input_rate(value: &str, dividend: &str) -> f64 {
        let value_input: f64 = value.parse().unwrap();
        let dividend_input: f64 = dividend.parse().unwrap();

        ((dividend_input * 100.0) / value_input) - 1.0
    }

    fn simulate_input_profit(dividend: &str, time: &str) -> f64 {
        let dividend_input: f64 = dividend.parse().unwrap();
        let time_input: f64 = time.parse().unwrap();

        dividend_input * time_input
    }

    fn simulate_input_value_finish(value: &str, dividend: &str, time: &str) -> f64 {
        let value_input: f64 = value.parse().unwrap();
        let dividend_input: f64 = dividend.parse().unwrap();
        let time_input: f64 = time.parse().unwrap();

        (dividend_input * time_input) + value_input
    }

    fn simulate_input_dividend(value: &str, div_yield: &str) -> f64 {
        let value_input: f64 = value.parse().unwrap();
        let div_yield_input: f64 = div_yield.parse().unwrap();

        ((div_yield_input / 100.0) * value_input) / 10.0
    }
}
