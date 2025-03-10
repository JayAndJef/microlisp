use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::parser::Object;

/// Represents a variable scope with symbol bindings and optional parent scope
#[derive(Debug, PartialEq, Default)]
pub struct Scope {
    parent: Option<Rc<RefCell<Scope>>>,
    values: HashMap<String, Object>,
}

impl Scope {
    /// Creates a new scope that extends a parent scope
    pub fn extend(parent_scope: Rc<RefCell<Self>>) -> Self {
        Self {
            parent: Some(parent_scope),
            values: Default::default(),
        }
    }

    /// Retrieves a value from the scope or its parent scopes
    pub fn get(&self, key: &str) -> Option<Object> {
        match self.values.get(key) {
            Some(v) => Some(v.clone()),
            None => self.parent.as_ref().and_then(|v| v.borrow().get(key)),
        }
    }

    /// Sets a value in the current scope
    pub fn set(&mut self, key: &str, value: Object) {
        self.values.insert(key.to_string(), value);
    }
}

/// Evaluates a microlisp object in the given scope
pub fn eval_object(object: &Object, scope: &mut Rc<RefCell<Scope>>) -> Result<Object, String> {
    match object {
        Object::Void => Ok(Object::Void),
        Object::Lambda(_p, _b) => Ok(Object::Void),
        Object::Bool(_) => Ok(object.clone()),
        Object::Float(f) => Ok(Object::Float(*f)),
        Object::Symbol(s) => eval_symbol(s, scope),
        Object::List(l) => eval_list(l, scope),
    }
}

/// Evaluates a symbol by looking up its value in the scope
pub fn eval_symbol(symbol: &str, scope: &mut Rc<RefCell<Scope>>) -> Result<Object, String> {
    match scope.borrow_mut().get(symbol) {
        Some(o) => Ok(o.clone()),
        None => Err(format!("Unbound symbol {}", symbol)),
    }
}

/// Evaluates a list, which could be a special form or function call
pub fn eval_list(list: &[Object], scope: &mut Rc<RefCell<Scope>>) -> Result<Object, String> {
    dbg!("In eval list", &list[0]);
    match &list[0] {
        Object::Symbol(s) => match s.as_str() {
            "+" | "-" | "*" | "/" | "<" | ">" | "=" | "!=" => eval_op(s, &list[1..], scope),
            "define" => eval_definition(&list[1..], scope), // eval define, add to env
            "if" => eval_if(&list[1..], scope),             // eval if
            "lambda" => eval_lambda(&list[1..]),            // eval lambda declaration
            _ => eval_function_call(s, &list[1..], scope), // eval function call. builtins must be populated
        },
        _ => {
            let mut new_list = Vec::new();
            for obj in list {
                let result = eval_object(obj, scope)?;
                match result {
                    Object::Void => {}
                    _ => new_list.push(result),
                }
            }
            Ok(Object::List(new_list))
        }
    }
}

/// Evaluates a binary operation such as +, -, *, /, <, >, =, or !=
pub fn eval_op(
    function: &str,
    body: &[Object],
    scope: &mut Rc<RefCell<Scope>>,
) -> Result<Object, String> {
    if body.len() != 2 {
        return Err("Binary operators must have 2 arguments".to_string());
    }

    let left = eval_object(&body[0], scope)?;
    let right = eval_object(&body[1], scope)?;
    let left_val = match left {
        Object::Float(n) => n,
        _ => return Err(format!("Left operand must be an integer {:?}", left)),
    };
    let right_val = match right {
        Object::Float(n) => n,
        _ => return Err(format!("Right operand must be an integer {:?}", right)),
    };

    match function {
        "+" => Ok(Object::Float(left_val + right_val)),
        "-" => Ok(Object::Float(left_val - right_val)),
        "*" => Ok(Object::Float(left_val * right_val)),
        "/" => Ok(Object::Float(left_val / right_val)),
        "<" => Ok(Object::Bool(left_val < right_val)),
        ">" => Ok(Object::Bool(left_val > right_val)),
        "=" => Ok(Object::Bool(left_val == right_val)),
        "!=" => Ok(Object::Bool(left_val != right_val)),
        _ => unreachable!(),
    }
}

/// Evaluates a definition expression (define ...)
pub fn eval_definition(list: &[Object], scope: &mut Rc<RefCell<Scope>>) -> Result<Object, String> {
    dbg!("in eval definition");
    if list.len() % 2 != 0 {
        return Err("Expected even number of definitions".to_string());
    } else if list.is_empty() {
        return Err("Definition body needs arguments".to_string());
    }

    for chunk in list.chunks(2) {
        let k = chunk[0].clone();
        let v = eval_object(&chunk[1], scope)?;

        match k {
            Object::Symbol(s) => {
                scope.borrow_mut().set(&s, v);
            }
            _ => return Err("Expected symbol as key in definition".to_string()),
        }
    }

    Ok(eval_object(list.last().unwrap(), scope).unwrap())
}

/// Evaluates a conditional expression (if ...)
pub fn eval_if(list: &[Object], scope: &mut Rc<RefCell<Scope>>) -> Result<Object, String> {
    if list.len() != 3 {
        return Err("Expected 3 items in if expression".to_string());
    }

    let cond = match eval_object(&list[0], scope)? {
        Object::Bool(b) => b,
        _ => return Err("Condition in if statement must evaluate to a boolean".to_string()),
    };

    eval_object(&list[if cond { 1 } else { 2 }], scope)
}

/// Evaluates a lambda expression and returns a Lambda object
pub fn eval_lambda(list: &[Object]) -> Result<Object, String> {
    if list.len() != 2 {
        return Err("Lambda definition should have an parameter list and a body".to_string());
    }

    let params = match &list[0] {
        Object::List(l) => {
            let mut params = Vec::new();
            for obj in l {
                if let Object::Symbol(p) = obj {
                    params.push(p.clone());
                } else {
                    return Err("Expected symbol names for lambda parameters".to_string());
                }
            }
            params
        }
        _ => return Err("Expected parameter list".to_string()),
    };

    let body = match &list[1] {
        Object::List(list) => list.clone(),
        _ => return Err("Expected list for Lamba Body".to_string()),
    };

    Ok(Object::Lambda(params, body))
}

/// Evaluates a function call by looking up the function and applying it to arguments
pub fn eval_function_call(
    name: &str,
    arguments: &[Object],
    scope: &mut Rc<RefCell<Scope>>,
) -> Result<Object, String> {
    let function = match scope.borrow().get(name) {
        Some(f) => f,
        None => return Err(format!("Unbound function symbol: {}", name)),
    };

    if let Object::Lambda(p, b) = function {
        let mut function_scope = Rc::new(RefCell::new(Scope::extend(scope.clone())));

        for (i, param) in p.iter().enumerate() {
            let value = eval_object(&arguments[i], scope)?;
            function_scope.borrow_mut().set(param, value);
        }

        eval_object(&Object::List(b), &mut function_scope)
    } else {
        Err("Function call symbol is not bound to a lambda".to_string())
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_bin_op() {
        assert_eq!(1, 1)
    }
}
