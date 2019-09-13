#[macro_use]
extern crate log;

use std::fs::File;
use std::io::Read;

#[macro_use]
pub mod util;

pub mod lexer;

pub mod parser;

pub mod runtime;

extern crate clap;

use clap::{App};

fn main() {
    let matches = App::new("Simple Calculator")
        .version("0.1")
        .author("Jack Quinn")
        .about("A simple calculator but support complex logic")
        .args_from_usage("-i, --input=[FILE] 'source code file'")
        .args_from_usage("-o, --output=[FILE] 'output file'")
        .get_matches();

    if let Some(f) = matches.value_of("path") {
        println!("path : {}", f);
    }
//    let buf = String::from("a = 6*7\n
//b = 5*8\n
//print(a, b)\n
//");
    let mut buf = String::new();
    let mut file = File::open(matches.value_of("input").unwrap_or("/dev/stdin")).expect("Cannot open file");
    file.read_to_string(&mut buf).expect("Cannot read file");
    let v = parser::parse(&buf, 0);
    debug!("{:#?}", v);
    runtime::run_code(&v);
}
// assign: ID = expr
// expr: add
// add: a + b | a - b | multi
// multi: a * b | a / b | value
// value: FLOAT | func_call | ( expr )
