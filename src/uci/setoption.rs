use std::str::SplitAsciiWhitespace;

pub fn setoption(stream: &mut SplitAsciiWhitespace, mut func: impl FnMut(&str, &str)) {
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

    match (name, value) {
        (Some(name), Some(value)) => {
            func(name, value);
        }
        _ => {}
    }
}
