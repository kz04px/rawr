use self::info::Info;
use self::stats::Stats;
use crate::chess::{mv::Mv, position::Position};
use crate::search::*;
use std::time::Instant;

#[must_use]
pub fn root(
    pos: Position,
    history: &[u64],
    _settings: settings::Type,
    info_printer: fn(&Info),
) -> Result<Mv, &'static str> {
    let mut stats = Stats::default();
    let start = Instant::now();
    let mv = greedy::greedy(&pos, &history, &mut stats);
    let elapsed = start.elapsed();

    if mv.is_none() {
        return Err("No bestmove");
    }

    let pv = vec![mv.unwrap()];

    info_printer(&Info {
        pos,
        depth: Some(stats.depth),
        seldepth: Some(stats.seldepth),
        nodes: Some(stats.nodes),
        score: None,
        mate: None,
        elapsed: Some(elapsed.as_millis()),
        pv,
    });

    Ok(mv.unwrap())
}
