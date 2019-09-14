use super::lexer::*;
use super::util::*;
use std::fmt::{Debug, Error, Formatter};


#[derive(PartialEq, Clone)]
pub struct FuncCall {
    pub func_name: Identifier,
    pub arg_list: Vec<Value>,
}

impl Debug for FuncCall {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.write_str(&self.func_name.0)?;
        //        f.write_str("(")?;
        self.arg_list.fmt(f)?;
        //        f.write_str(")")?;
        return Ok(());
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct ArgDecl(Identifier);

#[derive(PartialEq, Debug, Clone)]
pub struct FuncDecl {
    pub func_name: Identifier,
    pub arg_list: Vec<ArgDecl>,
    pub body: Block,
}

#[derive(PartialEq, Clone)]
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
#[derive(PartialEq, Clone)]
pub enum Value {
    VAR(Identifier),
    FLOAT(f64),
    INT(i32),
    BOOL(bool),
    FUNC_CALL(FuncCall),
}

impl Debug for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Value::VAR(x) => x.fmt(f),
            Value::FLOAT(x) => x.fmt(f),
            Value::FUNC_CALL(x) => x.fmt(f),
            Value::INT(x) => x.fmt(f),
            Value::BOOL(x) => x.fmt(f),
        }
    }
}


#[derive(PartialEq, Debug, Clone)]
pub struct If {
    pub cond: Vec<Value>,
    pub then: Vec<Block>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct While {
    pub cond: Value,
    pub then: Block,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Return(pub Value);

#[allow(non_camel_case_types)]
#[derive(PartialEq, Clone)]
pub enum Statement {
    ASSIGNMENT(Assign),
    RETURN(Return),
    EXPRESSION(Value),
    FUNC_DECL(FuncDecl),
    IF(If),
    WHILE(While),
    NOTHING
}

impl Debug for Statement {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Statement::ASSIGNMENT(x) => x.fmt(f),
            Statement::RETURN(x) => x.fmt(f),
            Statement::EXPRESSION(x) => x.fmt(f),
            Statement::FUNC_DECL(x) => x.fmt(f),
            Statement::IF(x) => x.fmt(f),
            Statement::WHILE(x) => x.fmt(f),
            Statement::NOTHING => f.write_str(";"),
        }
    }
}

#[derive(PartialEq, Clone)]
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

fn try_eat_semicolon(input: &str, pos: usize) -> Option<usize> {
    if let (Token::SEMICOLON, pos) = next_token(input, pos) {
        return Some(pos);
    }
    return None;
}

fn assignment_stmt(input: &str, pos: usize) -> Option<(Assign, usize)> {
    if let (Token::IDENTIFIER(id), pos) = next_token(input, pos) {
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


fn identifier(input: &str, pos: usize) -> Option<(Identifier, usize)> {
    if let (Token::IDENTIFIER(id), p) = next_token(input, pos) {
        return Some((id, p));
    }
    return None;
}

fn func_decl(input: &str, pos: usize) -> Option<(FuncDecl, usize)> {
    if let Some(mut pos) = try_eat_keyword(input, pos, "fn") {
        if let Some((func_name, p)) = identifier(input, pos) {
            pos = p;
            let mut arg_list = vec![];
            if let Some(p) = try_eat_operator(input, pos, "(") {
                pos = p;
                let mut expect_comma = false;
                loop {
                    if !expect_comma {
                        if let (Token::IDENTIFIER(v), p) = next_token(input, pos) {
                            pos = p;
                            arg_list.push(ArgDecl(v));
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
                    fatal(&format!("Error: expect ',' or ')' while trying to parse a function call at {}", pos));
                    break;
                }
            } else {
                fatal(&format!("Error: expect '(' after fn [id] at {}", pos));
            }
            if let Some(pos) = try_eat_operator(input, pos, "{") {
                let (body, pos) = block(input, pos);

                if let Some(pos) = try_eat_operator(input, pos, "}") {
                    return Some((FuncDecl {
                        func_name,
                        arg_list,
                        body,
                    }, pos));
                } else {
                    fatal(&format!("Error: not closing bracket at pos {}", pos));
                }
            }
        } else {
            fatal(&format!("Error: expect identifier after fn at {}", pos));
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
    if let Some((func_name, pos)) = identifier(input, pos) {
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
                error(&format!("Error: expect ',' or ')' while trying to parse a function call at {}", pos));
                break;
            }
            return Some((func, pos));
        }
    }
    return None;
}

fn if_stmt(input: &str, pos: usize) -> Option<(If, usize)> {
    if let Some(mut pos) = try_eat_keyword(input, pos, "if") {
        let mut if_ = If { cond: vec![], then: vec![] };
        let mut flag = true;
        let mut true_cond = false;
        while flag {
            flag = false;
            if !true_cond {
                if let Some((expr, p)) = expression(input, pos) {
                    if_.cond.push(expr);
                    pos = p;
                } else {
                    fatal(&format!("Error: expect expression after if at pos {}", pos));
                }
            } else {
                if_.cond.push(Value::BOOL(true));
            }
            if let Some(p) = try_eat_operator(input, pos, "{") {
                pos = p;
            } else {
                fatal(&format!("Error: expect bracket after \"if cond \" at pos {}", pos))
            }

            let (then, p) = block(input, pos);
            if_.then.push(then);
            pos = p;

            if let Some(p) = try_eat_operator(input, pos, "}") {
                pos = p;
            } else {
                fatal(&format!("Error: expect bracket after \"if cond {} stmts\" at pos {}", "{", pos))
            }
            if let Some(p) = try_eat_keyword(input, pos, "elif") {
                flag = true;
                pos = p;
            } else if let Some(p) = try_eat_keyword(input, pos, "else") {
                flag = true;
                pos = p;
                true_cond = true;
            }
        }
        return Some((if_, pos));
    }
    return None;
}

fn while_stmt(input: &str, pos: usize) -> Option<(While, usize)> {
    if let Some(mut pos) = try_eat_keyword(input, pos, "while") {
        let cond;

        if let Some((expr, p)) = expression(input, pos) {
            cond = expr;
            pos = p;
        } else {
            fatal(&format!("Error: expect expression after if at pos {}", pos));
            panic!();
        }

        if let Some(p) = try_eat_operator(input, pos, "{") {
            pos = p;
        } else {
            fatal(&format!("Error: expect bracket after \"while cond \" at pos {}", pos))
        }

        let (then, p) = block(input, pos);
        pos = p;

        if let Some(p) = try_eat_operator(input, pos, "}") {
            pos = p;
        } else {
            fatal(&format!("Error: expect bracket after \"while cond {} stmts\" at pos {}", "{", pos))
        }

        return Some((While{cond, then}, pos));
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
                    fatal_::<()>(&format!("Error: expect a expression at pos {}", pos));
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
    if let Token::INTEGER(i) = tk {
        return Some((Value::INT(i), pos));
    }
    if let Token::IDENTIFIER(id) = tk {
        return Some((Value::VAR(id), pos));
    }

    return None;
}

fn statement(input: &str, pos: usize) -> Option<(Statement, usize)> {
    if let Some((ass, pos)) = assignment_stmt(input, pos) {
        if let Some(pos) = try_eat_semicolon(input, pos) {
            return Some((Statement::ASSIGNMENT(ass), pos));
        }
    }
    if let Some((rtn, pos)) = return_stmt(input, pos) {
        if let Some(pos) = try_eat_semicolon(input, pos) {
            return Some((Statement::RETURN(rtn), pos));
        }
    }

    if let Some((val, pos)) = value(input, pos) {
        if let Some(pos) = try_eat_semicolon(input, pos) {
            return Some((Statement::EXPRESSION(val), pos));
        }
    }
    if let Some((decl, pos)) = func_decl(input, pos) {
        return Some((Statement::FUNC_DECL(decl), pos));
    }
    if let Some((if_, pos)) = if_stmt(input, pos) {
        return Some((Statement::IF(if_), pos));
    }

    if let Some((while_, pos)) = while_stmt(input, pos) {
        return Some((Statement::WHILE(while_), pos));
    }

    if let Some(pos) = try_eat_semicolon(input, pos) {
        return Some((Statement::NOTHING, pos));
    }
    return None;
}

pub fn block(input: &str, pos: usize) -> (Block, usize) {
    let mut b = Block(vec![]);
    let mut pos = pos;
    loop {

        match statement(input, pos) {
            Some((node, p)) => {
                match node {
                    Statement::NOTHING => {}
                    _ => {
                        b.0.push(node);
                    }
                }
                pos = p;
            }
            None => {
                break;
            }
        }
    }
    return (b, pos);
}


pub fn parse(input: &str, pos: usize) -> Block {
    let (b, pos) = block(input, pos);

    if pos != input.len() {
        error(&format!("Unknown error at: {}", pos));
    }

    return b;
}

// assign: ID = expr
// expr: add
// add: a + b | a - b | multi
// multi: a * b | a / b | value
// value: FLOAT | func_call | ( expr )
// func_decl: fn (arg1, arg2, arg3) -> {blblbl}
// if cond { } else {}
// if cond { }
// if cond { } else if cond {}