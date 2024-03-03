use crate::chess::position::Position;
use std::time::Instant;

pub fn perft(pos: &mut Position, depth: u8) {
    for i in 1..=depth {
        let started = Instant::now();
        let nodes = pos.perft(i);
        let dt = started.elapsed();

        if !dt.is_zero() {
            let nps = nodes as f64 / dt.as_secs_f64();
            println!(
                "info depth {i} nodes {nodes} time {} nps {}",
                dt.as_millis(),
                nps as u64
            );
        } else {
            println!("info depth {i} nodes {nodes} time {}", dt.as_millis());
        }

        if i == depth {
            println!("nodes {nodes}");
        }
    }
}
