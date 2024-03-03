#[cfg(test)]
mod tests {
    use rawr::chess::{mv::Mv, piece::Piece, position::Position};

    #[must_use]
    fn is_capture(pos: &Position, mv: &Mv) -> bool {
        let is_normal_capture = pos.get_them().is_set(mv.to);
        let is_ep_capture = pos.get_piece_on(mv.from).is_some()
            && pos.get_piece_on(mv.from).unwrap() == Piece::Pawn
            && pos.get_us().is_set(mv.from)
            && pos.ep.is_some()
            && pos.ep.unwrap() == mv.to;
        is_normal_capture || is_ep_capture
    }

    #[test]
    fn test_legal_captures() {
        const FENS: [(i32, &str); 19] = [
            (0, "startpos"),
            // Pawns
            (1, "4k3/8/8/2n5/3P4/8/8/4K3 w - - 0 1"),
            (0, "4k3/8/8/3n4/3P4/8/8/4K3 w - - 0 1"),
            (1, "4k3/8/8/4n3/3P4/8/8/4K3 w - - 0 1"),
            (2, "4k3/8/8/2nnn3/3P4/8/8/4K3 w - - 0 1"),
            (0, "4k3/8/8/2NNN3/3P4/8/8/4K3 w - - 0 1"),
            // Knights
            // Bishops
            (2, "4k3/p5p1/1p3p2/8/3B4/8/1P3P2/4K3 w - - 0 1"),
            // Rooks
            // Queens
            // Kings
            // En passant
            (2, "4k3/8/8/2PpP3/8/8/8/4K3 w - d6 0 1"),
            (2, "4k3/8/8/8/2pPp3/8/8/4K3 b - d3 0 1"),
            (4, "3Q4/2B3k1/1R2K3/2PpP3/2N5/8/8/8 w - d6 0 1"),
            // Promotions
            (4, "2n1k3/1P6/8/8/8/8/8/4K3 w - - 0 1"),
            (4, "4k3/8/8/8/8/8/1p6/2N1K3 b - - 0 1"),
            // Pins
            (0, "4k3/4r3/8/8/3p1p2/4P3/8/4K3 w - - 0 1"),
            (0, "4k3/4r3/8/8/3p1p2/8/4N3/4K3 w - - 0 1"),
            (0, "4k3/4r3/8/8/3p1p2/4B3/8/4K3 w - - 0 1"),
            (1, "4k3/4r3/8/8/3pRp2/8/8/4K3 w - - 0 1"),
            (1, "4k3/4r3/8/8/3pQp2/8/8/4K3 w - - 0 1"),
            // Castling
            (0, "4k3/8/8/8/8/8/8/R3K2R w KQ - 0 1"),
            (0, "r3k2r/8/8/8/8/8/8/4K3 w kq - 0 1"),
        ];

        for (num_captures, fen) in FENS {
            let pos = Position::from_fen(fen);
            let legal_captures = pos.legal_captures();

            println!("{fen}");
            legal_captures
                .iter()
                .for_each(|mv| assert!(is_capture(&pos, mv)));
            assert_eq!(num_captures as usize, legal_captures.len());
        }
    }

    #[test]
    fn test_captures_all() {
        const FENS: [&str; 18] = [
            "startpos",
            "4k3/8/8/2n5/3P4/8/8/4K3 w - - 0 1",
            "4k3/8/8/3n4/3P4/8/8/4K3 w - - 0 1",
            "4k3/8/8/4n3/3P4/8/8/4K3 w - - 0 1",
            "4k3/8/8/2nnn3/3P4/8/8/4K3 w - - 0 1",
            "4k3/8/8/2NNN3/3P4/8/8/4K3 w - - 0 1",
            "4k3/p5p1/1p3p2/8/3B4/8/1P3P2/4K3 w - - 0 1",
            "4k3/8/8/2PpP3/8/8/8/4K3 w - d6 0 1",
            "4k3/8/8/8/2pPp3/8/8/4K3 b - d3 0 1",
            "2n1k3/1P6/8/8/8/8/8/4K3 w - - 0 1",
            "4k3/8/8/8/8/8/1p6/2N1K3 b - - 0 1",
            "4k3/4r3/8/8/3p1p2/4P3/8/4K3 w - - 0 1",
            "4k3/4r3/8/8/3p1p2/8/4N3/4K3 w - - 0 1",
            "4k3/4r3/8/8/3p1p2/4B3/8/4K3 w - - 0 1",
            "4k3/4r3/8/8/3pRp2/8/8/4K3 w - - 0 1",
            "4k3/4r3/8/8/3pQp2/8/8/4K3 w - - 0 1",
            "4k3/8/8/8/8/8/8/R3K2R w KQ - 0 1",
            "r3k2r/8/8/8/8/8/8/4K3 w kq - 0 1",
        ];

        for fen in FENS {
            let pos: Position = Position::from_fen(fen);
            let legal_captures = pos.legal_captures();
            let count_captures =
                pos.legal_moves().iter().fold(
                    0,
                    |acc, mv| if is_capture(&pos, mv) { acc + 1 } else { acc },
                );

            legal_captures
                .iter()
                .for_each(|mv| assert!(is_capture(&pos, mv)));
            assert_eq!(legal_captures.len(), count_captures);
        }
    }
}
