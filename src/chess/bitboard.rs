use crate::chess::square::Square;
use std::fmt;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Bitboard(pub u64);

impl Bitboard {
    #[must_use]
    pub const fn from_index(sq: u8) -> Self {
        Self(1u64 << sq)
    }

    #[must_use]
    pub const fn from_square(sq: Square) -> Self {
        Self(1u64 << sq.0)
    }

    #[must_use]
    pub const fn from_file(sq: Square) -> Self {
        Self(0x101010101010101u64 << sq.file())
    }

    #[must_use]
    pub fn from_adj_files(sq: Square) -> Self {
        Self::from_file(sq).east() | Self::from_file(sq).west()
    }

    #[must_use]
    pub const fn from_rank(sq: Square) -> Self {
        Self(0xffu64 << (8 * sq.rank()))
    }

    #[must_use]
    pub const fn full() -> Self {
        Self(0xFFFFFFFFFFFFFFFF)
    }

    #[must_use]
    pub const fn empty() -> Self {
        Self(0x0)
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.0 == 0
    }

    #[must_use]
    pub const fn is_occupied(&self) -> bool {
        self.0 != 0
    }

    #[must_use]
    pub const fn is_full(&self) -> bool {
        self.0 == 0xFFFFFFFFFFFFFFFF
    }

    #[must_use]
    pub const fn is_set(&self, idx: Square) -> bool {
        ((self.0 >> idx.0) & 1) == 1
    }

    #[must_use]
    pub const fn north(&self) -> Self {
        Self(self.0 << 8)
    }

    #[must_use]
    pub const fn south(&self) -> Self {
        Self(self.0 >> 8)
    }

    #[must_use]
    pub const fn east(&self) -> Self {
        Self((self.0 << 1) & 0xfefefefefefefefe)
    }

    #[must_use]
    pub const fn west(&self) -> Self {
        Self((self.0 >> 1) & 0x7f7f7f7f7f7f7f7f)
    }

    #[must_use]
    pub const fn north_east(&self) -> Self {
        Self((self.0 << 9) & 0xfefefefefefefefe)
    }

    #[must_use]
    pub const fn north_west(&self) -> Self {
        Self((self.0 << 7) & 0x7f7f7f7f7f7f7f7f)
    }

    #[must_use]
    pub const fn south_east(&self) -> Self {
        Self((self.0 >> 7) & 0xfefefefefefefefe)
    }

    #[must_use]
    pub const fn south_west(&self) -> Self {
        Self((self.0 >> 9) & 0x7f7f7f7f7f7f7f7f)
    }

    #[must_use]
    pub const fn north_north(&self) -> Self {
        Self(self.0 << 16)
    }

    #[must_use]
    pub const fn count(&self) -> i32 {
        self.0.count_ones() as i32
    }

    #[must_use]
    pub fn flip(self) -> Bitboard {
        Self(self.0.swap_bytes())
    }

    #[must_use]
    pub fn lsb(self) -> Square {
        Square(self.0.trailing_zeros() as u8)
    }

    #[must_use]
    pub fn adjacent(self) -> Self {
        Bitboard(
            // North
            (self.0 << 8) |
            // South
            (self.0 >> 8) |
            // north west, south west, west
            (((self.0 << 7) | (self.0 >> 9) | (self.0 >> 1)) & 0x7f7f7f7f7f7f7f7f) |
            // north east, south east, east
            (((self.0 >> 7) | (self.0 << 9) | (self.0 << 1)) & 0xfefefefefefefefe),
        )
    }
}

impl BitAnd for Bitboard {
    type Output = Self;

    #[must_use]
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

impl Not for Bitboard {
    type Output = Self;

    #[must_use]
    fn not(self) -> Self {
        Self(!self.0)
    }
}

impl BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Bitboard) {
        self.0 &= rhs.0;
    }
}

impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Bitboard) {
        self.0 |= rhs.0;
    }
}

impl BitXorAssign for Bitboard {
    fn bitxor_assign(&mut self, rhs: Bitboard) {
        self.0 ^= rhs.0;
    }
}

impl BitXor for Bitboard {
    type Output = Bitboard;

    #[must_use]
    fn bitxor(self, rhs: Bitboard) -> Bitboard {
        Bitboard(self.0 ^ rhs.0)
    }
}

impl BitOr for Bitboard {
    type Output = Bitboard;

    #[must_use]
    fn bitor(self, rhs: Bitboard) -> Bitboard {
        Bitboard(self.0 | rhs.0)
    }
}

impl fmt::Display for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in (0..8).rev() {
            for x in 0..8 {
                let idx = Square(8 * y + x);
                if self.is_set(idx) {
                    write!(f, "1")?;
                } else {
                    write!(f, "0")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::chess::{bitboard::Bitboard, square::*};

    #[test]
    fn count_tests() {
        assert!(Bitboard(0).count() == 0);
        assert!(Bitboard(1).count() == 1);
        assert!(Bitboard(2).count() == 1);
        assert!(Bitboard(3).count() == 2);
        assert!(Bitboard::empty().count() == 0);
        assert!(Bitboard::full().count() == 64);
    }

    #[test]
    fn empty() {
        assert!(Bitboard(0).is_empty());
        assert!(!Bitboard(1).is_empty());
    }

    #[test]
    fn file_rank() {
        assert!(Bitboard::from_file(E4) == Bitboard(0x1010101010101010));
        assert!(Bitboard::from_rank(E4) == Bitboard(0xff000000));
    }

    #[test]
    fn adj_files() {
        assert!(Bitboard::from_adj_files(E4) == Bitboard(0x2828282828282828));
        assert!(Bitboard::from_adj_files(A1) == Bitboard(0x202020202020202));
        assert!(Bitboard::from_adj_files(H1) == Bitboard(0x4040404040404040));
    }

    #[test]
    fn bitxor() {
        assert!(Bitboard(0) ^ Bitboard(0) == Bitboard(0));
        assert!(Bitboard(1) ^ Bitboard(2) == Bitboard(3));
    }

    #[test]
    fn bitor() {
        assert!(Bitboard(1) | Bitboard(2) == Bitboard(3));
    }

    #[test]
    fn bitand() {
        assert!(Bitboard(1) & Bitboard(2) == Bitboard(0));
    }

    #[test]
    fn bitnot() {
        assert_eq!(!Bitboard(0), Bitboard(0xFFFFFFFFFFFFFFFF));
        assert_eq!(!Bitboard(0xFFFFFFFFFFFFFFFF), Bitboard(0));
    }

    #[test]
    fn north() {
        assert_eq!(Bitboard(0x0).north(), Bitboard(0x0));
        assert_eq!(Bitboard(0x1).north(), Bitboard(0x100));
        assert_eq!(Bitboard(0xff).north(), Bitboard(0xff00));
        assert_eq!(Bitboard(0xff0000000000).north(), Bitboard(0xff000000000000));
        assert_eq!(Bitboard(0xff00000000000000).north(), Bitboard(0x0));
    }

    #[test]
    fn south() {
        assert_eq!(Bitboard(0x0).south(), Bitboard(0x0));
        assert_eq!(Bitboard(0x100).south(), Bitboard(0x1));
        assert_eq!(Bitboard(0xff00).south(), Bitboard(0xff));
        assert_eq!(Bitboard(0xff000000000000).south(), Bitboard(0xff0000000000));
    }

    #[test]
    fn east() {
        assert_eq!(Bitboard(0x0).east(), Bitboard(0x0));
        assert_eq!(Bitboard(0x1).east(), Bitboard(0x2));
    }

    #[test]
    fn west() {
        assert_eq!(Bitboard(0x0).west(), Bitboard(0x0));
        assert_eq!(Bitboard(0x1).west(), Bitboard(0x0));
    }

    #[test]
    fn adjacent() {
        assert_eq!(Bitboard(0x0).adjacent(), Bitboard(0x0));
        assert_eq!(Bitboard(0x1).adjacent(), Bitboard(0x302));
        assert_eq!(Bitboard(0x80).adjacent(), Bitboard(0xc040));
        assert_eq!(
            Bitboard(0x8000000000000000).adjacent(),
            Bitboard(0x40c0000000000000)
        );
        assert_eq!(
            Bitboard(0x100000000000000).adjacent(),
            Bitboard(0x203000000000000)
        );
        assert_eq!(
            Bitboard(0x42000000004200).adjacent(),
            Bitboard(0xe7a5e70000e7a5e7)
        );
        assert_eq!(Bitboard(0x18000000).adjacent(), Bitboard(0x3c3c3c0000));
        assert_eq!(Bitboard(0x14000000).adjacent(), Bitboard(0x3e2a3e0000));
    }
}
