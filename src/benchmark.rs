use crate::chess::position::Position;
use std::time::{Duration, Instant};

const FENS: [&str; 13] = [
    "startpos",
    "bqnb1rkr/pp3ppp/3ppn2/2p5/5P2/P2P4/NPP1P1PP/BQ1BNRKR w HFhf - 2 9",
    "2nnrbkr/p1qppppp/8/1ppb4/6PP/3PP3/PPP2P2/BQNNRBKR w HEhe - 1 9",
    "b1q1rrkb/pppppppp/3nn3/8/P7/1PPP4/4PPPP/BQNNRKRB w GE - 1 9",
    "qbbnnrkr/2pp2pp/p7/1p2pp2/8/P3PP2/1PPP1KPP/QBBNNR1R w hf - 0 9",
    "1nbbnrkr/p1p1ppp1/3p4/1p3P1p/3Pq2P/8/PPP1P1P1/QNBBNRKR w HFhf - 0 9",
    "qnbnr1kr/ppp1b1pp/4p3/3p1p2/8/2NPP3/PPP1BPPP/QNB1R1KR w HEhe - 1 9",
    "q1bnrkr1/ppppp2p/2n2p2/4b1p1/2NP4/8/PPP1PPPP/QNB1RRKB w ge - 1 9",
    "qbn1brkr/ppp1p1p1/2n4p/3p1p2/P7/6PP/QPPPPP2/1BNNBRKR w HFhf - 0 9",
    "qnnbbrkr/1p2ppp1/2pp3p/p7/1P5P/2NP4/P1P1PPP1/Q1NBBRKR w HFhf - 0 9",
    "qn1rbbkr/ppp2p1p/1n1pp1p1/8/3P4/P6P/1PP1PPPK/QNNRBB1R w hd - 2 9",
    "qnr1bkrb/pppp2pp/3np3/5p2/8/P2P2P1/NPP1PP1P/QN1RBKRB w GDg - 3 9",
    "qb1nrkbr/1pppp1p1/1n3p2/p1B4p/8/3P1P1P/PPP1P1P1/QBNNRK1R w HEhe - 0 9",
];

pub fn benchmark() {
    let mut total_nodes = 0;
    let mut total_time = Duration::default();

    println!(
        "{:<3}  {:>10}  {:>12}  {:>10}  {:>10}  {:>6}  {:>7}   {}",
        "Pos", "Nodes", "ΣNodes", "NPS", "ΣNPS", "Time", "ΣTime", "FEN"
    );
    for (idx, fen) in FENS.iter().enumerate() {
        let now = Instant::now();
        let pos = Position::from_fen(fen);
        let nodes = pos.perft(6);
        let elapsed = now.elapsed();

        total_nodes += nodes;
        total_time += elapsed;
        let nps = if elapsed.is_zero() {
            0.0
        } else {
            nodes as f32 / elapsed.as_secs_f32()
        };
        let total_nps = if total_time.is_zero() {
            0.0
        } else {
            total_nodes as f32 / total_time.as_secs_f32()
        };

        println!(
            "{:<3}  {:>10}  {:>12}  {:>10}  {:>10}  {:>6.3}  {:>7.3}   {}",
            idx + 1,
            nodes,
            total_nodes,
            nps,
            total_nps,
            elapsed.as_secs_f32(),
            total_time.as_secs_f32(),
            fen
        );
    }
}
