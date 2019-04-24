<h1 align="center">Lotus</h1>

Basic currency and number formatting library.

# Installation

1. Add the following lines to your `Cargo.toml`
```toml
[dependencies]
Lotus = "0.1.1" 
```
2. Use it in your crate:
```rust
extern crate Lotus;
use Lotus::*;
```

# Usage
Here are some examples which illustrate the library functionality. Please
read the [documentation](https://docs.rs/Lotus/0.1.1/Lotus/index.html) as well.
```
extern crate Lotus;
use Lotus::*;

// Builder format (recommended)
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
assert_eq!("Rs. 2 000 000.0", rupee.format(2_000_000));
assert_eq!("Rs. (2 000.0)", rupee.format(-2000));
assert_eq!("Rs. 0.00", rupee.format(0));

// Using Lotus::new()
let dollar = Lotus::new("$", 3); // Lotus::new(symbol, precision)
assert_eq!("$ 50,000.035", dollar.format(50_000.035));

// Using lotus! macro
let f = lotus!(150, "$");     // lotus!(number, symbol)
assert_eq!("$ 150.00", f);

let g = lotus!(2_000_000);    // lotus!(number)
assert_eq!("2,000,000.00", g);
```
# Other Programming Languages

 - [JS](http://openexchangerates.github.io/accounting.js/)
 - [Go](https://github.com/leekchan/accounting)
