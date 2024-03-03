use std::ops::Not;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Side {
    Us = 0,
    Them,
}

impl Not for Side {
    type Output = Self;

    #[must_use]
    fn not(self) -> Self {
        match self {
            Side::Us => Side::Them,
            Side::Them => Side::Us,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::chess::side::Side;

    #[test]
    fn count_tests() {
        assert!(Side::Us == Side::Us);
        assert!(Side::Them == Side::Them);
        assert!(Side::Us != Side::Them);
        assert!(!Side::Us == Side::Them);
        assert!(!Side::Them == Side::Us);
    }
}
