use crate::chess::position::Position;
use crate::chess::zobrist;

impl Position {
    pub fn makenull(&mut self) {
        self.hash ^= zobrist::turn_key();
        if let Some(sq) = self.ep {
            self.hash ^= zobrist::ep_key(sq);
        }

        self.flip();
        self.halfmoves = 0;
        self.ep = None;

        debug_assert_eq!(self.hash, self.calculate_hash());
        debug_assert!(self.validate().is_ok());
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

    #[test]
    fn ep() {
        let mut pos1 =
            Position::from_fen("rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2");
        let mut pos2 =
            Position::from_fen("rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq e3 0 2");

        assert_ne!(pos1, pos2);
        assert_ne!(pos1.hash, pos2.hash);

        pos1.makenull();
        pos2.makenull();

        assert_eq!(pos1, pos2);
        assert_eq!(pos1.hash, pos2.hash);
    }

    #[test]
    fn same() {
        let pos1 = Position::from_fen("rnbqkbnr/pppppppp/8/8/8/5N2/PPPPPPPP/RNBQKB1R w KQkq - 0 1");
        let mut pos2 =
            Position::from_fen("rnbqkbnr/pppppppp/8/8/8/5N2/PPPPPPPP/RNBQKB1R b KQkq - 1 1");
        pos2.makenull();

        assert_eq!(pos1, pos2);
        assert_eq!(pos1.hash, pos2.hash);
    }
}
