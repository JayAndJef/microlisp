use std::{borrow, cell::RefCell, collections::HashMap, rc::Rc};

use crate::parser::Object;

#[derive(Debug, PartialEq, Default)]
pub struct Scope {
    parent: Option<Rc<RefCell<Scope>>>,
    values: HashMap<String, Object>,
}

impl Scope {
    pub fn extend(parent_scope: Rc<RefCell<Self>>) -> Self {
        Self {
            parent: Some(parent_scope),
            values: Default::default(),
        }
    }

    pub fn get(&self, key: &str) -> Option<Object> {
        match self.values.get(key) {
            Some(v) => Some(v.clone()),
            None => self.parent.as_ref().and_then(|v| v.borrow().get(key.clone()))
        }
    }

    pub fn set(&mut self, key: &str, value: Object) {
        self.values.insert(key.to_string(), value);
    }
}


pub fn eval_object(object: &Object, scope: &mut Rc<RefCell<Scope>>) -> Result<Object, String>{
    match object {
        Object::Void => Ok(Object::Void),
        Object::Lambda(_p, _b, ) => Ok(Object::Void),
        Object::Bool(_) => Ok(object.clone()),
        Object::Integer(n) => Ok(Object::Integer(*n)),
        Object::Float(f) => Ok(Object::Float(*f)),
        Object::Symbol(s) => eval_symbol(s, scope),
        Object::List(_) => todo!(),
    }
}


pub fn eval_symbol(symbol: &str, scope: &mut Rc<RefCell<Scope>>) -> Result<Object, String> {
    match scope.borrow_mut().get(symbol) {
        Some(o) => Ok(o.clone()),
        None => Err(format!("Unbound symbol {}", symbol)),
    }
}

pub fn eval_list(list: Vec<Object>, scope: &mut Rc<RefCell<Scope>>) -> Result<Object, String> {
    match &list[0] {
        Object::Symbol(s) => match s.as_str() {
            "define" => todo!(), // eval define, add to env
            "if" => todo!(), // eval if
            "lambda" => todo!(), // eval lambda declaration
            _ => todo!(), // eval function call. builtins must be populated
        },
        _ => todo!() // not sure about this right now
    }
}
