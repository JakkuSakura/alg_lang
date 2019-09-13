use super::parser::*;

use std::collections::HashMap;
use crate::runtime::PrimitiveType::VOID;
use crate::util::fatal_;
use std::fmt::{Display, Formatter, Error};

type Str = String;

#[derive(Clone)]
pub struct BuiltInFunc {
    execute: fn(&Scope, &FuncCall) -> PrimitiveType
}

#[derive(Clone)]
pub enum PrimitiveType {
    F64(f64),
    I32(i32),
    FUNCTION(FuncDecl),
    BUILTIN(BuiltInFunc),
    VOID,
}

impl Display for PrimitiveType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            PrimitiveType::F64(x) => { std::fmt::Display::fmt(x, f) }
            PrimitiveType::I32(x) => { std::fmt::Display::fmt(x, f) }
            PrimitiveType::FUNCTION(_) => { unimplemented!() }
            PrimitiveType::BUILTIN(_) => { unimplemented!() }
            VOID => { f.write_str("void") }
        }
    }
}

pub struct Scope<'a> {
    pub parent: Option<&'a Scope<'a>>,
    pub local: HashMap<Str, PrimitiveType>,
}

impl<'a> Scope<'a> {
    fn lookup(&self, name: &Str) -> PrimitiveType {
        debug!("Looking for {}", name);
        self.try_lookup(name).expect(&format!("Error because no value called {} at this scope", name))
    }
    fn try_lookup(&self, name: &Str) -> Option<PrimitiveType> {
        match self.local.get(name) {
            None => { self.parent.and_then(|x| x.try_lookup(name)) }
            Some(x) => { Some(x.clone()) }
        }
    }
}

fn get_value(scope: &Scope, v: &Value) -> PrimitiveType {
    match v {
        Value::VAR(id) => { scope.lookup(&id.0.to_string()) }
        Value::FLOAT(f) => { PrimitiveType::F64(*f) }
        Value::FUNC_CALL(fc) => {
            match scope.lookup(&fc.func_name.0.to_string()) {
                PrimitiveType::FUNCTION(f) => {
                    return run_block(Some(&scope), &f.body);
                }
                PrimitiveType::BUILTIN(builtin) => {
                    return (builtin.execute)(scope, fc);
                }
                _ => {
                    fatal_(&format!("Error: {} is not a function or built-in function", fc.func_name.0))
                }
            }
        }
    }
}

pub fn run_block<'a>(parent_scope: Option<&'a Scope>, blk: &Block) -> PrimitiveType {
    let mut scope = Scope {
        parent: parent_scope,
        local: HashMap::new(),
    };

    for statement in blk.0.iter() {
        match statement {
            Statement::ASSIGNMENT(ass) => {
                scope.local.insert(ass.id.0.to_string(), get_value(&scope, &ass.val));
            }
            Statement::RETURN(Return(value)) => {
                return get_value(&scope, value);
            }
            Statement::EXPRESSION(exp) => {
                get_value(&scope, exp);
            }
            Statement::FUNC_DECL(fun) => {
                scope.local.insert(fun.func_name.0.clone(), PrimitiveType::FUNCTION(fun.clone()));
            }
        }
    }
    return VOID;
}

fn add_builtin(scope: &Scope, func_call: &FuncCall) -> PrimitiveType {
    let x = &func_call.arg_list[0];
    let y = &func_call.arg_list[1];
    let o1 = get_value(scope, x);
    let o2 = get_value(scope, y);
    if let PrimitiveType::F64(f1) = o1 {
        if let PrimitiveType::F64(f2) = o2 {
            return PrimitiveType::F64(f1 + f2);
        }
    }

    fatal_(&format!("Unknown error"))
}

fn sub_builtin(scope: &Scope, func_call: &FuncCall) -> PrimitiveType {
    let x = &func_call.arg_list[0];
    let y = &func_call.arg_list[1];
    let o1 = get_value(scope, x);
    let o2 = get_value(scope, y);
    if let PrimitiveType::F64(f1) = o1 {
        if let PrimitiveType::F64(f2) = o2 {
            return PrimitiveType::F64(f1 - f2);
        }
    }

    fatal_(&format!("Unknown error"))
}

fn mult_builtin(scope: &Scope, func_call: &FuncCall) -> PrimitiveType {
    let x = &func_call.arg_list[0];
    let y = &func_call.arg_list[1];
    let o1 = get_value(scope, x);
    let o2 = get_value(scope, y);
    if let PrimitiveType::F64(f1) = o1 {
        if let PrimitiveType::F64(f2) = o2 {
            return PrimitiveType::F64(f1 * f2);
        }
    }

    fatal_(&format!("Unknown error"))
}

fn div_builtin(scope: &Scope, func_call: &FuncCall) -> PrimitiveType {
    let x = &func_call.arg_list[0];
    let y = &func_call.arg_list[1];
    let o1 = get_value(scope, x);
    let o2 = get_value(scope, y);
    if let PrimitiveType::F64(f1) = o1 {
        if let PrimitiveType::F64(f2) = o2 {
            return PrimitiveType::F64(f1 / f2);
        }
    }
    fatal_(&format!("Unknown error"))
}

fn print(scope: &Scope, func_call: &FuncCall) -> PrimitiveType {
    for o in func_call.arg_list.iter() {
        let v = get_value(scope, o);
        print!("{} ", v);
    }
    println!();
    return VOID;
}

pub fn run_code(root: &Block) {
    let mut root_scope = Scope {
        parent: None,
        local: Default::default(),
    };
    root_scope.local.insert(Str::from("+"), PrimitiveType::BUILTIN(BuiltInFunc { execute: add_builtin }));
    root_scope.local.insert(Str::from("-"), PrimitiveType::BUILTIN(BuiltInFunc { execute: sub_builtin }));
    root_scope.local.insert(Str::from("*"), PrimitiveType::BUILTIN(BuiltInFunc { execute: mult_builtin }));
    root_scope.local.insert(Str::from("/"), PrimitiveType::BUILTIN(BuiltInFunc { execute: div_builtin }));
    root_scope.local.insert(Str::from("print"), PrimitiveType::BUILTIN(BuiltInFunc { execute: print }));
    run_block(Some(&root_scope), root);
}