use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Piece {
    Pawn = 0,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
    None,
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Piece::Pawn => writeln!(f, "Pawn")?,
            Piece::Knight => writeln!(f, "Knight")?,
            Piece::Bishop => writeln!(f, "Bishop")?,
            Piece::Rook => writeln!(f, "Rook")?,
            Piece::Queen => writeln!(f, "Queen")?,
            Piece::King => writeln!(f, "King")?,
            Piece::None => writeln!(f, "None")?,
        }

        Ok(())
    }
}
