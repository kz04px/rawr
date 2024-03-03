use crate::chess::position::Position;

impl Position {
    #[must_use]
    pub fn from_fen(fen: &str) -> Self {
        let mut pos = Position::default();
        pos.set_fen(fen);
        pos
    }
}

#[cfg(test)]
mod tests {
    use crate::chess::position::Position;

    #[test]
    fn fens() {
        assert_eq!(Position::from_fen("startpos"), Position::startpos());
        assert_eq!(
            Position::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"),
            Position::startpos()
        );
    }
}
