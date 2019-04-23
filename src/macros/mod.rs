mod macros {
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
                    .format_positive("%v")
                    .build()
                    .unwrap();
                quick.format($x)
            }
        }
    }
}
