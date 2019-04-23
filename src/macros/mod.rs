mod macros {
    /// Quality of life formatting macros.
    #[macro_export]
    ///
    ///
    /// The `lotus` macro can
    /// - format currency (Ex: $ 2,000.0)
    /// - format numbers (Ex: 2,000.0)
    /// # Example
    /// ```
    /// # #[macro_use]
    /// # use crate::Lotus::*;
    ///
    /// let f = lotus!(150, "$");     // lotus!(number, symbol)
    /// assert_eq!("$ 150.00", f);
    /// 
    /// let g = lotus!(2_000_000);    // lotus!(number)
    /// assert_eq!("2,000,000.00", g);
    /// ```
    ///
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
                    .format_positive("%v")
                    .build()
                    .unwrap();
                quick.format($x)
            }
        }
    }
}
