use std::fmt::{Debug, Error, Formatter};

use super::util::*;

#[derive(PartialEq, Eq, Clone)]
pub struct Identifier(pub Str);

impl Debug for Identifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.write_str("ID#")?;
        self.0.raw_text(f)?;
        return Ok(());
    }
}

#[derive(PartialEq)]
pub enum Token {
    IDENTIFIER(Identifier),
    OPERATOR(&'static str),
    KEYWORD(&'static str),
    INTEND(i32),
    FLOAT(f64),
    NEWLINE,
    ERROR,
    EOF,
}

fn strcmp(input: &Str, pos: usize, s: &str) -> bool {
    if pos + s.len() > input.len() {
        return false;
    }

    unsafe {
        return input.0.get_unchecked(pos..pos + s.len()) == s.as_bytes();
    }
}

const KEYWORDS: [&str; 10] = [
    "for", "if", "while", "loop", "until", "return", "continue", "break", "to", "downto",
];
const OPERATORS: [&str; 13] = [
    "+", "-", "*", "/", "=", "**", "[", "]", "(", ")", "{", "}", ",",
];

pub fn next_token(input: &Str, pos: usize) -> (Token, usize) {
    if input.get(pos) == '\n' {
        println!("match new line");
        return (Token::NEWLINE, pos + 1);
    }
    let mut pos = pos;
    while input.get(pos) == ' ' || input.get(pos) == '\t' {
        pos += 1;
    }
    for k in KEYWORDS.iter() {
        if strcmp(input, pos, *k) {
            println!("match keyword");
            return (Token::KEYWORD(k), pos + k.len());
        }
    }
    for k in OPERATORS.iter() {
        if strcmp(input, pos, *k) {
            println!("match operator");
            return (Token::OPERATOR(k), pos + k.len());
        }
    }
    let ch = input.get(pos);
    if ch.is_alphabetic() || ch == '_' {
        let mut buf = Str::new();
        buf.push(ch);
        pos += 1;

        loop {
            let ch = input.get(pos);
            if ch.is_alphanumeric() || ch == '_' {
                buf.push(ch);
                pos += 1;
            } else {
                break;
            }
        }
        println!(
            "match identifier {}",
            String::from_utf8(buf.0.clone()).unwrap()
        );
        return (Token::IDENTIFIER(Identifier(buf)), pos);
    }
    if ch.is_numeric() {
        let mut n = (ch as u8 - '0' as u8) as f64;
        pos += 1;
        let mut dot = false;
        let mut ex = 0.1;
        loop {
            let ch = input.get(pos);
            if ch.is_numeric() {
                if dot {
                    n += ex * (ch as u8 - '0' as u8) as f64;
                    ex *= 0.1;
                } else {
                    n = n * 10.0 + (ch as u8 - '0' as u8) as f64;
                }
                pos += 1;
            } else if ch == '.' {
                if dot {
                    error!(
                        false,
                        "warn: repeated dots in the same number at pos {}", pos
                    );
                }
                dot = true;
                pos += 1;
            } else {
                break;
            }
        }
        println!("match number");
        return (Token::FLOAT(n), pos);
    }

    println!("match EOF");
    return (Token::EOF, pos);
}
