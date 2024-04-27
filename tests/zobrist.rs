#[cfg(test)]
mod tests {
    use rawr::chess::{mv::Mv, position::Position};

    #[test]
    fn zobrist() {
        let fens = [
            // Kings only
            "4k3/8/8/8/8/8/8/4K3 w - - 0 1",
            "4k3/8/8/8/8/8/8/4K3 b - - 0 1",
            // Add pawns
            "4k3/4p3/8/8/8/8/4P3/4K3 w - - 0 1",
            "4k3/4p3/8/8/8/8/4P3/4K3 b - - 0 1",
            // Castling
            "4k2r/8/8/8/8/8/8/4K2R w Kk - 0 1",
            "4k2r/8/8/8/8/8/8/4K2R b Kk - 0 1",
            "r3k3/8/8/8/8/8/8/R3K3 w Qq - 0 1",
            "r3k3/8/8/8/8/8/8/R3K3 b Qq - 0 1",
            // Promo
            "4k3/P7/8/8/8/8/p7/4K3 w - - 0 1",
            "4k3/P7/8/8/8/8/p7/4K3 b - - 0 1",
            // Promo capture
            "1n2k3/P7/8/8/8/8/p7/1N2K3 w - - 0 1",
            "1n2k3/P7/8/8/8/8/p7/1N2K3 b - - 0 1",
            // EP
            "4k3/8/8/3pP3/8/8/8/4K3 w - d6 0 1",
            "4k3/8/8/8/3Pp3/8/8/4K3 b - d3 0 1",
            // Other
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
            "r1bq1rk1/pp2ppbp/2np1np1/8/2BNP3/2N1BP2/PPPQ2PP/R3K2R b KQ - 4 9",
        ];

        for fen in fens {
            println!("{fen}");
            let pos = Position::from_fen(fen);

            pos.move_generator(|_piece, from, to, promo| {
                let mv = Mv { from, to, promo };
                let predicted = pos.predict_hash(&mv);
                let npos = pos.after_move::<true>(&mv);
                println!("- {}", mv.to_uci(&pos));
                assert_eq!(npos.calculate_hash(), predicted);
            });
        }
    }

    #[test]
    fn zobrist_different() {
        let fens = [
            // Side to move
            (
                "4k3/8/8/8/8/8/8/4K3 w - - 0 1",
                "4k3/8/8/8/8/8/8/4K3 b - - 0 1",
            ),
            // EP square
            (
                "4k3/8/8/8/4P3/8/8/4K3 w - e3 0 1",
                "4k3/8/8/8/4P3/8/8/4K3 w - - 0 1",
            ),
            // Castling permissions
            (
                "r3k2r/8/8/8/8/8/8/R3K2R w - - 0 1",
                "r3k2r/8/8/8/8/8/8/R3K2R w K - 0 1",
            ),
            (
                "r3k2r/8/8/8/8/8/8/R3K2R w - - 0 1",
                "r3k2r/8/8/8/8/8/8/R3K2R w Q - 0 1",
            ),
            (
                "r3k2r/8/8/8/8/8/8/R3K2R w - - 0 1",
                "r3k2r/8/8/8/8/8/8/R3K2R w k - 0 1",
            ),
            (
                "r3k2r/8/8/8/8/8/8/R3K2R w - - 0 1",
                "r3k2r/8/8/8/8/8/8/R3K2R w q - 0 1",
            ),
            (
                "r3k2r/8/8/8/8/8/8/R3K2R w K - 0 1",
                "r3k2r/8/8/8/8/8/8/R3K2R w k - 0 1",
            ),
            (
                "r3k2r/8/8/8/8/8/8/R3K2R w Q - 0 1",
                "r3k2r/8/8/8/8/8/8/R3K2R w q - 0 1",
            ),
            // Other
            (
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
                "rnbqkb1r/pppppppp/5n2/8/8/5N2/PPPPPPPP/RNBQKB1R w KQkq - 2 2",
            ),
        ];

        for (fen1, fen2) in fens {
            println!("{fen1} vs {fen2}");
            let pos1 = Position::from_fen(fen1);
            let pos2 = Position::from_fen(fen2);

            assert_ne!(pos1.hash, pos2.hash);
        }
    }

    #[test]
    fn zobrist_same() {
        let fens = [
            (
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 50 1",
            ),
            (
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 50",
            ),
        ];

        for (fen1, fen2) in fens {
            println!("{fen1} vs {fen2}");
            let pos1 = Position::from_fen(fen1);
            let pos2 = Position::from_fen(fen2);

            assert_eq!(pos1.hash, pos2.hash);
        }
    }
}
