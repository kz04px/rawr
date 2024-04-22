#[cfg(test)]
mod tests {
    use rawr::{
        chess::position::Position,
        search::{
            hashtable::Hashtable,
            negamax::{self, INF},
            stats::Stats,
            ttentry::TTEntry,
        },
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
            ("k7/6R1/5R1P/8/8/8/8/K7 w - - 0 1", "f6f8"),
            ("k7/8/8/8/8/5r1p/6r1/K7 b - - 0 1", "f3f1"),
        ];

        let mut tt = Hashtable::<TTEntry>::new(16);
        let should_stop = |_stats: &Stats| false;

        for (fen, movestr) in tests {
            let pos = Position::from_fen(fen);
            let mut history = vec![];
            let mut stats = Stats::default();
            let _ = negamax::negamax(
                &pos,
                &mut history,
                &mut tt,
                &mut stats,
                &should_stop,
                -INF,
                INF,
                0,
                4,
            );
            assert!(!pos.in_check());
            assert!(stats.best_move.is_some());
            assert_eq!(movestr, stats.best_move.unwrap().to_uci(&pos));
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

        let mut tt = Hashtable::<TTEntry>::new(16);
        let should_stop = |_stats: &Stats| false;

        for (fen, movestr) in tests {
            println!("{}", fen);
            let pos = Position::from_fen(fen);
            let mut history = vec![];
            let mut stats = Stats::default();
            let _ = negamax::negamax(
                &pos,
                &mut history,
                &mut tt,
                &mut stats,
                &should_stop,
                -INF,
                INF,
                0,
                4,
            );
            assert!(!pos.in_check());
            assert!(stats.best_move.is_some());
            assert_eq!(movestr, stats.best_move.unwrap().to_uci(&pos));
        }
    }

    #[test]
    fn test_3fold() {
        let tests = [
            ("7k/2QQ4/8/8/8/PPP5/2q5/K7 b - - 0 1", "c2c1"),
            ("k7/2Q5/ppp5/8/8/8/2qq4/7K w - - 0 1", "c7c8"),
        ];

        let mut tt = Hashtable::<TTEntry>::new(16);
        let should_stop = |_stats: &Stats| false;

        for (fen, movestr) in tests {
            println!("{}", fen);
            let pos = Position::from_fen(fen);
            let mut history = vec![];
            let mut stats = Stats::default();
            let _ = negamax::negamax(
                &pos,
                &mut history,
                &mut tt,
                &mut stats,
                &should_stop,
                -INF,
                INF,
                0,
                4,
            );
            assert!(!pos.in_check());
            assert!(stats.best_move.is_some());
            assert_eq!(movestr, stats.best_move.unwrap().to_uci(&pos));
        }
    }

    #[test]
    fn test_50move() {
        let tests = [
            ("7k/8/R7/1R6/7K/8/7P/8 w - - 99 1", "h2h3"),
            ("8/7p/8/7k/1r6/r7/8/7K b - - 99 1", "h7h6"),
            ("8/8/8/P7/8/6n1/3R4/R3K2k w Q - 99 1", "a5a6"),
            ("r3k2K/3r4/6N1/8/p7/8/8/8 b q - 99 1", "a4a3"),
            // ("7k/1R6/R7/8/8/8/P7/4K3 w - - 99 1", "a6a8"),
            // ("4K3/p7/8/8/8/r7/1r6/7k b - - 99 1", "a3a1"),
        ];

        let mut tt = Hashtable::<TTEntry>::new(16);
        let should_stop = |_stats: &Stats| false;

        for (fen, movestr) in tests {
            println!("{}", fen);
            let pos = Position::from_fen(fen);
            let mut history = vec![];
            let mut stats = Stats::default();
            let _ = negamax::negamax(
                &pos,
                &mut history,
                &mut tt,
                &mut stats,
                &should_stop,
                -INF,
                INF,
                0,
                4,
            );
            assert!(!pos.in_check());
            assert!(stats.best_move.is_some());
            assert_eq!(movestr, stats.best_move.unwrap().to_uci(&pos));
        }
    }

    #[test]
    fn test_stalemate() {
        let tests = [
            ("k5q1/p7/8/6q1/6q1/6q1/8/Q6K w - - 0 1", "a1a7"),
            ("q6k/8/6Q1/6Q1/6Q1/8/P7/K5Q1 b - - 0 1", "a8a2"),
        ];

        let mut tt = Hashtable::<TTEntry>::new(16);
        let should_stop = |_stats: &Stats| false;

        for (fen, movestr) in tests {
            println!("{}", fen);
            let pos = Position::from_fen(fen);
            let mut history = vec![];
            let mut stats = Stats::default();
            let _ = negamax::negamax(
                &pos,
                &mut history,
                &mut tt,
                &mut stats,
                &should_stop,
                -INF,
                INF,
                0,
                4,
            );
            assert!(!pos.in_check());
            assert!(stats.best_move.is_some());
            assert_eq!(movestr, stats.best_move.unwrap().to_uci(&pos));
        }
    }

    #[test]
    fn test_underpromotions() {
        let tests = [
            ("8/5P1k/8/4B1K1/8/1B6/2N5/8 w - - 0 1", "f7f8n"),
            ("8/2n5/1b6/8/4b1k1/8/5p1K/8 b - - 0 1", "f2f1n"),
        ];

        let mut tt = Hashtable::<TTEntry>::new(16);
        let should_stop = |_stats: &Stats| false;

        for (fen, movestr) in tests {
            println!("{}", fen);
            let pos = Position::from_fen(fen);
            let mut history = vec![];
            let mut stats = Stats::default();
            let _ = negamax::negamax(
                &pos,
                &mut history,
                &mut tt,
                &mut stats,
                &should_stop,
                -INF,
                INF,
                0,
                4,
            );
            assert!(!pos.in_check());
            assert!(stats.best_move.is_some());
            assert_eq!(movestr, stats.best_move.unwrap().to_uci(&pos));
        }
    }
}
