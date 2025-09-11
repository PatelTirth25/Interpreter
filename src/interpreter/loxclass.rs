use core::fmt;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{object::Object, token::Token};

use super::{
    loxcallable::LoxCallable, loxfunction::LoxFunction, loxinstance::LoxInstance, Interpreter,
};

#[derive(Clone, Debug)]
pub struct LoxClass {
    pub name: Token,
    pub superclass: Option<Rc<LoxClass>>,
    pub methods: HashMap<String, Rc<RefCell<LoxFunction>>>,
}

impl LoxClass {
    pub fn new(
        name: Token,
        superclass: Option<Rc<LoxClass>>,
        methods: HashMap<String, Rc<RefCell<LoxFunction>>>,
    ) -> Self {
        Self {
            name,
            superclass,
            methods,
        }
    }

    pub fn find_method(&self, name: &str) -> Option<Rc<RefCell<LoxFunction>>> {
        if let Some(method) = self.methods.get(name) {
            Some(Rc::clone(method))
        } else if let Some(superclass) = &self.superclass {
            superclass.find_method(name)
        } else {
            None
        }
    }
}

impl LoxCallable for LoxClass {
    fn arity(&self) -> usize {
        if let Some(initializer) = self.find_method("init") {
            initializer.borrow().arity()
        } else {
            0
        }
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        args: &Vec<Object>,
    ) -> Result<Object, super::NZErrors> {
        let loxinstance = LoxInstance::new(&Rc::new(LoxClass::new(
            self.name.clone(),
            self.superclass.clone(),
            self.methods.clone(),
        )));
        let initializer = self.find_method("init");

        if let Some(initializer) = initializer {
            initializer
                .borrow()
                .bind(Rc::new(RefCell::new(loxinstance.clone())))
                .call(interpreter, args)?;
        }
        Ok(Object::Instance(Rc::new(RefCell::new(loxinstance))))
    }
}

impl fmt::Display for LoxClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Class {}", self.name.lexeme)
    }
}
