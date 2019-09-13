
pub fn error(err: &str) {
    eprintln!("{}", err);
}

pub fn fatal<T>(err: &str) -> T {
    eprintln!("{}", err);
    std::process::exit(-1);
}
pub fn get(s: &str, i: usize) -> char {
    if i >= s.len() {
        return '\u{FFFF}';
    }
    return s.as_bytes()[i] as char;
}

