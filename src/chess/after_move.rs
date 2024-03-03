use crate::chess::mv::Mv;
use crate::chess::position::Position;

impl Position {
    #[must_use]
    pub fn after_move<const UPDATE_HASH: bool>(&self, mv: &Mv) -> Self {
        let mut npos = *self;
        npos.makemove::<UPDATE_HASH>(mv);
        npos
    }
}
