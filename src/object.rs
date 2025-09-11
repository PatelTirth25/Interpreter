use core::fmt;
use std::{cell::RefCell, rc::Rc};

use crate::interpreter::{loxcallable::LoxCallable, loxclass::LoxClass, loxinstance::LoxInstance};

#[derive(Clone, Debug)]
pub enum Object {
    Number(f64),
    Boolean(bool),
    String(String),
    Callable(Rc<dyn LoxCallable>),
    Class(Rc<LoxClass>),
    Instance(Rc<RefCell<LoxInstance>>),
    Nill,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::Nill => write!(f, "nil"),
            Object::Number(n) => write!(f, "{}", n),
            Object::Boolean(b) => write!(f, "{}", b),
            Object::String(s) => write!(f, "{}", s),
            Object::Callable(c) => write!(f, "{}", c),
            Object::Instance(i) => write!(f, "{}", i.borrow()),
            Object::Class(c) => write!(f, "{}", c),
        }
    }
}
