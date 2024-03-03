use rawr::uci;
use std::io;

fn main() -> io::Result<()> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    match input.trim() {
        "uci" => uci::listen::listen(),
        "quit" => {}
        _ => println!("Unknown protocol"),
    };

    Ok(())
}
