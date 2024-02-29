use crate::chess::{mv::Mv, position::Position};
use crate::search::stats::Stats;

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
pub fn greedy(pos: &Position, history: &[u64], stats: &mut Stats) -> Option<Mv> {
    let mut best_move = Option::<Mv>::default();
    let mut best_score = -1_000_000;

    pos.move_generator(|mv| {
        stats.nodes += 1;
        let npos = pos.after_move::<true>(&mv);
        let is_threefold = history.iter().fold(
            0,
            |acc, hash| if *hash == npos.hash { acc + 1 } else { acc },
        ) >= 2;
        let in_check = npos.in_check();
        let can_move = npos.count_moves() > 0;
        let is_checkmate = in_check && !can_move;
        let is_50move = npos.halfmoves >= 100 && !is_checkmate;
        let is_stalemate = !in_check && !can_move;

        let score = if is_threefold || is_50move || is_stalemate {
            -50
        } else if is_checkmate {
            1_000_000
        } else {
            -eval(&npos)
        };

        if score > best_score {
            best_score = score;
            best_move = Some(mv);
        }
    });

    stats.depth = 1;
    stats.seldepth = 1;

    best_move
}
