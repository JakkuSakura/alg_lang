use super::parser::*;

use std::collections::HashMap;
use crate::runtime::PrimitiveType::VOID;
use crate::util::fatal_;
use std::fmt::{Display, Formatter, Error};
use std::cell::RefCell;
use std::rc::Rc;


#[derive(Clone)]
pub struct BuiltInFunc {
    execute: fn(Rc<RefCell<Scope>>, &FuncCall) -> PrimitiveType
}

impl PartialEq for BuiltInFunc {
    fn eq(&self, other: &Self) -> bool {
        self.execute as usize == other.execute as usize
    }
}

#[derive(PartialEq, Clone)]
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

pub struct Scope {
    pub parent: Option<Rc<RefCell<Scope>>>,
    pub local: HashMap<String, PrimitiveType>,
}

impl Scope {
    fn lookup(&self, name: &str) -> PrimitiveType {
        debug!("Looking for {}", name);
        self.try_lookup(name).expect(&format!("Error because no value called {} at this scope", name))
    }
    fn try_lookup(&self, name: &str) -> Option<PrimitiveType> {
        match self.local.get(name) {
            None => {
                match &self.parent {
                    None => None,
                    Some(x) => {
                        x.as_ref().borrow().try_lookup(name)
                    }
                }
            }
            Some(x) => { Some(x.clone()) }
        }
    }
}

fn get_value(scope: Rc<RefCell<Scope>>, v: &Value) -> PrimitiveType {
    match v {
        Value::VAR(id) => { scope.as_ref().borrow().lookup(&id.0.to_string()) }
        Value::FLOAT(f) => { PrimitiveType::F64(*f) }
        Value::FUNC_CALL(fc) => {
            let func;
            {
                func = scope.as_ref().borrow().lookup(&fc.func_name.0.to_string());
            }
            match func {
                PrimitiveType::FUNCTION(f) => {
                    return run_block(true, Some(scope), &f.body);
                }
                PrimitiveType::BUILTIN(builtin) => {
                    return (builtin.execute)(scope, fc);
                }
                _ => {
                    fatal_(&format!("Error: {} is not a function or built-in function", fc.func_name.0))
                }
            }
        }
        Value::INT(i) => PrimitiveType::I32(*i),
        Value::BOOL(b) => PrimitiveType::I32(*b as i32),
    }
}

pub fn to_boolean(v: &PrimitiveType) -> bool {
    *v != PrimitiveType::I32(0) && *v != PrimitiveType::F64(0.0) && *v != PrimitiveType::VOID
}

pub fn run_block(new_scope: bool, parent_scope: Option<Rc<RefCell<Scope>>>, blk: &Block) -> PrimitiveType {
    let scope = if new_scope {
        Rc::new(RefCell::new(Scope {
            parent: parent_scope,
            local: HashMap::new(),
        }))
    } else {
        parent_scope.unwrap()
    };

    for statement in blk.0.iter() {
        match statement {
            Statement::ASSIGNMENT(ass) => {
                let val = get_value(scope.clone(), &ass.val);
                scope.as_ref().borrow_mut().local.insert(ass.id.0.clone(), val);
            }
            Statement::RETURN(Return(value)) => {
                return get_value(scope.clone(), value);
            }
            Statement::EXPRESSION(exp) => {
                get_value(scope.clone(), exp);
            }
            Statement::FUNC_DECL(fun) => {
                scope.as_ref().borrow_mut().local.insert(fun.func_name.0.clone(), PrimitiveType::FUNCTION(fun.clone()));
            }
            Statement::IF(x) => {
                for i in 0..x.cond.len() {
                    let v = get_value(scope.clone(), &x.cond[i]);
                    if to_boolean(&v) {
                        run_block(false, Some(scope.clone()), &x.then[i]);
                        break;
                    }
                }
            }
            Statement::WHILE(x) => {
                while to_boolean(&get_value(scope.clone(), &x.cond)) {
                    run_block(false, Some(scope.clone()), &x.then);
                }
            }
            Statement::NOTHING => { /*nothing*/ }
        }
    }
    return VOID;
}

fn add_builtin(scope: Rc<RefCell<Scope>>, func_call: &FuncCall) -> PrimitiveType {
    let x = &func_call.arg_list[0];
    let y = &func_call.arg_list[1];
    let o1 = get_value(scope.clone(), x);
    let o2 = get_value(scope.clone(), y);
    if let PrimitiveType::F64(f1) = o1 {
        if let PrimitiveType::F64(f2) = o2 {
            return PrimitiveType::F64(f1 + f2);
        }
    }
    if let PrimitiveType::I32(f1) = o1 {
        if let PrimitiveType::I32(f2) = o2 {
            return PrimitiveType::I32(f1 + f2);
        }
    }

    fatal_(&format!("Unknown error"))
}

fn sub_builtin(scope: Rc<RefCell<Scope>>, func_call: &FuncCall) -> PrimitiveType {
    let x = &func_call.arg_list[0];
    let y = &func_call.arg_list[1];
    let o1 = get_value(scope.clone(), x);
    let o2 = get_value(scope, y);
    if let PrimitiveType::F64(f1) = o1 {
        if let PrimitiveType::F64(f2) = o2 {
            return PrimitiveType::F64(f1 - f2);
        }
    }
    if let PrimitiveType::I32(f1) = o1 {
        if let PrimitiveType::I32(f2) = o2 {
            return PrimitiveType::I32(f1 - f2);
        }
    }
    fatal_(&format!("Unknown error"))
}

fn multi_builtin(scope: Rc<RefCell<Scope>>, func_call: &FuncCall) -> PrimitiveType {
    let x = &func_call.arg_list[0];
    let y = &func_call.arg_list[1];
    let o1 = get_value(scope.clone(), x);
    let o2 = get_value(scope, y);
    if let PrimitiveType::F64(f1) = o1 {
        if let PrimitiveType::F64(f2) = o2 {
            return PrimitiveType::F64(f1 * f2);
        }
    }
    if let PrimitiveType::I32(f1) = o1 {
        if let PrimitiveType::I32(f2) = o2 {
            return PrimitiveType::I32(f1 * f2);
        }
    }
    fatal_(&format!("Unknown error"))
}

fn div_builtin(scope: Rc<RefCell<Scope>>, func_call: &FuncCall) -> PrimitiveType {
    let x = &func_call.arg_list[0];
    let y = &func_call.arg_list[1];
    let o1 = get_value(scope.clone(), x);
    let o2 = get_value(scope, y);
    if let PrimitiveType::F64(f1) = o1 {
        if let PrimitiveType::F64(f2) = o2 {
            return PrimitiveType::F64(f1 / f2);
        }
    }
    if let PrimitiveType::I32(f1) = o1 {
        if let PrimitiveType::I32(f2) = o2 {
            return PrimitiveType::I32(f1 / f2);
        }
    }

    fatal_(&format!("Unknown error"))
}

fn print(scope: Rc<RefCell<Scope>>, func_call: &FuncCall) -> PrimitiveType {
    for o in func_call.arg_list.iter() {
        let v = get_value(scope.clone(), o);
        print!("{} ", v);
    }
    println!();
    return VOID;
}

pub fn run_code(root: &Block) {
    let root_scope = Rc::new(RefCell::new(Scope {
        parent: None,
        local: Default::default(),
    }));
    {
        let mut ref_mut = root_scope.as_ref().borrow_mut();

        ref_mut.local.insert("-".to_string(), PrimitiveType::BUILTIN(BuiltInFunc { execute: sub_builtin }));
        ref_mut.local.insert("+".to_string(), PrimitiveType::BUILTIN(BuiltInFunc { execute: add_builtin }));
        ref_mut.local.insert("*".to_string(), PrimitiveType::BUILTIN(BuiltInFunc { execute: multi_builtin }));
        ref_mut.local.insert("/".to_string(), PrimitiveType::BUILTIN(BuiltInFunc { execute: div_builtin }));
        ref_mut.local.insert("print".to_string(), PrimitiveType::BUILTIN(BuiltInFunc { execute: print }));
    }

    run_block(false, Some(root_scope), root);
}