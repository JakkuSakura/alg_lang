use std::fmt::{Debug, Error, Formatter, Write};
use std::hash::{Hash, Hasher};

#[macro_export]
macro_rules! error {
    ($fatal:expr, $($arg:tt)*) => {
        eprintln!($($arg)*);
        if $fatal {
            std::process::exit(-1)
        };
    }
}

#[derive(PartialEq, Eq, Clone)]
pub struct Str(pub Vec<u8>);

impl Str {
    pub fn raw_text(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_str(unsafe { String::from_utf8_unchecked(self.0.clone()) }.as_str())?;
        return Ok(());
    }
}
impl Hash for Str {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl Debug for Str {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_char('"')?;
        self.raw_text(f)?;
        f.write_char('"')?;
        return Result::Ok(());
    }
}

impl Str {
    pub fn new() -> Str {
        Str(vec![])
    }
    pub fn new_with_cap(s: usize) -> Str {
        Str(Vec::with_capacity(s))
    }
    pub fn from(s: &str) -> Str {
        Str(s.as_bytes().to_vec())
    }
    pub fn from_moved_str(str: String) -> Str {
        Str(str.into_bytes())
    }
    pub fn get(&self, i: usize) -> char {
        let Str(x) = self;
        if i >= x.len() {
            return '\u{FFFF}';
        }
        return x[i] as char;
    }
    pub fn len(&self) -> usize {
        let Str(x) = self;
        return x.len();
    }
    pub fn push(&mut self, c: char) {
        self.0.push(c as u8);
    }
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        self.0.as_mut_slice()
    }
    pub fn as_mut(&self) -> &[u8] {
        self.0.as_slice()
    }
}
