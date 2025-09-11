use core::fmt;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{error::NZErrors, object::Object, token::Token};

use super::loxclass::LoxClass;

#[derive(Debug, Clone)]
pub struct LoxInstance {
    klass: Rc<LoxClass>,
    fields: HashMap<String, Object>,
}

impl LoxInstance {
    pub fn new(klass: &Rc<LoxClass>) -> Self {
        Self {
            klass: Rc::clone(klass),
            fields: HashMap::new(),
        }
    }

    pub fn get(&self, name: &Token) -> Result<Object, NZErrors> {
        if let Some(value) = self.fields.get(&name.lexeme) {
            Ok(value.clone())
        } else {
            if let Some(method) = self.klass.find_method(&name.lexeme) {
                let bound = method.borrow().bind(Rc::new(RefCell::new(self.clone())));
                Ok(Object::Callable(Rc::new(bound)))
            } else {
                Err(NZErrors::RuntimeError(
                    name.clone(),
                    format!("Undefined property '{}'.", name.lexeme),
                ))
            }
        }
    }

    pub fn set(&mut self, name: &Token, value: Object) {
        self.fields.insert(name.lexeme.clone(), value);
    }
}

impl fmt::Display for LoxInstance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} Instance", self.klass.name.lexeme)
    }
}
