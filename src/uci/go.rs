use super::perft;
use super::split;
use crate::chess::position::Position;
use crate::search::info::Info;
use crate::search::root;
use crate::search::settings;
use std::str::SplitAsciiWhitespace;

fn info_printer(info: &Info) {
    print!("info");
    if let Some(d) = info.depth {
        print!(" depth {d}");
    }
    if let Some(d) = info.seldepth {
        print!(" seldepth {d}");
    }
    if let Some(s) = info.score {
        print!(" score cp {s}");
    }
    if let Some(s) = info.mate {
        print!(" score mate {s}");
    }
    if let Some(n) = info.nodes {
        print!(" nodes {n}");
    }
    if let Some(t) = info.elapsed {
        print!(" time {t}");
    }
    if let (Some(t), Some(n)) = (info.elapsed, info.nodes) {
        if t > 0 {
            print!(" nps {}", (n as u128 * 1000) / t);
        }
    }
    if !info.pv.is_empty() {
        print!(" pv");
        for mv in &info.pv {
            print!(" {}", mv.to_uci(&info.pos))
        }
    }
    println!();
}

#[must_use]
pub fn parse_go(stream: &mut SplitAsciiWhitespace) -> Result<settings::Type, &'static str> {
    let mut wtime = None;
    let mut btime = None;
    let mut winc = None;
    let mut binc = None;
    let mut movestogo = None;
    let mut depth = None;
    let mut nodes = None;
    let mut movetime = None;
    let mut infinite = None;
    let mut perft = None;
    let mut split = None;

    loop {
        match (
            stream.next().unwrap_or_default(),
            stream.next().unwrap_or_default(),
        ) {
            ("wtime", n) => wtime = n.parse::<u32>().ok(),
            ("btime", n) => btime = n.parse::<u32>().ok(),
            ("winc", n) => winc = n.parse::<u32>().ok(),
            ("binc", n) => binc = n.parse::<u32>().ok(),
            ("movestogo", n) => movestogo = n.parse::<u32>().ok(),
            ("depth", n) => depth = n.parse::<i32>().ok(),
            ("nodes", n) => nodes = n.parse::<u64>().ok(),
            ("movetime", n) => movetime = n.parse::<u32>().ok(),
            ("infinite", _) => infinite = Some(true),
            ("perft", n) => perft = n.parse::<u8>().ok(),
            ("split", n) => split = n.parse::<u8>().ok(),
            ("", _) => break,
            (_, _) => return Err("Uh oh"),
        }
    }

    match (wtime, btime, depth, nodes, movetime, infinite, perft, split) {
        (Some(wt), Some(bt), None, None, None, None, None, None) => {
            Ok(settings::Type::Time(wt, bt, winc, binc, movestogo))
        }
        (None, None, Some(d), None, None, None, None, None) => Ok(settings::Type::Depth(d)),
        (None, None, None, Some(n), None, None, None, None) => Ok(settings::Type::Nodes(n)),
        (None, None, None, None, Some(m), None, None, None) => Ok(settings::Type::Movetime(m)),
        (None, None, None, None, None, Some(_), None, None) => Ok(settings::Type::Infinite),
        (None, None, None, None, None, None, Some(d), None) => Ok(settings::Type::Perft(d)),
        (None, None, None, None, None, None, None, Some(d)) => Ok(settings::Type::SplitPerft(d)),
        _ => Err("Uh oh"),
    }
}

pub fn go(stream: &mut SplitAsciiWhitespace, pos: &mut Position, history: &mut Vec<u64>) {
    let opts = parse_go(stream);
    if opts.is_err() {
        return;
    }

    match opts.as_ref().unwrap() {
        settings::Type::Time(_, _, _, _, _)
        | settings::Type::Depth(_)
        | settings::Type::Nodes(_)
        | settings::Type::Movetime(_)
        | settings::Type::Infinite => {
            let bestmove = root::root(*pos, history, opts.unwrap(), info_printer);
            match bestmove {
                Ok(mv) => println!("bestmove {}", mv.to_uci(pos)),
                Err(_) => println!("bestmove 0000"),
            }
        }
        settings::Type::Perft(depth) => perft::perft(pos, *depth),
        settings::Type::SplitPerft(depth) => split::split(pos, *depth),
    }
}
