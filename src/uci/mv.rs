use crate::chess::{colour::Colour, mv::Mv, piece::Piece, position::Position};

impl Mv {
    #[must_use]
    pub fn to_uci(&self, pos: &Position) -> String {
        let mut movestr = String::from("");

        if pos.turn == Colour::Black {
            movestr += &self.from.flip().to_string();
            movestr += &self.to.flip().to_string();
        } else {
            movestr += &self.from.to_string();
            movestr += &self.to.to_string();
        }

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
