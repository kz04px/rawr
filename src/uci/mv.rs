use crate::chess::{
    colour::Colour,
    mv::Mv,
    piece::Piece,
    position::Position,
    square::{Square, SquareIdx},
};

impl Mv {
    #[must_use]
    pub fn to_uci(&self, pos: &Position) -> String {
        let mut movestr = String::from("");

        let from = if pos.turn == Colour::White {
            self.from
        } else {
            self.from.flip()
        };

        let to = {
            let sq = if !pos.is_frc && pos.get_us().is_set(self.to) {
                if self.to.file() > self.from.file() {
                    Square::from_index(SquareIdx::G1)
                } else {
                    Square::from_index(SquareIdx::C1)
                }
            } else {
                self.to
            };

            if pos.turn == Colour::White {
                sq
            } else {
                sq.flip()
            }
        };

        movestr += &from.to_string();
        movestr += &to.to_string();
        movestr += match self.promo {
            Piece::Knight => "n",
            Piece::Bishop => "b",
            Piece::Rook => "r",
            Piece::Queen => "q",
            _ => "",
        };

        movestr
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::chess::{mv::Mv, piece::Piece, position::Position, square::*};

//     #[test]
//     fn uci_strings() {
//         let tests = [
//             (E2, E4, Piece::None, "e2e4"),
//             (E7, E8, Piece::None, "e7e8"),
//             (E7, E8, Piece::Queen, "e7e8q"),
//         ];

//         for (from, to, promo, movestr) in tests {
//             let pos = Position::startpos();
//             let mv = Mv { from, to, promo };
//             assert_eq!(mv.to_uci(&pos), movestr);
//         }
//     }

//     #[test]
//     fn uci_strings_flipped() {
//         let tests = [
//             (E2, E4, Piece::None, "e7e5"),
//             (E7, E8, Piece::None, "e2e1"),
//             (E7, E8, Piece::Queen, "e2e1q"),
//         ];

//         for (from, to, promo, _movestr) in tests {
//             let _mv = Mv { from, to, promo };
//             assert_eq!(mv.to_uci(pos), movestr);
//         }
//     }
// }
