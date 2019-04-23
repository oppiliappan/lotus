#[cfg(test)]
mod tests {
    use crate::*;

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
            .format_positive("%s %v")
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
