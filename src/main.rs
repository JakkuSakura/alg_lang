use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::fmt::{Debug, Formatter, Error, Write};
use std::process::exit;
use crate::Token::IDENTIFIER;

macro_rules! error {
    ($fatal:expr, $($arg:tt)*) => {
        eprintln!($($arg)*);
        if $fatal {
            exit(-1)
        };
    }
}


#[derive(PartialEq)]
struct Str(Vec<u8>);
impl Str {
    fn raw_text(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_str(unsafe { String::from_utf8_unchecked(self.0.clone()) }.as_str())?;
        return Ok(());
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
    fn new() -> Str {
        Str(vec![])
    }
    fn new_with_cap(s: usize) -> Str {
        Str(Vec::with_capacity(s))
    }
    fn from(s: &str) -> Str {
        Str(s.as_bytes().to_vec())
    }
    fn from_moved_str(str: String) -> Str {
        Str(str.into_bytes())
    }
    fn get(&self, i: usize) -> char {
        let Str(x) = self;
        if i >= x.len() { return '\u{FFFF}'; }
        return x[i] as char;
    }
    fn len(&self) -> usize {
        let Str(x) = self;
        return x.len();
    }
    fn push(&mut self, c: char) {
        self.0.push(c as u8);
    }
    fn as_mut_slice(&mut self) -> &mut [u8] {
        self.0.as_mut_slice()
    }
    fn as_mut(&self) -> &[u8] {
        self.0.as_slice()
    }
}

#[derive(PartialEq)]
enum Token {
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

const KEYWORDS: [&str; 10] = ["for", "if", "while", "loop", "until", "return",
    "continue", "break", "to", "downto"];
const OPERATORS: [&str; 13] = ["+", "-", "*", "/", "=", "**", "[", "]", "(", ")", "{", "}", ","];

fn next_token(input: &Str, pos: usize) -> (Token, usize) {
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
            if ch.is_alphanumeric() || ch == '_'
            {
                buf.push(ch);
                pos += 1;
            } else {
                break;
            }
        }
        println!("match identifier {}", String::from_utf8(buf.0.clone()).unwrap());
        return (Token::IDENTIFIER(Identifier(buf)), pos);
    }
    if ch.is_numeric() {
        let mut n = (ch as u8 - '0' as u8) as f64;
        pos += 1;
        let mut dot = false;
        let mut ex = 0.1;
        loop {
            let ch = input.get(pos);
            if ch.is_numeric()
            {
                if dot {
                    n += ex * (ch as u8 - '0' as u8) as f64;
                    ex *= 0.1;
                } else {
                    n = n * 10.0 + (ch as u8 - '0' as u8) as f64;
                }
                pos += 1;
            } else if ch == '.' {
                if dot {
                    eprintln!("warn: repeated dots in the same number");
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

trait Node: Debug {}

#[derive(PartialEq)]
struct Identifier(Str);

impl Debug for Identifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.write_str("ID#")?;
        self.0.raw_text(f)?;
        return Ok(());
    }
}

#[derive(Debug)]
struct FuncCall { func_name: Identifier, arg_list: Vec<Value> }

impl Node for FuncCall {}

#[derive(Debug)]
struct FuncDecl {
    func_name: Str,
    arg_list: Vec<Box<dyn Node>>,
    action: Vec<Box<dyn Node>>,
}

impl Node for FuncDecl {}

#[derive(Debug)]
struct Assign {
    id: Identifier,
    val: Value,
}


#[derive(Debug)]
enum Value {
    NODE(Box<dyn Node>),
    VAR(Identifier),
    FLOAT(f64),
    FUNC_CALL(FuncCall),
}


impl Node for Assign {}

struct Block(Vec<Box<dyn Node>>);

impl Node for Block {}

impl Debug for Block {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_str("Block ");
        self.0.fmt(f)
    }
}

fn try_eat_operator(input: &Str, pos: usize, operator: &str) -> Option<usize> {
    let (tk, pos) = next_token(input, pos);
    if let Token::OPERATOR(s) = tk {
        if s == operator {
            return Some(pos);
        }
    }
    return None;
}

fn try_eat_newline(input: &Str, pos: usize) -> Option<usize> {
    let (tk, pos) = next_token(input, pos);
    if Token::NEWLINE == tk {
        return Some(pos);
    }
    return None;
}

fn assign(input: &Str, pos: usize) -> Option<(Assign, usize)> {
    let (tk, pos) = next_token(input, pos);
    if let Token::IDENTIFIER(id) = tk {
        let pos = try_eat_operator(input, pos, "=");
        if pos.is_some() {
            let r = value(input, pos.unwrap());
            if r.is_some() {
                let (val, pos) = r.unwrap();
                return Some((Assign { id, val }, pos));
            }
        } else { return None; }
    }
    return None;
}

fn func_call(input: &Str, pos: usize) -> Option<(FuncCall, usize)> {
    let (tk, pos) = next_token(input, pos);
    if let Token::IDENTIFIER(func_name) = tk {
        if let Some(mut pos) = try_eat_operator(input, pos, "(") {
            let mut func = FuncCall { func_name, arg_list: vec![] };
            let mut expect_comma = false;
            loop {
                if !expect_comma {
                    if let Some((v, p)) = value(input, pos) {
                        pos = p;
                        func.arg_list.push(v);
                        expect_comma = true;
                        continue;
                    }
                } else {
                    if let Some(p) = try_eat_operator(input, pos, ",") {
                        pos = p;
                        expect_comma = false;
                        continue;
                    }
                }
                if let Some(p) = try_eat_operator(input, pos, ")") {
                    pos = p;
                    break;
                }
                error!(true, "Error: expect ',' or ')' while trying to parse a function call at {}", pos);
                break;
            }
            return Some((func, pos));
        }
    }
    return None;
}

fn value(input: &Str, pos: usize) -> Option<(Value, usize)> {
    if let Some((x, p)) = func_call(input, pos) {
        return Some((Value::FUNC_CALL(x), p));
    }

    let (tk, pos) = next_token(input, pos);
    if let Token::FLOAT(f) = tk {
        return Some((Value::FLOAT(f), pos));
    }
    if let Token::IDENTIFIER(id) = tk {
        return Some((Value::VAR(id), pos));
    }

    return None;
}


fn statement(input: &Str, pos: usize) -> Option<(Box<dyn Node>, usize)> {
    if let Some((ass, pos)) = assign(input, pos) {
        let result = try_eat_newline(input, pos);
        if result.is_some() || pos == input.len() - 1 {
            return Some((Box::new(ass), result.unwrap()));
        }
    }

    return None;
}

fn parse(input: &Str, pos: usize) -> Block {
    let mut b = Block(vec![]);
    let mut pos = pos;
    loop {
        match statement(input, pos) {
            Some((node, p)) => {
                b.0.push(node);
                pos = p;
            }
            None => {
                if pos != input.len() {
                    error!(false, "Unknown error at: {}", pos);
                }
                break;
            }
        }
    }
    return b;
}

fn main() {
    let mut buf = String::from("a = 1\nb = 2\nc = a\nd=foo(a,b,2333)\n");
//    let mut file = File::open("/dev/stdin").expect("Cannot open file");
//    file.read_to_string(&mut buf).expect("Cannot read file");
    let mut input = Str::from_moved_str(buf);
    let v = parse(&input, 0);
    println!("{:#?}", v);
}
// assign: ID = value
// value: FLOAT | func_call | add
