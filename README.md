# prgrs - A simple to use progress bar for your iterators
prgrs is a progress bar for rust, that aims to work like the python package [tqdm](https://github.com/tqdm/tqdm).

Have a look at the [Documentation](https://docs.rs/prgrs)

Example:
```rust
use prgrs::{Prgrs, writeln};
use std::{thread, time};

fn main() {
    for i in Prgrs::new(0..1000, 1000) {
        thread::sleep(time::Duration::from_millis(50));
        if i % 10 == 0{
            let str = format!("{}", i);
            writeln(&str).expect("prgrs::writeln: Some Problem occured while trying to print");
        }
    }
}
```

## Todos:
- Fix Issue where in some terminal emulators e.g. urxvt the entire screen is cleared and the progress bar is always at the very top and writeln doesn't work
  - The issue here seems to be, that most terminals don't work with multiple terminal::Terminal instances. But they are needed, because of the writeln function. I'll try to find a solution ASAP
- Prevent flickering
