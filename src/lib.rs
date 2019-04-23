use std::default::Default;
use std::fmt::Display;

#[macro_use]
extern crate derive_builder;

#[macro_use]
mod macros;
mod tests;


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
