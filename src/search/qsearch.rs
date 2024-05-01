use crate::chess::{mv::Mv, position::Position};
use crate::search::eval::eval;
use crate::search::stats::Stats;

pub const INF: i32 = 10_000_000;

fn sort(pos: &Position, moves: &mut Vec<Mv>) {
    let piece_values = [100, 300, 325, 500, 900, 0];
    let mut scores = [0; 218];

    if moves.len() < 2 {
        return;
    }

    // Score
    for i in 0..moves.len() {
        let captured = pos.get_piece_on(moves[i].to);
        let piece = pos.get_piece_on(moves[i].from);

        if let Some(captured) = captured {
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
pub fn qsearch(pos: &Position, stats: &mut Stats, mut alpha: i32, beta: i32, ply: i32) -> i32 {
    debug_assert!(-INF <= alpha);
    debug_assert!(alpha < beta);
    debug_assert!(beta <= INF);

    let stand_pat = eval(&pos);
    stats.seldepth = std::cmp::max(stats.seldepth, ply);

    if stand_pat >= beta {
        return stand_pat;
    }

    if stand_pat > alpha {
        alpha = stand_pat;
    }

    let mut best_score = stand_pat;
    let mut moves = pos.legal_captures();
    sort(&pos, &mut moves);

    for mv in moves {
        stats.nodes += 1;

        let npos = pos.after_move::<false>(&mv);
        let score = -qsearch(&npos, stats, -beta, -alpha, ply + 1);

        if score > best_score {
            best_score = score;
        }

        if score > alpha {
            alpha = score;
        }

        if alpha >= beta {
            break;
        }
    }

    best_score
}
