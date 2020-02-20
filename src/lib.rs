//! prgrs is a progress bar for rust, that aims to work like the python package [tqdm](https://github.com/tqdm/tqdm).
use std::io::Write;
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
}

impl<T: Iterator> Iterator for Prgrs<T> {
    type Item = T::Item;

    fn next(&mut self) -> std::option::Option<Self::Item> {
        let next = self.iter.next();
        let mut terminal = terminal::stdout();
        if let Retrieved::CursorPosition(_x, y) = terminal.get(Value::CursorPosition).unwrap() {
            match next {
                Some(_) => {
                    self.curr += 1;
                    if let Retrieved::CursorPosition(_x, y) =
                        terminal.get(Value::CursorPosition).unwrap()
                    {
                        terminal.batch(Action::MoveCursorTo(0, y)).unwrap();
                        terminal
                            .write(
                                format!(
                                    "{} ({:.0}%)",
                                    self.create_bar(),
                                    (self.curr as f32 / self.size as f32) * 100.
                                )
                                .as_bytes(),
                            )
                            .unwrap();
                        terminal.flush_batch().unwrap();
                    }
                }
                None => {
                    terminal.batch(Action::MoveCursorTo(0, y)).unwrap();
                    terminal
                        .write(format!("{} ({:.0}%)\n", self.create_bar(), 100).as_bytes())
                        .unwrap();
                    terminal.flush_batch().unwrap();
                }
            }
        }
        next
    }
}

/// Used to write somethin to the terminal, while displaying a progress bar
///
/// # Example
/// ```
/// use prgrs::{Prgrs, writeln};
/// for i in Prgrs::new(0..100, 100){
///     writeln("test");
///}
/// ```

pub fn writeln(text: &str) {
    let mut terminal = terminal::stdout();
    if let Retrieved::CursorPosition(_x, y) = terminal.get(Value::CursorPosition).unwrap() {
        terminal.batch(Action::MoveCursorTo(0, y)).unwrap();
        terminal
            .act(Action::ClearTerminal(Clear::FromCursorDown))
            .unwrap();
        terminal.write(format!("{}\n", text).as_bytes()).unwrap();

        terminal.flush_batch().unwrap();
    }
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
