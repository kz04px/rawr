use crate::chess::mv::Mv;
use crate::chess::piece::Piece;
use crate::chess::position::Position;
use crate::chess::square::Square;
use crate::chess::square::E1;
use crate::chess::square::E8;
use std::str::SplitAsciiWhitespace;

pub fn moves(stream: &mut SplitAsciiWhitespace, pos: &mut Position, history: &mut Vec<u64>) {
    for movestr in stream.by_ref() {
        let mv = if let Some(gg) = pos.legal_moves().iter().find(|x| x.to_uci(pos) == movestr) {
            Some(*gg)
        } else if movestr == "e1g1" {
            Some(Mv {
                from: E1,
                to: Square::from_coords(pos.castle_files[0], 0),
                promo: Piece::None,
            })
        } else if movestr == "e1c1" {
            Some(Mv {
                from: E1,
                to: Square::from_coords(pos.castle_files[1], 0),
                promo: Piece::None,
            })
        } else if movestr == "e8g8" {
            Some(Mv {
                from: E8,
                to: Square::from_coords(pos.castle_files[2], 0),
                promo: Piece::None,
            })
        } else if movestr == "e8c8" {
            Some(Mv {
                from: E8,
                to: Square::from_coords(pos.castle_files[3], 0),
                promo: Piece::None,
            })
        } else {
            None
        };

        if let Some(found) = mv {
            pos.makemove::<true>(&found);
            history.push(pos.hash);
        } else {
            println!("info string unknown move {}", movestr);
        }
    }
}
