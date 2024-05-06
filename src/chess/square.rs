use std::fmt;

#[rustfmt::skip]
pub enum SquareIdx {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct Square(pub u8);

impl Square {
    pub const fn from_coords(x: u8, y: u8) -> Self {
        Square(8 * y + x)
    }

    pub const fn from_index(idx: SquareIdx) -> Self {
        Square(idx as u8)
    }

    #[must_use]
    pub const fn rank(self) -> i32 {
        self.0 as i32 / 8
    }

    #[must_use]
    pub const fn file(self) -> i32 {
        self.0 as i32 % 8
    }

    #[must_use]
    pub fn flip(self) -> Square {
        Self(self.0 ^ 56)
    }

    #[must_use]
    pub fn maybe_flip(self, flip: bool) -> Square {
        Self(self.0 ^ (56 * flip as u8))
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let rank = self.0 % 8;
        let file = self.0 / 8;

        let ranks = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
        let files = ['1', '2', '3', '4', '5', '6', '7', '8'];

        write!(f, "{}", ranks[rank as usize])?;
        write!(f, "{}", files[file as usize])?;

        Ok(())
    }
}
