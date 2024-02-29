use rawr::uci;
use std::io;

fn main() -> io::Result<()> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    match input.trim() {
        "uci" => uci::listen::listen(),
        "about" => {
            println!(
                "Rawr version {}",
                option_env!("CARGO_PKG_VERSION").unwrap_or("unknown")
            );
            println!("https://github.com/kz04px/rawr");
            if cfg!(debug_assertions) {
                println!("Debug enabled");
            }
        }
        "quit" => {}
        _ => println!("Unknown protocol"),
    };

    Ok(())
}
