use std::io::{self, Write};

pub struct Prgrs<T: Iterator> {
    iter: T,
    size: usize,
    curr: usize,
}

impl<T: Iterator> Prgrs<T> {
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
        match next {
            Some(_) => {
                self.curr += 1;
                print!("\r{} ({}/{})", self.create_bar(), self.curr, self.size);
                io::stdout().flush().unwrap();
            }
            None => {
                println!();
            }
        }
        next
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
