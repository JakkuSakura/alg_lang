use super::parser::*;
use crate::util::Str;
use std::collections::HashMap;
struct Scope {
    local: HashMap<Str, f64>,
}

fn get_value(v: &Value) -> f64 {
    unimplemented!();
}

pub fn run_code(root: &Block) {
    let mut scope = Scope {
        local: HashMap::new(),
    };

    for statement in root.0.iter() {
        match statement {
            Statement::ASSIGNMENT(ass) => {
                scope.local.insert(ass.id.0.clone(), get_value(&ass.val));
            }
        }
    }
}
