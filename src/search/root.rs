use self::hashtable::Hashtable;
use self::info::Info;
use self::negamax::INF;
use self::stats::Stats;
use self::ttentry::TTEntry;
use crate::chess::colour::Colour;
use crate::chess::{mv::Mv, position::Position};
use crate::search::*;
use std::time::Instant;

const MAX_DEPTH: i32 = 128;

#[must_use]
pub fn root(
    pos: Position,
    history: &mut Vec<u64>,
    tt: &mut Hashtable<TTEntry>,
    settings: settings::Type,
    info_printer: fn(&Info),
) -> Result<Mv, &'static str> {
    let start = Instant::now();
    let mut stats = Stats::default();
    let mut best_move = Option::<Mv>::default();
    let should_stop = |stats: &Stats| match settings {
        settings::Type::Time(wtime, btime, _, _, mtg) => {
            let ustime = if pos.get_turn() == Colour::White {
                wtime
            } else {
                btime
            };

            start.elapsed().as_millis() >= (ustime / mtg.unwrap_or(30)) as u128
        }
        settings::Type::Movetime(time) => start.elapsed().as_millis() >= time as u128,
        settings::Type::Depth(d) => stats.depth > d,
        settings::Type::Nodes(n) => stats.nodes >= n,
        settings::Type::Infinite => false,
        settings::Type::Perft(_) => panic!("Uh oh perft"),
        settings::Type::SplitPerft(_) => panic!("Uh oh split perft"),
    };

    for depth in 1..MAX_DEPTH {
        stats.depth = depth;

        let score = negamax::negamax(
            &pos,
            history,
            tt,
            &mut stats,
            &should_stop,
            -INF,
            INF,
            0,
            depth,
            false,
        );
        let elapsed = start.elapsed();

        if stats.best_move.is_none() {
            return Err("No bestmove");
        }

        if depth > 1 && should_stop(&stats) {
            break;
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
            hashfull: tt.hashfull(),
            pv,
        });

        best_move = stats.best_move;
    }

    Ok(best_move.unwrap())
}
