# prgrs - A simple to use progress bar for your iterators
prgrs is a progress bar for rust, that aims to work like the python package [tqdm](https://github.com/tqdm/tqdm).

Example:
```rust
use std::{thread, time};
use prgrs::{Prgrs, writeln};

for i in Prgrs::new(0..1000, 1000) {
    thread::sleep(time::Duration::from_millis(5));
    let str = format!("{}", i);
    writeln(&str).expect("prgrs::writeln: Some Problem occured while trying to print");
}
```

## Todos:
- Set default length in proportion to the size of the terminal
- Make the length of the progress bar customizable
- Prevent flickering
