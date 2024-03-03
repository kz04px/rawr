use crate::chess::mv::Mv;
use crate::chess::position::Position;

impl Position {
    #[must_use]
    pub fn legal_moves(&self) -> Vec<Mv> {
        let mut movelist = Vec::with_capacity(128);

        self.move_generator(|mv| {
            movelist.push(mv);
        });

        movelist
    }
}
