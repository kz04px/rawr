use crate::chess::piece::Piece;
use crate::chess::square::Square;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct Mv {
    pub from: Square,
    pub to: Square,
    pub promo: Piece,
}

impl Mv {
    #[must_use]
    pub fn flipped(&self) -> Mv {
        Self {
            from: self.from.flip(),
            to: self.to.flip(),
            promo: self.promo,
        }
    }
}

// impl fmt::Display for Mv {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}", self.from)?;
//         write!(f, "{}", self.to)?;
//         if self.promo != Piece::None {
//             write!(f, "{}", self.promo)?;
//         }
//         Ok(())
//     }
// }
