use std::default::Default;
use std::fmt::Display;

#[macro_use]
extern crate derive_builder;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_formats_some_floats() {
        let dollar = Lotus::new("$", 2);
        assert_eq!("$ 200.00", dollar.format(200));
    }
    #[test]
    fn it_formats_some_floats_with_some_extras() {
        let pound = Lotus::new("GBP", 1);
        assert_eq!("GBP 5,000.4", pound.format(5000.35f32));
    }
    #[test]
    fn it_formats_negative_floats() {
        let dollar = Lotus::new("$", 3);
        assert_eq!("$ (5,000.000)", dollar.format(-5000.0));
    }
    #[test]
    fn it_formats_a_zero_valued_float() {
        let dollar = Lotus::new("Rs. ", 2);
        assert_eq!("Rs.  --", dollar.format(0.00));
    }
    #[test]
    fn it_tests_if_builders_are_working_fine() {
        let rupee = LotusBuilder::default()
            .symbol("Rs.")
            .precision(1)
            .format("%s %v")
            .format_negative("%s (%v)")
            .format_zero("%s 0.00")
            .decimal_str(".")
            .thousand_str(" ")
            .build()
            .unwrap();
        assert_eq!("Rs. (2 000.0)", rupee.format(-2000.));
        assert_eq!("Rs. 0.00", rupee.format(0.));
        assert_eq!("Rs. 2 000 000.0", rupee.format(2_000_000.0));
    }
    #[test]
    fn lets_see_if_defaults_are_working() {
        let default_dollar = LotusBuilder::default()
            .format_zero("%s 0")
            .format_negative("%s neg %v")
            .build()
            .unwrap();
        assert_eq!("$ 123.00", default_dollar.format(123.0));
        assert_eq!("$ neg 123.00", default_dollar.format(-123.0));
        assert_eq!("$ 0", default_dollar.format(0.0));
    }
    #[test]
    fn macro_tests() {
        assert_eq!("$ 200.00", lotus!(200.0, "$"));
        assert_eq!("Rs. 200.00", lotus!(200, "Rs."));
    }
    #[test]
    fn macro_tests_arm_2() {
        assert_eq!("2,000,000.00", lotus!(2_000_000.0));
    }
}

#[derive(Debug, Builder)]
#[builder(default)]
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

    pub fn format<T: Into<f64> + Display>(&self, in_number: T) -> String {
        let number: f64 = in_number.into();
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

            let mut currencied = String::new();
            if number.is_sign_negative() {
                currencied.push_str(self.format_negative);
            } else {
                currencied.push_str(self.format);
            }
            currencied = currencied.replace("%v", formatted_float.as_str());
            currencied = currencied.replace("%s", &self.symbol[..]);
            return currencied;
        }
    }
}

#[macro_export]
macro_rules! lotus {
    ($x:expr, $y:expr) => {
        {
            let quick = LotusBuilder::default()
                .symbol($y)
                .build()
                .unwrap();
            quick.format($x)
        }
    };
    ($x: expr) => {
        {
            let quick = LotusBuilder::default()
                .format("%v")
                .build()
                .unwrap();
            quick.format($x)
        }
    }
}

impl<'a> Default for Lotus<'a> {
    fn default() -> Self {
        Lotus {
            symbol: "$",
            precision: 2,
            thousand_str: ",",
            decimal_str: ".",
            format: "%s %v",
            format_negative: "%s (%v)",
            format_zero: "%s --",
        }
    }
}
