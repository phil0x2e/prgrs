//! prgrs is a progress bar for rust, that aims to work like the python package [tqdm](https://github.com/tqdm/tqdm).
//! # Example
//! ```
//! use prgrs::{Prgrs, writeln, Length};
//! use std::{thread, time};
//!
//! fn main() {
//!     for i in Prgrs::new(0..1000, 1000).set_length_move(Length::Proportional(0.5)){
//!         thread::sleep(time::Duration::from_millis(10));
//!         if i % 10 == 0{
//!             let str = format!("{}", i);
//!             writeln(&str).ok();
//!         }
//!     }
//! }
//! ```
use std::io::{self, Error, ErrorKind, Stdout, Write};
use terminal::{error, Action, Clear, Retrieved, Terminal, Value};
use terminal_size::{terminal_size, Height, Width};

pub struct Prgrs<T: Iterator> {
    iter: T,
    size: usize,
    curr: usize,
    len: Length,
    term: Terminal<Stdout>,
}

/// Use this struct to set the length of the progress debug_assert!
/// # Proportional (better use this when possible)
/// When using Proportional values below 0. are rounded to 0. and above 1. are rounded to 1.
///
/// A value of 0. means the progress bar will have a single step
///
/// A value of 1. will fill make the progress bar fit the entire width of the screen
/// # Absolute (use carful)
/// When using Absolute you specify the total length of the bar including the percentage count and parentheses
///
/// **Careful** values that are larger than the terminal will NOT be handled in a special manner, which will probably lead to glitches
///
/// values, that would make make the bar smaller than a single step however like negative values or for example 2 are ignored and the bar will have a single steps
pub enum Length {
    Absolute(usize),
    Proportional(f32),
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
            len: Length::Proportional(0.33),
            term: terminal::stdout(),
        }
    }

    /// Set the length of the progress bar, either as an absolute value or proportional to the size of the terminal
    ///
    /// By default the length is set to Length::Proportional(0.33)
    ///
    /// To set an absolute value please use Length::Absolute(val) and to set a proportional value use Length::Proportional(val)
    /// # Example
    /// ```
    /// use prgrs::{Prgrs, Length};
    /// let mut p = Prgrs::new(0..100, 100);
    /// p.set_length(Length::Proportional(0.5));
    /// for _ in p{
    ///     // do something here
    ///}
    /// ```
    pub fn set_length(&mut self, len: Length) {
        self.len = len;
    }

    /// Same as set_length, but the Instance of Prgrs, on which it is called is moved out and returned afterwards, which is useful for a oneliner
    /// # Example
    /// ```
    /// use prgrs::{Prgrs, Length};
    /// for _ in Prgrs::new(0..100, 100).set_length_move(Length::Proportional(0.5)){
    ///     // do something here
    ///}
    /// ```
    pub fn set_length_move(mut self, len: Length) -> Self {
        self.len = len;
        self
    }

    fn get_absolute_length(&self) -> usize {
        match self.len {
            Length::Absolute(l) => l,
            Length::Proportional(mut p) => {
                if let Ok(Retrieved::TerminalSize(x, _y)) = self.term.get(Value::TerminalSize) {
                    if p > 1. {
                        p = 1.;
                    }
                    if p < 0. {
                        p = 0.;
                    }
                    (x as f32 * p) as usize
                } else {
                    30
                }
            }
        }
    }

    fn create_bar(&self) -> String {
        let symbol = "#";
        let len = self.get_absolute_length();
        let mut steps = 1;
        if len > 10 {
            steps = len - 9; // 9 is length of all the other characters in the progress bar
        }
        let mut buf = String::new();
        buf.push_str("[");
        let ratio = self.curr as f32 / self.size as f32;
        let num_symbols = (ratio * steps as f32) as usize;
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
        if let Retrieved::CursorPosition(_x, y) = self.term.get(Value::CursorPosition)? {
            self.curr += 1;
            self.term.batch(Action::MoveCursorTo(0, y))?;
            self.term.act(Action::ClearTerminal(Clear::CurrentLine))?;
            let mut percentage = (self.curr as f32 / self.size as f32) * 100.;
            if percentage > 100. {
                percentage = 100.;
            }
            self.term
                .write(format!("{} ({:3.0}%)", self.create_bar(), percentage).as_bytes())?;
            self.term.flush_batch()?;
        }
        Ok(())
    }
}

impl<T: Iterator> Iterator for Prgrs<T> {
    type Item = T::Item;

    fn next(&mut self) -> std::option::Option<Self::Item> {
        let next = self.iter.next();
        if let Err(_e) = self.print_bar() {
            let mut percentage = (self.curr as f32 / self.size as f32) * 100.;
            if percentage > 100. {
                percentage = 100.;
            }
            print!("{} ({:3.0}%)\r", self.create_bar(), percentage);
            io::stdout().flush().ok();
        }

        if let None = next {
            println!("");
        }
        next
    }
}

/// Use this function to write to the terminal, while displaying a progress bar.
///
/// It may return an error, when the size of the terminal couldn't be determined.
///
/// # Example
/// ```
/// use prgrs::{Prgrs, writeln};
/// for i in Prgrs::new(0..100, 100){
///     if let Err(_) = writeln("test") {
///         println!("test")
///     }
/// }
/// ```
pub fn writeln(text: &str) -> Result<(), Error> {
    let size = terminal_size();
    if let Some((Width(w), Height(_h))) = size {
        // The whitespaces override the rest of the line, because \r doesn't delete characters already printed
        let num_missing_whitespaces = w as usize - text.len();
        let whitespaces = (0..num_missing_whitespaces)
            .map(|_| " ")
            .collect::<String>();
        println!("\r{}{}", text, whitespaces);
        return Ok(());
    } else {
        return Err(Error::new(
            ErrorKind::Other,
            "Issue determining size of your terminal",
        ));
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
