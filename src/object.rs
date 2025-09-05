use core::fmt;
use std::rc::Rc;

use crate::interpreter::loxcallable::LoxCallable;

#[derive(Clone, Debug)]
pub enum Object {
    // Class(Rc<RefCell<LoxClass>>),
    // Instance(Rc<RefCell<LoxInstance>>),
    Number(f64),
    Boolean(bool),
    String(String),
    Callable(Rc<dyn LoxCallable>),
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
        }
    }
}
