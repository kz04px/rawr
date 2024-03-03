use super::square::Square;
use crate::chess::colour::Colour;
use crate::chess::piece::Piece;
use crate::chess::position::Position;

impl Position {
    pub fn get_fen(&self) -> String {
        let npos = if self.turn == Colour::White {
            *self
        } else {
            let mut npos = *self;
            npos.flip();
            npos
        };
        let mut fen: String = String::default();

        // Board
        for y in (0..8).rev() {
            let mut num_spaces: i32 = 0;

            for x in 0..8 {
                let sq = Square::from_coords(x, y);
                let is_occupied = npos.is_occupied(sq);

                if is_occupied && num_spaces > 0 {
                    fen += &num_spaces.to_string();
                    num_spaces = 0;
                }

                match (npos.get_piece_on(sq), npos.get_colour_on(sq)) {
                    (Some(Piece::Pawn), Some(Colour::White)) => fen += "P",
                    (Some(Piece::Pawn), Some(Colour::Black)) => fen += "p",
                    (Some(Piece::Knight), Some(Colour::White)) => fen += "N",
                    (Some(Piece::Knight), Some(Colour::Black)) => fen += "n",
                    (Some(Piece::Bishop), Some(Colour::White)) => fen += "B",
                    (Some(Piece::Bishop), Some(Colour::Black)) => fen += "b",
                    (Some(Piece::Rook), Some(Colour::White)) => fen += "R",
                    (Some(Piece::Rook), Some(Colour::Black)) => fen += "r",
                    (Some(Piece::Queen), Some(Colour::White)) => fen += "Q",
                    (Some(Piece::Queen), Some(Colour::Black)) => fen += "q",
                    (Some(Piece::King), Some(Colour::White)) => fen += "K",
                    (Some(Piece::King), Some(Colour::Black)) => fen += "k",
                    (None, None) => {
                        num_spaces += 1;
                    }
                    (_, _) => panic!("Uh oh"),
                }
            }

            if num_spaces > 0 {
                fen += &num_spaces.to_string();
            }

            if y > 0 {
                fen += "/";
            }
        }

        // Side to move
        match self.turn {
            Colour::White => fen += " w",
            Colour::Black => fen += " b",
        };

        // Castling permissions
        if !npos.us_ksc && !npos.us_qsc && !npos.them_ksc && !npos.them_qsc {
            fen += " -";
        } else {
            fen += " ";
            if npos.us_ksc {
                fen += "K";
            }
            if npos.us_qsc {
                fen += "Q";
            }
            if npos.them_ksc {
                fen += "k";
            }
            if npos.them_qsc {
                fen += "q";
            }
        }

        // En passant
        match npos.ep {
            Some(sq) => {
                fen += " ";
                fen += &sq.to_string();
            }
            None => fen += " -",
        }

        // Halfmove counter
        fen += " ";
        fen += &npos.halfmoves.to_string();

        // Fullmove counter
        fen += " ";
        fen += &npos.fullmoves.to_string();

        fen
    }
}
