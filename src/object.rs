use core::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    // Class(Rc<RefCell<LoxClass>>),
    // Callable(Function),
    // Instance(Rc<RefCell<LoxInstance>>),
    Number(f64),
    Boolean(bool),
    String(String),
    Nill,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::Nill => write!(f, "nil"),
            Object::Number(n) => write!(f, "{}", n),
            Object::Boolean(b) => write!(f, "{}", b),
            Object::String(s) => write!(f, "{}", s),
        }
    }
}
