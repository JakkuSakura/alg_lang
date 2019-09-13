use super::lexer::*;
use super::util::*;
use std::fmt::{Debug, Error, Formatter};

#[derive(Clone)]

pub struct FuncCall {
    pub func_name: Identifier,
    pub arg_list: Vec<Value>,
}

impl Debug for FuncCall {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.write_str(&self.func_name.0.to_string())?;
        //        f.write_str("(")?;
        self.arg_list.fmt(f)?;
        //        f.write_str(")")?;
        return Ok(());
    }
}

#[derive(Debug, Clone)]
pub struct ArgDecl(String);

#[derive(Debug, Clone)]
pub struct FuncDecl {
    pub func_name: String,
    pub arg_list: Vec<ArgDecl>,
    pub body: Block,
}

#[derive(Clone)]
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

#[allow(non_camel_case_types)]
#[derive(Clone)]
pub enum Value {
    VAR(Identifier),
    FLOAT(f64),
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

#[derive(Debug, Clone)]
pub struct Return(pub Value);

#[derive(Clone)]
pub enum Statement {
    ASSIGNMENT(Assign),
    RETURN(Return),
    EXPRESSION(Value)
}

impl Debug for Statement {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Statement::ASSIGNMENT(x) => x.fmt(f),
            Statement::RETURN(x) => x.fmt(f),
            Statement::EXPRESSION(x) => x.fmt(f),
        }
    }
}

#[derive(Clone)]
pub struct Block(pub Vec<Statement>);

impl Debug for Block {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_str("Block ")?;
        self.0.fmt(f)?;
        return Ok(());
    }
}

fn try_eat_keyword(input: &str, pos: usize, keyword: &str) -> Option<usize> {
    let (tk, pos) = next_token(input, pos);
    if let Token::KEYWORD(s) = tk {
        if s == keyword {
            return Some(pos);
        }
    }
    return None;
}


fn try_eat_operator(input: &str, pos: usize, operator: &str) -> Option<usize> {
    let (tk, pos) = next_token(input, pos);
    if let Token::OPERATOR(s) = tk {
        if s == operator {
            return Some(pos);
        }
    }
    return None;
}

fn try_eat_blank(input: &str, pos: usize) -> Option<usize> {
    let (tk, pos) = next_token(input, pos);
    if Token::NEWLINE == tk {
        return Some(pos);
    }
    return None;
}

fn try_eat_newline(input: &str, pos: usize) -> Option<usize> {
    let (tk, pos) = next_token(input, pos);
    if Token::NEWLINE == tk {
        return Some(pos);
    } else if Token::EOF == tk {
        return Some(pos);
    }
    return None;
}

fn assignment_stmt(input: &str, pos: usize) -> Option<(Assign, usize)> {
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

fn return_stmt(input: &str, pos: usize) -> Option<(Return, usize)> {
    if let Some(pos) = try_eat_keyword(input, pos, "return") {
        if let Some((v, pos)) = expression(input, pos) {
            return Some((Return(v), pos));
        }
    }
    return None;
}

fn func_call(input: &str, pos: usize) -> Option<(FuncCall, usize)> {
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
                fatal::<()>(&format!("Error: expect ',' or ')' while trying to parse a function call at {}", pos));
                break;
            }
            return Some((func, pos));
        }
    }
    return None;
}

fn expression(input: &str, pos: usize) -> Option<(Value, usize)> {
    return addition_expr(input, pos);
}

fn expr_implementation(
    input: &str,
    pos: usize,
    oper_list: &[&str],
    fun: fn(&str, usize) -> Option<(Value, usize)>,
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
                        func_name: Identifier(oper.to_string()),
                        arg_list: vec![o1, o2],
                    });
                    o1 = x;
                } else {
                    fatal::<()>(&format!("Error: expect a expression at pos {}", pos));
                }
            }
        }

        return Some((o1, pos));
    }
    return None;
}

fn addition_expr(input: &str, pos: usize) -> Option<(Value, usize)> {
    const LIST: [&str; 2] = ["+", "-"];
    return expr_implementation(input, pos, &LIST, multiplication_expr);
}

fn multiplication_expr(input: &str, pos: usize) -> Option<(Value, usize)> {
    const LIST: [&str; 2] = ["*", "/"];
    return expr_implementation(input, pos, &LIST, value);
}

fn value(input: &str, pos: usize) -> Option<(Value, usize)> {
    if let Some((x, p)) = func_call(input, pos) {
        return Some((Value::FUNC_CALL(x), p));
    }
    if let Some(pos) = try_eat_operator(input, pos, "(") {
        if let Some((v, pos)) = expression(input, pos) {
            if let Some(pos) = try_eat_operator(input, pos, ")") {
                return Some((v, pos));
            } else {
                error(&format!("Error: not closing parenthesis at pos {}", pos));
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

fn statement(input: &str, pos: usize) -> Option<(Statement, usize)> {

    if let Some((ass, pos)) = assignment_stmt(input, pos) {
        if let Some(pos) = try_eat_newline(input, pos) {
            return Some((Statement::ASSIGNMENT(ass), pos));
        }
    }
    if let Some((rtn, pos)) = return_stmt(input, pos) {
        if let Some(pos) = try_eat_newline(input, pos) {
            return Some((Statement::RETURN(rtn), pos));
        }
    }

    if let Some((val, pos)) = value(input, pos) {
        if let Some(pos) = try_eat_newline(input, pos) {
            return Some((Statement::EXPRESSION(val), pos));
        }
    }


    return None;
}

pub fn parse(input: &str, pos: usize) -> Block {
    let mut b = Block(vec![]);
    let mut pos = pos;
    loop {
        while let Some(p) = try_eat_blank(input, pos) {
            pos = p;
        }

        match statement(input, pos) {
            Some((node, p)) => {
                b.0.push(node);
                pos = p;
            }
            None => {
                if pos != input.len() {
                    error(&format!("Unknown error at: {}", pos));
                }
                break;
            }
        }
    }
    return b;
}
