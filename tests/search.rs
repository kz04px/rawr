#[cfg(test)]
mod tests {
    use rawr::{
        chess::position::Position,
        search::{greedy, stats::Stats},
    };

    #[test]
    fn test_mate_in_one() {
        let tests = [
            ("6k1/R7/6K1/8/8/8/8/8 w - - 0 1", "a7a8"),
            ("8/8/8/8/8/6k1/r7/6K1 b - - 0 1", "a2a1"),
            ("6k1/4R3/6K1/q7/8/8/8/8 w - - 0 1", "e7e8"),
            ("8/8/8/8/Q7/6k1/4r3/6K1 b - - 0 1", "e2e1"),
            ("6k1/8/6K1/q3R3/8/8/8/8 w - - 0 1", "e5e8"),
            ("8/8/8/8/Q3r3/6k1/8/6K1 b - - 0 1", "e4e1"),
        ];

        for (fen, movestr) in tests {
            let pos = Position::from_fen(fen);
            let history = vec![];
            let mut stats = Stats::default();
            let bestmove = greedy::greedy(&pos, &history, &mut stats);
            assert!(!pos.in_check());
            assert!(bestmove.is_some());
            assert_eq!(movestr, bestmove.unwrap().to_uci(&pos));
        }
    }

    #[test]
    fn test_capture_valuable() {
        let tests = [
            ("5k2/8/8/b7/2N5/r7/8/5K2 w - - 0 1", "c4a3"),
            ("5k2/8/8/B7/2n5/R7/8/5K2 b - - 0 1", "c4a3"),
            ("5k2/8/8/b7/2N5/r7/8/5K2 w - - 0 1", "c4a3"),
            ("5k2/8/8/B7/2n5/R7/8/5K2 b - - 0 1", "c4a3"),
            ("4k3/8/8/1n1p4/2P5/8/8/4K3 w - - 0 1", "c4b5"),
            ("4k3/8/8/2p5/1N1P4/8/8/4K3 b - - 0 1", "c5b4"),
        ];

        for (fen, movestr) in tests {
            println!("{}", fen);
            let pos = Position::from_fen(fen);
            let history = vec![];
            let mut stats = Stats::default();
            let bestmove = greedy::greedy(&pos, &history, &mut stats);
            assert!(!pos.in_check());
            assert!(bestmove.is_some());
            assert_eq!(movestr, bestmove.unwrap().to_uci(&pos));
        }
    }
}
