use crate::chess::mv::Mv;
use crate::chess::position::Position;

impl Position {
    #[must_use]
    pub fn legal_moves(&self) -> Vec<Mv> {
        let mut movelist = Vec::with_capacity(128);

        self.move_generator(|_piece, from, to, promo| {
            movelist.push(Mv { from, to, promo });
        });

        movelist
    }
}
