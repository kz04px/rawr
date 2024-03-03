use crate::chess::position::Position;
use std::str::SplitAsciiWhitespace;

pub fn moves(stream: &mut SplitAsciiWhitespace, pos: &mut Position, history: &mut Vec<u64>) {
    for movestr in stream.by_ref() {
        let moves = pos.legal_moves();
        let mv = moves.iter().find(|x| x.to_uci(pos) == movestr);
        if let Some(found) = mv {
            pos.makemove::<true>(found);
            history.push(pos.hash);
        }
    }
}
