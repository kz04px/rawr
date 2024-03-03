use crate::chess::bitboard::Bitboard;
use crate::chess::square::Square;

#[derive(Debug, Clone)]
pub struct BitboardIter(u64);

impl Iterator for BitboardIter {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            None
        } else {
            let index = self.0.trailing_zeros() as u8;
            self.0 &= self.0 - 1;
            Some(Square(index))
        }
    }
}

impl IntoIterator for Bitboard {
    type Item = Square;
    type IntoIter = BitboardIter;

    fn into_iter(self) -> Self::IntoIter {
        BitboardIter(self.0)
    }
}
