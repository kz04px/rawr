use crate::chess::{mv::Mv, position::Position};
use crate::search::stats::Stats;
use rand::seq::SliceRandom;

#[must_use]
pub fn random(pos: &Position, stats: &mut Stats) -> Option<Mv> {
    let moves = pos.legal_moves();
    let mv = moves.choose(&mut rand::thread_rng());

    stats.depth = 1;
    stats.seldepth = 1;
    stats.nodes = 1;

    mv.copied()
}
