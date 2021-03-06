[![prgrs crate](https://img.shields.io/crates/v/prgrs.svg)](https://crates.io/crates/prgrs)
[![prgrs documentation](https://docs.rs/prgrs/badge.svg)](https://docs.rs/prgrs)
# prgrs - A simple to use progress bar for your iterators
prgrs is a simple progress bar for rust, that aims to work like the python package [tqdm](https://github.com/tqdm/tqdm).

It is a small library with just a single dependency.

prgrs should work for almost any linux terminal emulator. Windows could work too, because terminal supports windows but I haven't tested yet, so please let me know if you have.

Have a look at the [Documentation](https://docs.rs/prgrs).

Please use Version 0.6.0 or higher, the older versions only work correctly with a small amount of terminals.

### Example:
```rust
use prgrs::{Prgrs, writeln};
use std::{thread, time};

fn main() {
    for i in Prgrs::new(0..1000, 1000) {
        thread::sleep(time::Duration::from_millis(10));
        if i % 10 == 0{
            let str = format!("{}", i);
            writeln(&str).expect("prgrs::writeln: Some Problem occured while trying to print");
        }
    }
}
```
The output will look something like this:
```
[##############                     ] ( 42%)
```

## Todos:
- Feature to print current iteration from total iterations instead of a percentage
- Prevent flickering
