use std::str::SplitAsciiWhitespace;

pub fn setoption(stream: &mut SplitAsciiWhitespace, mut func_hash: impl FnMut(usize)) {
    match stream.next() {
        Some("name") => {}
        _ => return,
    }

    let name = stream.next();

    match stream.next() {
        Some("value") => {}
        _ => return,
    }

    let value = stream.next();

    if name.is_none() || value.is_none() {
        return;
    }

    match (name.unwrap(), value.unwrap()) {
        ("Hash" | "hash", v) => {
            if let Ok(size) = v.parse::<usize>() {
                func_hash(size);
            }
        }
        _ => {}
    }
}
