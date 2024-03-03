use crate::search::eval::eval;
use crate::uci::{go, position};
use crate::{chess::position::Position, uci::moves};

pub fn listen() {
    println!("id name Rawr 0.8.1");
    println!("id author kz04px");
    println!("uciok");

    let mut pos = Position::from_fen("startpos");
    let mut history = vec![pos.hash];

    loop {
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(0) => break,
            Ok(_) => {}
            Err(_) => break,
        }

        let mut stream = input.split_ascii_whitespace();
        match stream.next().unwrap_or("") {
            "ucinewgame" => {
                pos = Position::startpos();
                history.clear();
                history.push(pos.hash);
            }
            "isready" => println!("readyok"),
            "print" | "display" | "board" => print!("{pos}"),
            "go" => go::go(&mut stream, &mut pos, &mut history),
            "position" => position::position(&mut stream, &mut pos, &mut history),
            "moves" => moves::moves(&mut stream, &mut pos, &mut history),
            "history" => history.iter().for_each(|hash| println!("{:#x}", hash)),
            "eval" => println!("{}", eval(&pos)),
            "quit" => break,
            _ => {}
        }
    }
}
