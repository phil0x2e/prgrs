//! prgrs is a progress bar for rust, that aims to work like the python package [tqdm](https://github.com/tqdm/tqdm).
use std::io::{self, Write};
use terminal::{error, Action, Clear, Retrieved, Value};

pub struct Prgrs<T: Iterator> {
    iter: T,
    size: usize,
    curr: usize,
}

impl<T: Iterator> Prgrs<T> {
    /// Creates a new Prgrs struct.
    ///
    /// You have to specify the number of elements in the Iterator as the second argument
    /// # Example
    /// ```
    /// use prgrs::Prgrs;
    /// for _ in Prgrs::new(0..100, 100){
    ///     // do something here
    ///}
    /// ```
    pub fn new(it: T, size: usize) -> Self {
        Prgrs::<T> {
            iter: it,
            size,
            curr: 0,
        }
    }

    fn create_bar(&self) -> String {
        let steps = 50;
        let symbol = "#";
        let mut buf = String::new();
        buf.push_str("[");
        let ratio = self.curr as f32 / self.size as f32;
        let num_symbols = (ratio * steps as f32) as u32;
        for _ in 0..num_symbols {
            buf.push_str(symbol);
        }
        for _ in 0..steps - num_symbols {
            buf.push_str(" ");
        }
        buf.push_str("]");
        buf
    }

    fn print_bar(&mut self) -> error::Result<()> {
        let mut terminal = terminal::stdout();
        if let Retrieved::CursorPosition(_x, y) = terminal.get(Value::CursorPosition)? {
            self.curr += 1;
            terminal.batch(Action::MoveCursorTo(0, y))?;
            let mut percentage = (self.curr as f32 / self.size as f32) * 100.;
            if percentage > 100. {
                percentage = 100.;
            }
            terminal.write(format!("{} ({:.0}%)", self.create_bar(), percentage).as_bytes())?;
            terminal.flush_batch()?;
        }
        Ok(())
    }
}

impl<T: Iterator> Iterator for Prgrs<T> {
    type Item = T::Item;

    fn next(&mut self) -> std::option::Option<Self::Item> {
        let next = self.iter.next();
        match self.print_bar() {
            Err(_e) => {
                let mut percentage = (self.curr as f32 / self.size as f32) * 100.;
                if percentage > 100. {
                    percentage = 100.;
                }
                print!("{} ({:.0}%)\r", self.create_bar(), percentage);
                match io::stdout().flush() {
                    Err(_) => {}
                    Ok(_) => {}
                }
            }
            Ok(_) => {}
        }

        match next {
            Some(n) => Some(n),
            None => {
                println!("");
                None
            }
        }
    }
}

/// Used to write somethin to the terminal, while displaying a progress bar
///
/// # Example
/// ```
/// use prgrs::{Prgrs, writeln};
/// for i in Prgrs::new(0..100, 100){
///     writeln("test").expect("Error while printing");
///}
/// ```

pub fn writeln(text: &str) -> error::Result<()> {
    let mut terminal = terminal::stdout();
    if let Retrieved::CursorPosition(_x, y) = terminal.get(Value::CursorPosition)? {
        terminal.batch(Action::MoveCursorTo(0, y))?;
        terminal.act(Action::ClearTerminal(Clear::FromCursorDown))?;
        terminal.write(format!("{}\n", text).as_bytes())?;
        terminal.flush_batch()?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::Prgrs;
    #[test]
    fn test_prgrs() {
        assert_eq!(Prgrs::new(1..100, 100).next(), (1..100).next());
        assert_eq!(Prgrs::new(1..100, 100).last(), (1..100).last());
    }
}
