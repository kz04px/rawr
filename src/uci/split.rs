use crate::chess::position::Position;
use std::time::Instant;

pub fn split(pos: &mut Position, depth: u8) {
    let moves = pos.legal_moves();
    let mut total = 0u64;

    let start = Instant::now();
    for mv in moves {
        let npos = pos.after_move::<false>(&mv);
        let nodes = npos.perft(depth - 1);
        total += nodes;

        println!("{} {nodes}", mv.to_uci(pos));
    }
    let elapsed = start.elapsed();

    let nps = if elapsed.is_zero() {
        None
    } else {
        Some((total as f64 / elapsed.as_secs_f64()) as u64)
    };

    if let Some(nps) = nps {
        println!("nps {}", nps);
    }
    println!("time {}", elapsed.as_millis());
    println!("nodes {total}");
}
