#[cfg(test)]
mod tests {
    use rawr::chess::position::Position;

    #[test]
    fn perft_enpassant() {
        let tests: Vec<(&str, Vec<u64>)> = vec![
            // EP
            ("8/8/8/8/1k1PpN1R/8/8/4K3 b - d3 0 1", vec![1, 9, 193]),
            ("8/8/8/8/1k1Ppn1R/8/8/4K3 b - d3 0 1", vec![1, 17, 220]),
            ("4k3/8/8/2PpP3/8/8/8/4K3 w - d6 0 1", vec![1, 9, 47, 376]),
            ("4k3/8/8/8/2pPp3/8/8/4K3 b - d3 0 1", vec![1, 9, 47, 376]),
            // EP - pinned diagonal
            ("4k3/b7/8/2Pp4/8/8/8/6K1 w - d6 0 1", vec![1, 5, 45]),
            ("4k3/7b/8/4pP2/8/8/8/1K6 w - e6 0 1", vec![1, 5, 45]),
            ("6k1/8/8/8/2pP4/8/B7/3K4 b - d3 0 1", vec![1, 5, 45]),
            ("1k6/8/8/8/4Pp2/8/7B/4K3 b - e3 0 1", vec![1, 5, 45]),
            ("4k3/b7/8/1pP5/8/8/8/6K1 w - b6 0 1", vec![1, 6, 52]),
            ("4k3/7b/8/5Pp1/8/8/8/1K6 w - g6 0 1", vec![1, 6, 51]),
            ("6k1/8/8/8/1Pp5/8/B7/4K3 b - b3 0 1", vec![1, 6, 52]),
            ("1k6/8/8/8/5pP1/8/7B/4K3 b - g3 0 1", vec![1, 6, 51]),
            ("4k3/K7/8/1pP5/8/8/8/6b1 w - b6 0 1", vec![1, 6, 66]),
            ("4k3/7K/8/5Pp1/8/8/8/1b6 w - g6 0 1", vec![1, 6, 60]),
            ("6B1/8/8/8/1Pp5/8/k7/4K3 b - b3 0 1", vec![1, 6, 66]),
            ("1B6/8/8/8/5pP1/8/7k/4K3 b - g3 0 1", vec![1, 6, 60]),
            ("4k3/b7/8/2Pp4/3K4/8/8/8 w - d6 0 1", vec![1, 5, 44]),
            ("4k3/8/1b6/2Pp4/3K4/8/8/8 w - d6 0 1", vec![1, 6, 59]),
            ("4k3/8/b7/1Pp5/2K5/8/8/8 w - c6 0 1", vec![1, 6, 49]),
            ("4k3/8/7b/5pP1/5K2/8/8/8 w - f6 0 1", vec![1, 6, 49]),
            ("4k3/7b/8/4pP2/4K3/8/8/8 w - e6 0 1", vec![1, 5, 44]),
            ("4k3/8/6b1/4pP2/4K3/8/8/8 w - e6 0 1", vec![1, 6, 53]),
            ("4k3/8/3K4/1pP5/8/q7/8/8 w - b6 0 1", vec![1, 5, 114]),
            ("7k/4K3/8/1pP5/8/q7/8/8 w - b6 0 1", vec![1, 8, 171]),
            // EP - double check
            ("4k3/2rn4/8/2K1pP2/8/8/8/8 w - e6 0 1", vec![1, 4, 75]),
            // EP - pinned horizontal
            ("4k3/8/8/K2pP2r/8/8/8/8 w - d6 0 1", vec![1, 6, 94]),
            ("4k3/8/8/K2pP2q/8/8/8/8 w - d6 0 1", vec![1, 6, 130]),
            ("4k3/8/8/r2pP2K/8/8/8/8 w - d6 0 1", vec![1, 6, 87]),
            ("4k3/8/8/q2pP2K/8/8/8/8 w - d6 0 1", vec![1, 6, 129]),
            ("8/8/8/8/1k1Pp2R/8/8/4K3 b - d3 0 1", vec![1, 8, 125]),
            ("8/8/8/8/1R1Pp2k/8/8/4K3 b - d3 0 1", vec![1, 6, 87]),
            // EP - pinned vertical
            ("k7/8/4r3/3pP3/8/8/8/4K3 w - d6 0 1", vec![1, 5, 70]),
            ("k3K3/8/8/3pP3/8/8/8/4r3 w - d6 0 1", vec![1, 6, 91]),
            // EP - in check
            ("4k3/8/8/4pP2/3K4/8/8/8 w - e6 0 1", vec![1, 9, 49]),
            ("8/8/8/4k3/5Pp1/8/8/3K4 b - f3 0 1", vec![1, 9, 50]),
            // EP - block check
            ("4k3/8/K6r/3pP3/8/8/8/8 w - d6 0 1", vec![1, 6, 109]),
            ("4k3/8/K6q/3pP3/8/8/8/8 w - d6 0 1", vec![1, 6, 151]),
        ];

        for (fen, results) in tests {
            println!("{fen}");
            let pos = Position::from_fen(&fen.to_owned());
            for (idx, result) in results.iter().enumerate() {
                let nodes = pos.perft(idx as u8);
                assert_eq!(nodes, *result);
            }
        }
    }

    #[test]
    fn perft_double_checked() {
        let tests: Vec<(&str, Vec<u64>)> = vec![
            ("4k3/8/4r3/8/8/8/3p4/4K3 w - - 0 1", vec![1, 4, 80, 320]),
            ("4k3/8/4q3/8/8/8/3b4/4K3 w - - 0 1", vec![1, 4, 143, 496]),
        ];

        for (fen, results) in tests {
            println!("{fen}");
            let pos = Position::from_fen(&fen.to_owned());
            for (idx, result) in results.iter().enumerate() {
                let nodes = pos.perft(idx as u8);
                assert_eq!(nodes, *result);
            }
        }
    }

    #[test]
    fn perft_pins() {
        let tests: Vec<(&str, Vec<u64>)> = vec![
            ("4k3/8/8/8/1b5b/8/3Q4/4K3 w - - 0 1", vec![1, 3, 54, 1256]),
            ("4k3/8/8/8/1b5b/8/3R4/4K3 w - - 0 1", vec![1, 3, 54, 836]),
            ("4k3/8/8/8/1b5b/2Q5/5P2/4K3 w - - 0 1", vec![1, 6, 98, 2274]),
            ("4k3/8/8/8/1b5b/2R5/5P2/4K3 w - - 0 1", vec![1, 4, 72, 1300]),
            ("4k3/8/8/8/1b2r3/8/3Q4/4K3 w - - 0 1", vec![1, 3, 66, 1390]),
            (
                "4k3/8/8/8/1b2r3/8/3QP3/4K3 w - - 0 1",
                vec![1, 6, 119, 2074],
            ),
        ];

        for (fen, results) in tests {
            println!("{fen}");
            let pos = Position::from_fen(&fen.to_owned());
            for (idx, result) in results.iter().enumerate() {
                let nodes = pos.perft(idx as u8);
                assert_eq!(nodes, *result);
            }
        }
    }
}
