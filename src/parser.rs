use super::lexer::*;
use super::util::*;
use std::fmt::{Debug, Error, Formatter};

pub struct FuncCall {
    func_name: Identifier,
    arg_list: Vec<Value>,
}

impl Debug for FuncCall {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        self.func_name.0.raw_text(f)?;
        //        f.write_str("(")?;
        self.arg_list.fmt(f)?;
        //        f.write_str(")")?;
        return Ok(());
    }
}

#[derive(Debug)]
pub struct ArgDecl();

#[derive(Debug)]
pub struct FuncDecl {
    func_name: Str,
    arg_list: Vec<ArgDecl>,
    body: Block,
}

pub struct Assign {
    pub id: Identifier,
    pub val: Value,
}

impl Debug for Assign {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        self.id.fmt(f)?;
        f.write_str(" = ")?;
        self.val.fmt(f)?;
        return Ok(());
    }
}

pub enum Value {
    VAR(Identifier),
    FLOAT(f64),
    #[allow(non_camel_case_types)]
    FUNC_CALL(FuncCall),
}

impl Debug for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Value::VAR(x) => {
                x.fmt(f)?;
            }
            Value::FLOAT(x) => {
                x.fmt(f)?;
            }
            Value::FUNC_CALL(x) => {
                x.fmt(f)?;
            }
        };
        return Ok(());
    }
}

pub enum Statement {
    ASSIGNMENT(Assign),
}

impl Debug for Statement {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Statement::ASSIGNMENT(x) => {
                x.fmt(f)?;
            }
        };
        return Ok(());
    }
}

pub struct Block(pub Vec<Statement>);

impl Debug for Block {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_str("Block ")?;
        self.0.fmt(f)?;
        return Ok(());
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
            let r = expression(input, pos.unwrap());
            if r.is_some() {
                let (val, pos) = r.unwrap();
                return Some((Assign { id, val }, pos));
            }
        } else {
            return None;
        }
    }
    return None;
}

fn func_call(input: &Str, pos: usize) -> Option<(FuncCall, usize)> {
    let (tk, pos) = next_token(input, pos);
    if let Token::IDENTIFIER(func_name) = tk {
        if let Some(mut pos) = try_eat_operator(input, pos, "(") {
            let mut func = FuncCall {
                func_name,
                arg_list: vec![],
            };
            let mut expect_comma = false;
            loop {
                if !expect_comma {
                    if let Some((v, p)) = expression(input, pos) {
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
                error!(
                    true,
                    "Error: expect ',' or ')' while trying to parse a function call at {}", pos
                );
                break;
            }
            return Some((func, pos));
        }
    }
    return None;
}

fn expression(input: &Str, pos: usize) -> Option<(Value, usize)> {
    return addition_expr(input, pos);
}

fn expr_implementation(
    input: &Str,
    pos: usize,
    oper_list: &[&str],
    fun: fn(&Str, usize) -> Option<(Value, usize)>,
) -> Option<(Value, usize)> {
    let mut flag = true;
    if let Some((mut o1, mut pos)) = fun(input, pos) {
        while flag {
            flag = false;
            for oper in oper_list {
                let r = try_eat_operator(input, pos, oper);
                if r.is_none() {
                    continue;
                }
                flag = true;
                pos = r.unwrap();
                if let Some((o2, p)) = fun(input, pos) {
                    pos = p;
                    let x = Value::FUNC_CALL(FuncCall {
                        func_name: Identifier(Str::from(oper)),
                        arg_list: vec![o1, o2],
                    });
                    o1 = x;
                } else {
                    error!(true, "Error: expect a expression at pos {}", pos);
                }
            }
        }

        return Some((o1, pos));
    }
    return None;
}

fn addition_expr(input: &Str, pos: usize) -> Option<(Value, usize)> {
    const LIST: [&str; 2] = ["+", "-"];
    return expr_implementation(input, pos, &LIST, multiplication_expr);
}

fn multiplication_expr(input: &Str, pos: usize) -> Option<(Value, usize)> {
    const LIST: [&str; 2] = ["*", "/"];
    return expr_implementation(input, pos, &LIST, value);
}

fn value(input: &Str, pos: usize) -> Option<(Value, usize)> {
    if let Some((x, p)) = func_call(input, pos) {
        return Some((Value::FUNC_CALL(x), p));
    }
    if let Some(pos) = try_eat_operator(input, pos, "(") {
        if let Some((v, pos)) = expression(input, pos) {
            if let Some(pos) = try_eat_operator(input, pos, ")") {
                return Some((v, pos));
            } else {
                error!(false, "Error: not closing parenthesis at pos {}", pos);
                return Some((v, pos));
            }
        }
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

fn statement(input: &Str, pos: usize) -> Option<(Statement, usize)> {
    if let Some((ass, pos)) = assign(input, pos) {
        let result = try_eat_newline(input, pos);
        if result.is_some() || pos == input.len() - 1 {
            return Some((Statement::ASSIGNMENT(ass), result.unwrap()));
        }
    }

    return None;
}

pub(crate) fn parse(input: &Str, pos: usize) -> Block {
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
