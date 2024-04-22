use rawr::{
    chess::{mv::Mv, position::Position},
    search::hashtable::Hashtable,
};

#[derive(Copy, Clone, Default, PartialEq)]
struct TTEntryPerft {
    hash: u64,
    nodes: u64,
    depth: i32,
}

#[must_use]
fn ttperft(pos: &Position, tt: &mut Hashtable<TTEntryPerft>, depth: i32) -> u64 {
    if depth == 0 {
        return 1;
    } else if depth == 1 {
        return pos.count_moves() as u64;
    }

    let mut total = 0;
    let ttentry = tt.poll(pos.hash);
    if ttentry.hash == pos.hash && ttentry.depth == depth {
        return ttentry.nodes;
    }

    pos.move_generator(|_piece, from, to, promo| {
        let mv = Mv { from, to, promo };
        let npos = pos.after_move::<true>(&mv);
        let nodes = ttperft(&npos, tt, depth - 1);
        total += nodes;
    });

    tt.add(
        pos.hash,
        &TTEntryPerft {
            hash: pos.hash,
            nodes: total,
            depth,
        },
    );

    total
}

#[cfg(test)]
mod tests {
    use crate::{ttperft, TTEntryPerft};
    use rawr::{chess::position::Position, search::hashtable::Hashtable};

    #[test]
    fn test_ttperft() {
        let tests: [(&str, [u64; 4]); 6] = [
            (
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
                [20, 400, 8902, 197281],
            ),
            (
                "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
                [48, 2039, 97862, 4085603],
            ),
            ("4k3/8/8/8/8/8/8/4K2R w K - 0 1", [15, 66, 1197, 7059]),
            ("4k3/8/8/8/8/8/8/R3K3 w Q - 0 1", [16, 71, 1287, 7626]),
            ("4k2r/8/8/8/8/8/8/4K3 w k - 0 1", [5, 75, 459, 8290]),
            ("r3k3/8/8/8/8/8/8/4K3 w q - 0 1", [5, 80, 493, 8897]),
        ];

        let mut tt = Hashtable::<TTEntryPerft>::new(16);

        for (fen, results) in tests {
            println!("FEN: {}", fen);
            let pos = Position::from_fen(fen);

            for (idx, expected) in results.iter().enumerate() {
                let nodes = ttperft(&pos, &mut tt, idx as i32 + 1);
                // let nodes = pos.perft(idx as u8 + 1);
                println!("depth {} nodes {}", idx + 1, nodes);
                assert_eq!(nodes, *expected);
            }
        }
    }
}
