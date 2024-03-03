use crate::chess::position::Position;
use std::str::SplitAsciiWhitespace;

pub fn position(stream: &mut SplitAsciiWhitespace, pos: &mut Position, history: &mut Vec<u64>) {
    // Parse startpos/fen
    let fen: String = match stream.next() {
        Some("startpos") => {
            stream.next();
            "startpos".to_string()
        }
        Some("fen") => {
            let fen: String = stream
                .take_while(|&part| part != "moves")
                .fold(String::new(), |a, b| a + b + " ");
            fen
        }
        _ => String::new(),
    };

    // Set FEN
    pos.set_fen(fen.trim());

    history.clear();
    history.push(pos.hash);

    // Parse moves
    for movestr in stream.by_ref() {
        let moves = pos.legal_moves();
        let mv = moves.iter().find(|x| x.to_uci(pos).as_str() == movestr);
        if let Some(found) = mv {
            pos.makemove::<true>(found);
            history.push(pos.hash);
        }
    }
}
