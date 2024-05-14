use crate::chess::bitboard::Bitboard;
use crate::chess::colour::Colour;
use crate::chess::mv::Mv;
use crate::chess::piece::Piece;
use crate::chess::side::Side;
use crate::chess::square::Square;
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Position {
    pub colours: [Bitboard; 2],
    pub pieces: [Bitboard; 6],
    pub halfmoves: i32,
    pub fullmoves: i32,
    pub turn: Colour,
    pub ep: Option<Square>,
    pub us_ksc: bool,
    pub us_qsc: bool,
    pub them_ksc: bool,
    pub them_qsc: bool,
    pub castle_files: [u8; 4],
    pub hash: u64,
    pub is_frc: bool,
}

impl Default for Position {
    #[must_use]
    fn default() -> Self {
        Self {
            colours: [Bitboard(0x0), Bitboard(0x0)],
            pieces: [
                Bitboard(0x0),
                Bitboard(0x0),
                Bitboard(0x0),
                Bitboard(0x0),
                Bitboard(0x0),
                Bitboard(0x0),
            ],
            halfmoves: 0,
            fullmoves: 1,
            turn: Colour::White,
            ep: None,
            us_ksc: false,
            us_qsc: false,
            them_ksc: false,
            them_qsc: false,
            castle_files: [7, 0, 7, 0],
            hash: 0u64,
            is_frc: false,
        }
    }
}

impl Position {
    #[must_use]
    pub const fn startpos() -> Self {
        Self {
            colours: [Bitboard(0xFFFF), Bitboard(0xFFFF000000000000)],
            pieces: [
                Bitboard(0x00FF00000000FF00),
                Bitboard(0x4200000000000042),
                Bitboard(0x2400000000000024),
                Bitboard(0x8100000000000081),
                Bitboard(0x800000000000008),
                Bitboard(0x1000000000000010),
            ],
            halfmoves: 0,
            fullmoves: 1,
            turn: Colour::White,
            ep: None,
            us_ksc: true,
            us_qsc: true,
            them_ksc: true,
            them_qsc: true,
            castle_files: [7, 0, 7, 0],
            hash: 0x3adfd94b38170629,
            is_frc: false,
        }
    }

    #[must_use]
    pub fn from_flipped(pos: &Position) -> Self {
        let mut npos = *pos;
        npos.flip();
        npos
    }

    #[must_use]
    pub const fn get_turn(&self) -> Colour {
        self.turn
    }

    #[must_use]
    pub const fn get_us(&self) -> Bitboard {
        self.colours[Side::Us as usize]
    }

    #[must_use]
    pub const fn get_them(&self) -> Bitboard {
        self.colours[Side::Them as usize]
    }

    #[must_use]
    pub fn get_white(&self) -> Bitboard {
        if self.turn == Colour::White {
            self.colours[Side::Us as usize]
        } else {
            self.colours[Side::Them as usize]
        }
    }

    #[must_use]
    pub fn get_black(&self) -> Bitboard {
        if self.turn == Colour::White {
            self.colours[Side::Them as usize]
        } else {
            self.colours[Side::Us as usize]
        }
    }

    #[must_use]
    pub const fn get_side(&self, side: Side) -> Bitboard {
        self.colours[side as usize]
    }

    #[must_use]
    pub fn get_empty(&self) -> Bitboard {
        !(self.colours[0] | self.colours[1])
    }

    #[must_use]
    pub fn get_occupied(&self) -> Bitboard {
        self.colours[0] | self.colours[1]
    }

    #[must_use]
    pub fn get_pawns(&self) -> Bitboard {
        self.pieces[Piece::Pawn as usize]
    }

    #[must_use]
    pub fn get_knights(&self) -> Bitboard {
        self.pieces[Piece::Knight as usize]
    }

    #[must_use]
    pub fn get_bishops(&self) -> Bitboard {
        self.pieces[Piece::Bishop as usize]
    }

    #[must_use]
    pub fn get_rooks(&self) -> Bitboard {
        self.pieces[Piece::Rook as usize]
    }

    #[must_use]
    pub fn get_queens(&self) -> Bitboard {
        self.pieces[Piece::Queen as usize]
    }

    #[must_use]
    pub fn get_kings(&self) -> Bitboard {
        self.pieces[Piece::King as usize]
    }

    #[must_use]
    pub fn get_piece(&self, idx: usize) -> Bitboard {
        self.pieces[idx]
    }

    #[must_use]
    pub fn get_piece_on(&self, sq: Square) -> Option<Piece> {
        if self.pieces[Piece::Pawn as usize].is_set(sq) {
            Some(Piece::Pawn)
        } else if self.pieces[Piece::Knight as usize].is_set(sq) {
            Some(Piece::Knight)
        } else if self.pieces[Piece::Bishop as usize].is_set(sq) {
            Some(Piece::Bishop)
        } else if self.pieces[Piece::Rook as usize].is_set(sq) {
            Some(Piece::Rook)
        } else if self.pieces[Piece::Queen as usize].is_set(sq) {
            Some(Piece::Queen)
        } else if self.pieces[Piece::King as usize].is_set(sq) {
            Some(Piece::King)
        } else {
            None
        }
    }

    #[must_use]
    pub fn get_colour_on(&self, sq: Square) -> Option<Colour> {
        if self.colours[0].is_set(sq) {
            Some(self.turn)
        } else if self.colours[1].is_set(sq) {
            Some(!self.turn)
        } else {
            None
        }
    }

    #[must_use]
    pub fn is_occupied(&self, sq: Square) -> bool {
        self.get_occupied().is_set(sq)
    }

    #[must_use]
    pub fn is_empty(&self, sq: Square) -> bool {
        !self.is_occupied(sq)
    }

    #[must_use]
    pub fn is_capture(&self, mv: &Mv) -> bool {
        self.get_them().is_set(mv.to)
            || (self.get_pawns().is_set(mv.from) && self.ep.is_some() && self.ep.unwrap() == mv.to)
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let npos = if self.turn == Colour::White {
            *self
        } else {
            let mut npos = *self;
            npos.flip();
            npos
        };

        for y in (0..8).rev() {
            for x in 0..8 {
                let idx = Square(8 * y + x);
                if npos.get_pawns().is_set(idx) {
                    if npos.get_white().is_set(idx) {
                        write!(f, "P")?;
                    } else {
                        write!(f, "p")?;
                    }
                } else if npos.get_knights().is_set(idx) {
                    if npos.get_white().is_set(idx) {
                        write!(f, "N")?;
                    } else {
                        write!(f, "n")?;
                    }
                } else if npos.get_bishops().is_set(idx) {
                    if npos.get_white().is_set(idx) {
                        write!(f, "B")?;
                    } else {
                        write!(f, "b")?;
                    }
                } else if npos.get_rooks().is_set(idx) {
                    if npos.get_white().is_set(idx) {
                        write!(f, "R")?;
                    } else {
                        write!(f, "r")?;
                    }
                } else if npos.get_queens().is_set(idx) {
                    if npos.get_white().is_set(idx) {
                        write!(f, "Q")?;
                    } else {
                        write!(f, "q")?;
                    }
                } else if npos.get_kings().is_set(idx) {
                    if npos.get_white().is_set(idx) {
                        write!(f, "K")?;
                    } else {
                        write!(f, "k")?;
                    }
                } else {
                    write!(f, "-")?;
                }
            }
            writeln!(f)?;
        }

        writeln!(f, "Turn: {}", self.turn)?;
        writeln!(f, "Check: {}", self.in_check())?;
        writeln!(f, "Halfmoves: {}", npos.halfmoves)?;
        writeln!(f, "Fullmoves: {}", npos.fullmoves)?;
        if npos.ep.is_some() {
            writeln!(f, "EP: {}", npos.ep.unwrap())?;
        } else {
            writeln!(f, "EP: -")?;
        }
        if !npos.us_ksc && !npos.us_qsc && !npos.them_ksc && !npos.them_qsc {
            writeln!(f, "Castling: -")?;
        } else {
            write!(f, "Castling: ")?;
            if npos.us_ksc {
                write!(f, "{}", ('A' as u8 + self.castle_files[0]) as char)?;
            }
            if npos.us_qsc {
                write!(f, "{}", ('A' as u8 + self.castle_files[1]) as char)?;
            }
            if npos.them_ksc {
                write!(f, "{}", ('a' as u8 + self.castle_files[2]) as char)?;
            }
            if npos.them_qsc {
                write!(f, "{}", ('a' as u8 + self.castle_files[3]) as char)?;
            }
            writeln!(f)?;
        }
        writeln!(f, "Hash: {:#x}", self.hash)?;
        writeln!(f, "FRC: {}", self.is_frc)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chess::square::SquareIdx;

    #[test]
    fn startpos() {
        let pos = Position::startpos();
        let moves = pos.legal_moves();
        for mv in moves {
            assert!(!pos.is_capture(&mv));
        }
    }

    #[test]
    fn captures() {
        let pos = Position::from_fen("r3k2r/6P1/5B2/3pP3/8/2N5/8/R3K2R w KQkq d6 0 2");
        let captures = [
            // En passant
            (SquareIdx::E5, SquareIdx::D6, Piece::None),
            // Promotion
            (SquareIdx::G7, SquareIdx::H8, Piece::Queen),
            (SquareIdx::G7, SquareIdx::H8, Piece::Rook),
            (SquareIdx::G7, SquareIdx::H8, Piece::Bishop),
            (SquareIdx::G7, SquareIdx::H8, Piece::Knight),
            // Regular
            (SquareIdx::C3, SquareIdx::D5, Piece::None),
            (SquareIdx::A1, SquareIdx::A8, Piece::None),
            (SquareIdx::H1, SquareIdx::H8, Piece::None),
        ];

        for (from, to, promo) in captures {
            let mv = Mv {
                from: Square::from_index(from),
                to: Square::from_index(to),
                promo,
            };
            assert!(pos.is_capture(&mv));
        }
    }

    #[test]
    fn noncaptures() {
        let pos = Position::from_fen("r3k2r/6P1/5B2/3pP3/8/2N5/8/R3K2R w KQkq d6 0 2");
        let noncaptures = [
            // Castling
            (SquareIdx::E1, SquareIdx::H1, Piece::None),
            (SquareIdx::E1, SquareIdx::A1, Piece::None),
            // Promotion
            (SquareIdx::G7, SquareIdx::G8, Piece::Queen),
            (SquareIdx::G7, SquareIdx::G8, Piece::Rook),
            (SquareIdx::G7, SquareIdx::G8, Piece::Bishop),
            (SquareIdx::G7, SquareIdx::G8, Piece::Knight),
            // Regular
            (SquareIdx::E5, SquareIdx::E6, Piece::None),
            (SquareIdx::E1, SquareIdx::F2, Piece::None),
        ];

        for (from, to, promo) in noncaptures {
            let mv = Mv {
                from: Square::from_index(from),
                to: Square::from_index(to),
                promo,
            };
            assert!(!pos.is_capture(&mv));
        }
    }

    #[test]
    fn capture_movegen() {
        let fens = [
            "r3k2r/6P1/5B2/3pP3/8/2N5/8/R3K2R w KQkq d6 0 2",
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
            "4k3/8/8/8/8/8/8/4K2R w K - 0 1",
            "r3k1r1/8/8/8/8/8/8/R3K2R w KQq - 0 1",
            "8/1n4N1/2k5/8/8/5K2/1N4n1/8 b - - 0 1",
        ];
        for fen in fens {
            let pos = Position::from_fen(fen);
            let captures = pos.legal_captures();
            for capture in captures {
                assert!(pos.is_capture(&capture));
            }
        }
    }
}
