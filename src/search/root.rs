use self::info::Info;
use self::stats::Stats;
use crate::chess::{mv::Mv, position::Position};
use crate::search::*;
use std::time::Instant;

#[must_use]
pub fn root(
    pos: Position,
    history: &mut Vec<u64>,
    _settings: settings::Type,
    info_printer: fn(&Info),
) -> Result<Mv, &'static str> {
    let mut stats = Stats::default();
    let start = Instant::now();
    let score = negamax::negamax(&pos, history, &mut stats, 2);
    let elapsed = start.elapsed();

    if stats.best_move.is_none() {
        return Err("No bestmove");
    }

    let pv = vec![stats.best_move.unwrap()];

    info_printer(&Info {
        pos,
        depth: Some(stats.depth),
        seldepth: Some(stats.seldepth),
        nodes: Some(stats.nodes),
        score: Some(score),
        mate: None,
        elapsed: Some(elapsed.as_millis()),
        pv,
    });

    Ok(stats.best_move.unwrap())
}
