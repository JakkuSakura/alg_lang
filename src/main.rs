use crate::util::Str;

#[macro_use]
pub mod util;

pub mod lexer;

pub mod parser;

pub mod runtime;

fn main() {
    let buf = String::from("b = 2 * 3 * (4 - 6)\n");
    //    let mut file = File::open("/dev/stdin").expect("Cannot open file");
    //    file.read_to_string(&mut buf).expect("Cannot read file");
    let input = Str::from_moved_str(buf);
    let v = parser::parse(&input, 0);
    println!("{:#?}", v);
    runtime::run_code(&v);
}
// assign: ID = expr
// expr: add
// add: a + b | a - b | multi
// multi: a * b | a / b | value
// value: FLOAT | func_call | ( expr )
