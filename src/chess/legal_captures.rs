use crate::chess::mv::Mv;
use crate::chess::piece::Piece;
use crate::chess::position::Position;

impl Position {
    #[must_use]
    pub fn legal_captures(&self) -> Vec<Mv> {
        let mut movelist = Vec::with_capacity(128);

        self.move_generator(|piece, from, to, promo| {
            let is_normal_capture = self.get_them().is_set(to);
            let is_ep_capture = piece == Piece::Pawn && self.ep.is_some() && to == self.ep.unwrap();

            if !is_normal_capture && !is_ep_capture {
                return;
            }

            movelist.push(Mv { from, to, promo });
        });

        movelist
    }
}
