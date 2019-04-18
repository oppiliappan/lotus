use std::default::Default;

#[cfg(test)]
mod tests {

    use crate::Lotus;

    #[test]
    fn it_formats_some_floats() {
        let dollar = Lotus::new("$", 2);
        assert_eq!("$ 5,000.00", dollar.format(5000.0));
    }
    #[test]
    fn it_formats_some_floats_with_some_extras() {
        let pound = Lotus::new("GBP", 4);
        assert_eq!("GBP 5,000.2500", pound.format(5000.25));
    }
    #[test]
    fn it_formats_negative_floats() {
        let pound = Lotus::new("$", 3);
        assert_eq!("$ (5,000.000)", pound.format(-5000.0));
    }
    #[test]
    fn it_formats_a_zero_valued_float() {
        let pound = Lotus::new("$", 2);
        assert_eq!("$ --", pound.format(0.00));
    }
}

#[derive(Debug)]
pub struct Lotus<'a> {
    symbol: &'a str,
    precision: u8,
    thousand_str: &'a str,
    decimal_str: &'a str,
    format: &'a str,
    format_negative: &'a str,
    format_zero: &'a str
}

impl<'a> Lotus<'a> {
    pub fn new(symbol: &str, precision: u8) -> Lotus {
        Lotus {
            symbol,
            precision,
            ..Default::default()
        }
    }

    pub fn format(&self, number: f64) -> String {

        if number == 0. {
            let value = format!("{:.*}", self.precision as usize, number);
            let currencied = self.format_zero.replace("%v", value.as_str());
            return currencied.replace("%s", self.symbol)
        } else {
            let value = format!("{:.*}", self.precision as usize, number.abs());
            let mut float_iter = value.split(".");
            let integral   = float_iter.next().unwrap();
            let fractional = float_iter.next().unwrap();

            let mut formatted_integral = String::new();

        for ( i, letter ) in integral.chars().rev().enumerate() {
            if i % 3 == 0 && i != 0 && letter != '-' {
                formatted_integral.push_str(&self.thousand_str[..]);
            }
            formatted_integral.push(letter);
        }
        let formatted_integral = formatted_integral.chars().rev().collect::<String>();
        let formatted_float = format!("{}{}{}", formatted_integral, self.decimal_str, fractional);

        let mut currencied = self.format.replace("%s", &self.symbol[..]);
        currencied = currencied.replace("%v", &formatted_float[..]);
        currencied
    }
}

impl<'a> Default for Lotus<'a> {
    fn default() -> Self {
        Lotus {
            symbol: "$".into(),
            precision: 2,
            thousand_str: ",",
            decimal_str: ".",
            format: "%s %v",
            format_negative: "%s (%v)",
            format_zero: "%s --",
        }
    }
}
