use crate::chess::position::Position;

impl Position {
    #[must_use]
    pub fn after_null(&self) -> Self {
        let mut npos = *self;
        npos.makenull();
        npos
    }
}
