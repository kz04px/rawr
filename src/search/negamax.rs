use crate::chess::{mv::Mv, position::Position};
use crate::search::hashtable::Hashtable;
use crate::search::qsearch::qsearch;
use crate::search::stats::Stats;
use crate::search::ttentry::Flag;
use crate::search::ttentry::TTEntry;

pub const INF: i32 = 10_000_000;
const MATE_SCORE: i32 = 1_000_000;
const DRAW_SCORE: i32 = -50;

fn sort(pos: &Position, moves: &mut Vec<Mv>, ttmove: &Option<Mv>) {
    let piece_values = [100, 300, 325, 500, 900, 0];
    let mut scores = [0; 218];

    if moves.len() < 2 {
        return;
    }

    // Score
    for i in 0..moves.len() {
        let captured = pos.get_piece_on(moves[i].to);
        let piece = pos.get_piece_on(moves[i].from);

        if ttmove.is_some() && ttmove.unwrap() == moves[i] {
            scores[i] = 1_000_000;
        } else if let Some(captured) = captured {
            scores[i] =
                10 * piece_values[captured as usize] - piece_values[piece.unwrap() as usize];
        } else {
            scores[i] = 0;
        }
    }

    // Sort
    for i in 0..moves.len() - 1 {
        let mut best = i;

        for j in i + 1..moves.len() {
            if scores[j] > scores[best] {
                best = j;
            }
        }

        (moves[i], moves[best]) = (moves[best], moves[i]);
        (scores[i], scores[best]) = (scores[best], scores[i]);
    }
}

#[must_use]
pub fn negamax(
    pos: &Position,
    history: &mut Vec<u64>,
    tt: &mut Hashtable<TTEntry>,
    stats: &mut Stats,
    should_stop: &impl Fn(&Stats) -> bool,
    mut alpha: i32,
    beta: i32,
    ply: i32,
    mut depth: i32,
) -> i32 {
    debug_assert!(-INF <= alpha);
    debug_assert!(alpha < beta);
    debug_assert!(beta <= INF);

    let alpha_orig = alpha;
    let in_check = pos.in_check();
    let is_root = ply == 0;

    stats.seldepth = std::cmp::max(stats.seldepth, ply);

    // Check extensions
    if in_check {
        depth += 1;
    }

    // Poll the TT
    let ttentry = tt.poll(pos.hash);
    let mut ttmove = None;
    if ttentry.hash == pos.hash {
        ttmove = Some(ttentry.mv);
    }

    if depth <= 0 {
        return qsearch(pos, stats, alpha, beta, ply);
    }

    if should_stop(stats) {
        return 0;
    }

    let mut best_move = Option::<Mv>::default();
    let mut best_score = -INF;
    let is_50move = pos.halfmoves >= 100;
    let is_threefold = history
        .iter()
        .rev()
        .take(pos.halfmoves as usize)
        .step_by(2)
        .filter(|hash| **hash == pos.hash)
        .count()
        >= if is_root { 3 } else { 2 };

    if is_50move || is_threefold {
        return DRAW_SCORE;
    }

    let mut moves = pos.legal_moves();
    sort(&pos, &mut moves, &ttmove);

    for mv in moves {
        stats.nodes += 1;
        let npos = pos.after_move::<true>(&mv);
        history.push(npos.hash);

        let score = -negamax(
            &npos,
            history,
            tt,
            stats,
            should_stop,
            -beta,
            -alpha,
            ply + 1,
            depth - 1,
        );
        history.pop();

        if score > best_score {
            best_score = score;
            best_move = Some(mv);
        }

        if score > alpha {
            alpha = score;
        }

        if alpha >= beta {
            break;
        }
    }

    if best_move.is_none() {
        if in_check {
            return -MATE_SCORE + ply;
        } else {
            return DRAW_SCORE;
        }
    }

    debug_assert!(best_move.is_some());
    debug_assert!(best_score > -INF);
    debug_assert!(best_score >= -MATE_SCORE);

    // Create TT entry
    let flag = if best_score <= alpha_orig {
        Flag::Upper
    } else if best_score >= beta {
        Flag::Lower
    } else {
        Flag::Exact
    };
    let new_entry = TTEntry {
        hash: pos.hash,
        mv: best_move.unwrap(),
        score: best_score,
        depth,
        flag,
    };
    tt.add(pos.hash, &new_entry);
    debug_assert_eq!(new_entry, tt.poll(pos.hash));

    // Update PV
    stats.best_move = best_move;

    best_score
}
