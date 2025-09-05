use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{error::NZErrors, object::Object, token::Token};

#[derive(Debug)]
pub struct Environment {
    enclosing: Option<Rc<RefCell<Environment>>>,
    hashmap: HashMap<String, Object>,
}

impl Environment {
    pub fn new(enclosing: Option<Rc<RefCell<Environment>>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            enclosing,
            hashmap: HashMap::new(),
        }))
    }
    pub fn define(&mut self, name: String, value: Object) {
        self.hashmap.insert(name, value);
    }
    pub fn get(&self, token: &Token) -> Result<Object, NZErrors> {
        match self.hashmap.get(&token.lexeme) {
            Some(value) => Ok(value.clone()),
            None => {
                if let Some(enclosing) = &self.enclosing {
                    return enclosing.borrow().get(token);
                }
                Err(NZErrors::RuntimeError(
                    token.clone(),
                    format!("Undefined variable '{}'.", token.lexeme),
                ))
            }
        }
    }
    pub fn assign(&mut self, token: &Token, value: Object) -> Result<(), NZErrors> {
        match self.hashmap.get_mut(&token.lexeme) {
            Some(v) => {
                *v = value;
                Ok(())
            }
            None => {
                if let Some(enclosing) = &mut self.enclosing {
                    return enclosing.borrow_mut().assign(token, value);
                }
                Err(NZErrors::RuntimeError(
                    token.clone(),
                    format!("Undefined variable '{}'.", token.lexeme),
                ))
            }
        }
    }
}
