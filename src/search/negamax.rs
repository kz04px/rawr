use crate::chess::{mv::Mv, position::Position};
use crate::search::stats::Stats;

const INF: i32 = 10_000_000;
const MATE_SCORE: i32 = 1_000_000;
const DRAW_SCORE: i32 = -50;

#[must_use]
fn eval(pos: &Position) -> i32 {
    let mut score = 0;

    // Us
    score += 100 * (pos.get_pawns() & pos.get_us()).count();
    score += 300 * (pos.get_knights() & pos.get_us()).count();
    score += 320 * (pos.get_bishops() & pos.get_us()).count();
    score += 500 * (pos.get_rooks() & pos.get_us()).count();
    score += 900 * (pos.get_queens() & pos.get_us()).count();
    // Them
    score -= 100 * (pos.get_pawns() & pos.get_them()).count();
    score -= 300 * (pos.get_knights() & pos.get_them()).count();
    score -= 320 * (pos.get_bishops() & pos.get_them()).count();
    score -= 500 * (pos.get_rooks() & pos.get_them()).count();
    score -= 900 * (pos.get_queens() & pos.get_them()).count();

    score
}

#[must_use]
pub fn negamax(pos: &Position, history: &mut Vec<u64>, stats: &mut Stats, depth: i32) -> i32 {
    if depth <= 0 {
        return eval(pos);
    }

    let mut best_move = Option::<Mv>::default();
    let mut best_score = -INF;
    let is_50move = pos.halfmoves >= 100;
    let is_threefold = history
        .iter()
        .fold(0, |acc, hash| if *hash == pos.hash { acc + 1 } else { acc })
        >= 3;

    if is_50move || is_threefold {
        return DRAW_SCORE;
    }

    for mv in pos.legal_moves() {
        stats.nodes += 1;
        let npos = pos.after_move::<true>(&mv);
        history.push(npos.hash);

        let score = -negamax(&npos, history, stats, depth - 1);
        history.pop();

        if score > best_score {
            best_score = score;
            best_move = Some(mv);
        }
    }

    if best_move.is_none() {
        if pos.in_check() {
            return -MATE_SCORE;
        } else {
            return DRAW_SCORE;
        }
    }

    debug_assert!(best_move.is_some());
    debug_assert!(best_score > -INF);
    debug_assert!(best_score >= -MATE_SCORE);

    stats.best_move = best_move;

    best_score
}
