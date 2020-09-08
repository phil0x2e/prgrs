//! prgrs is a simple progress bar for rust, that aims to work like the python package [tqdm](https://github.com/tqdm/tqdm).
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
//!
//! The output will look something like this:
//!
//! `[##############                     ] ( 42%)`
//!
use std::io::{self, Error, ErrorKind, Write};
use terminal_size::{terminal_size, Height, Width};

pub struct Prgrs<T: Iterator> {
    iter: T,
    size: usize,
    curr: usize,
    len: Length,
}

/// Use this struct to [set the length](struct.Prgrs.html#method.set_length) of the progress bar.
/// The lengths include the percentage count and parentheses at the end of the bar.
/// # Proportional (better use this whenever possible)
/// When using the Proportional variant values below 0. are rounded to 0. and values above 1. are rounded to 1.
///
/// A value of 0. means the progress bar will have a single step.
///
/// A value of 1. will make the progress bar fit the entire width of the screen.
///
/// The proportions are calculated for each iteration, so you could change the size of the terminal while displaying a progress bar, but this still may lead to smaller glitches.
/// # Absolute (use with care)
/// **Careful:** Values that are larger than the terminal will NOT be handled in a special manner, which will probably lead to glitches.
///
/// Values, that would make make the bar smaller than a single step however like negative values or for example 2 are ignored and the bar will have a single step.
pub enum Length {
    /// Used to set the absolute length of the progress bar in characters
    Absolute(usize),
    /// Used to set the length proportional to the width of your terminal
    ///
    /// The supplied value should be between 0 and 1
    Proportional(f64),
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
        }
    }

    /// Set the length of the progress bar. The default is `Length::Proportional(0.33)`
    ///
    /// To set an absolute value use [`Length::Absolute(val)`](enum.Length.html#variant.Absolute) and to set a proportional value use [`Length::Proportional(val)`](enum.Length.html#variant.Proportional)
    /// # Examples
    /// ```
    /// use prgrs::{Prgrs, Length};
    /// let mut p = Prgrs::new(0..100, 100);
    /// p.set_length(Length::Proportional(0.5));
    /// for _ in p{
    ///     // do something here
    ///}
    /// ```
    /// ```
    /// use prgrs::{Prgrs, Length};
    /// let mut p = Prgrs::new(0..100, 100);
    /// p.set_length(Length::Absolute(40));
    /// for _ in p{
    ///     // do something here
    ///}
    /// ```
    pub fn set_length(&mut self, len: Length) {
        self.len = len;
    }

    /// Same as [set_length()](struct.Prgrs.html#method.set_length), but the Instance of Prgrs, on which it is called is moved out and returned afterwards, which is useful for a oneliner
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
                if let Some((Width(x), Height(_y))) = terminal_size() {
                    if p > 1. {
                        p = 1.;
                    } else if p < 0. {
                        p = 0.;
                    }
                    (x as f64 * p) as usize
                } else {
                    50
                }
            }
        }
    }

    fn get_ratio(&self) -> f64 {
        self.curr as f64 / self.size as f64
    }

    fn create_bar(&self) -> String {
        let symbol = "#";
        let len = self.get_absolute_length();
        let mut steps = 1;
        let additional_chars = "[] (100%)".len();
        if len > additional_chars + 1 {
            steps = len - additional_chars;
        }
        let mut buf = String::from("[");
        if self.size == 0 {
            for _ in 0..steps {
                buf.push_str(symbol);
            }
        } else {
            let num_symbols = (self.get_ratio() * steps as f64) as usize;
            for _ in 0..num_symbols {
                buf.push_str(symbol);
            }
            for _ in 0..steps - num_symbols {
                buf.push_str(" ");
            }
        }
        buf.push_str("]");
        buf
    }
}

impl<T: Iterator> Iterator for Prgrs<T> {
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.iter.next();
        let mut percentage = self.get_ratio() * 100.;
        if percentage > 100. || percentage.is_nan() {
            percentage = 100.;
        }
        if let Some((Width(w), Height(_h))) = terminal_size() {
            let whitespaces = std::iter::repeat(" ").take(w as usize).collect::<String>();
            print!(
                "\r{}\r{} ({:3.0}%)\r",
                whitespaces,
                self.create_bar(),
                percentage
            );
        } else {
            print!("{} ({:3.0}%)\r", self.create_bar(), percentage);
        }
        io::stdout().flush().ok();

        if let None = next {
            println!("");
        }
        self.curr += 1;
        next
    }
}

/// Use this function to write to the terminal, while displaying a progress bar.
///
/// It may return an error, when the size of the terminal couldn't be determined.
///
///In this case the supplied text will **NOT** be printed, so you may want to print it with `println!()` instead.
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
    if let Some((Width(w), Height(_h))) = terminal_size() {
        // The whitespaces override the rest of the line, because \r doesn't delete characters already printed
        let whitespaces = (w as usize).checked_sub(text.len()).unwrap_or(0);
        let whitespaces = std::iter::repeat(" ").take(whitespaces).collect::<String>();
        println!("\r{}{}", text, whitespaces);
        Ok(())
    } else {
        Err(Error::new(
            ErrorKind::Other,
            "Issue determining size of your terminal",
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_prgrs() {
        assert_eq!(Prgrs::new(1..100, 100).next(), (1..100).next());
        assert_eq!(Prgrs::new(1..100, 100).last(), (1..100).last());
        assert_eq!(Prgrs::new(0..0, 0).next(), None);
    }
}
