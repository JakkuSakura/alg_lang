
pub fn error(err: &str) {
    eprintln!("{}", err);
}

pub fn fatal_<T>(err: &str) -> T {
    panic!("{}", err);

}
pub fn fatal(err: &str) {
    panic!("{}", err);
}
pub fn get(s: &str, i: usize) -> char {
    if i >= s.len() {
        return '\u{FFFF}';
    }
    return s.as_bytes()[i] as char;
}

