use std::fmt;
use std::ops::Not;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Colour {
    White = 0,
    Black,
}

impl Not for Colour {
    type Output = Self;

    #[must_use]
    fn not(self) -> Self {
        match self {
            Colour::White => Colour::Black,
            Colour::Black => Colour::White,
        }
    }
}

impl fmt::Display for Colour {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Colour::White => write!(f, "White")?,
            Colour::Black => write!(f, "Black")?,
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::chess::colour::Colour;

    #[test]
    fn operators() {
        assert!(Colour::White == Colour::White);
        assert!(Colour::Black == Colour::Black);
        assert!(Colour::White != Colour::Black);
        assert!(!Colour::White == Colour::Black);
        assert!(!Colour::Black == Colour::White);
    }
}
