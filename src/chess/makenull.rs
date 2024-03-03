use crate::chess::position::Position;

impl Position {
    pub fn makenull(&mut self) {
        self.flip();
        self.halfmoves = 0;
        self.hash ^= 0x8445;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn makenull() {
        let mut pos = Position::startpos();

        assert_eq!(pos, Position::startpos());
        assert_eq!(pos.hash, Position::startpos().hash);

        pos.makenull();

        assert_ne!(pos, Position::startpos());
        assert_ne!(pos.hash, Position::startpos().hash);

        pos.makenull();

        assert_eq!(pos, Position::startpos());
        assert_eq!(pos.hash, Position::startpos().hash);
    }
}
