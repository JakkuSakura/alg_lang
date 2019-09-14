use std::fmt::{Debug, Error, Formatter};

use super::util::*;

#[derive(PartialEq, Eq, Clone)]
pub struct Identifier(pub String);

impl Debug for Identifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.write_str("ID#")?;
        f.write_str(&self.0.to_string())?;
        return Ok(());
    }
}

#[derive(PartialEq)]
pub enum Token {
    IDENTIFIER(Identifier),
    OPERATOR(&'static str),
    KEYWORD(&'static str),
    INTEGER(i32),
    FLOAT(f64),
    SEMICOLON,
    ERROR,
    EOF,
}

fn strcmp(input: &str, pos: usize, s: &str) -> bool {
    if pos + s.len() > input.len() {
        return false;
    }

    unsafe {
        return input.get_unchecked(pos..pos + s.len()) == s;
    }
}

const KEYWORDS: [&str; 13] = [
    "for", "if", "while", "loop", "until", "return", "continue", "break", "to", "downto", "fn", "else", "elif"
];
const OPERATORS: [&str; 14] = [
    "+", "-", "*", "/", "=", "**", "[", "]", "(", ")", "{", "}", ",", "->"
];

pub fn next_token(input: &str, pos: usize) -> (Token, usize) {

    let mut pos = pos;
    while get(input, pos) == ' ' || get(input,pos) == '\t' || get(input,pos) == '\n' || get(input,pos) == '\r' {
        pos += 1;
    }
    if pos >= input.len() {
        debug!("match EOF");
        return (Token::EOF, pos);
    }

    if get(input, pos) == ';' {
        debug!("match semicolon");
        return (Token::SEMICOLON, pos + 1);
    }
    for k in KEYWORDS.iter() {
        if strcmp(input, pos, *k) {
            debug!("match keyword");
            return (Token::KEYWORD(k), pos + k.len());
        }
    }
    for k in OPERATORS.iter() {
        if strcmp(input, pos, *k) {
            debug!("match operator");
            return (Token::OPERATOR(k), pos + k.len());
        }
    }
    let ch = get(input, pos);
    if ch.is_alphabetic() || ch == '_' {
        let mut buf = String::new();
        buf.push(ch);
        pos += 1;

        loop {
            let ch = get(input, pos);
            if ch.is_alphanumeric() || ch == '_' {
                buf.push(ch);
                pos += 1;
            } else {
                break;
            }
        }
        debug!("match identifier {}", buf);
        return (Token::IDENTIFIER(Identifier(buf)), pos);
    }
    if ch.is_numeric() {
        let mut n = (ch as u8 - '0' as u8) as f64;
        pos += 1;
        let mut dot = false;
        let mut ex = 0.1;
        loop {
            let ch = get(input, pos);
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
                    error(&format!("warn: repeated dots in the same number at pos {}", pos));
                }
                dot = true;
                pos += 1;
            } else {
                break;
            }
        }
        debug!("match number");
        if dot {
            return (Token::FLOAT(n), pos);
        } else {
            return (Token::INTEGER(n as i32), pos);
        }
    }


    return (Token::ERROR, pos);
}
