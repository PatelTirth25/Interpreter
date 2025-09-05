use core::fmt;
use std::{cell::RefCell, rc::Rc};

use crate::{ast::Stmt, environment::Environment, error::NZErrors, object::Object, token::Token};

use super::{loxcallable::LoxCallable, Interpreter};

#[derive(Clone, Debug)]
pub struct LoxFunction {
    name: Token,
    params: Vec<Token>,
    body: Vec<Stmt>,
    closure: Rc<RefCell<Environment>>,
}

impl LoxFunction {
    pub fn new(
        name: Token,
        params: Vec<Token>,
        body: Vec<Stmt>,
        closure: Rc<RefCell<Environment>>,
    ) -> Self {
        Self {
            name,
            params,
            body,
            closure,
        }
    }
}

impl fmt::Display for LoxFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let function = Stmt::Function {
            name: self.name.clone(),
            params: self.params.clone(),
            body: self.body.clone(),
        };

        write!(f, "{}", function)
    }
}

impl LoxCallable for LoxFunction {
    fn arity(&self) -> usize {
        self.params.len()
    }

    fn call(&self, interpreter: &mut Interpreter, args: &Vec<Object>) -> Result<Object, NZErrors> {
        let environment = Environment::new(Some(Rc::clone(&self.closure)));
        for (param, arg) in self.params.iter().zip(args.iter()) {
            environment
                .borrow_mut()
                .define(param.lexeme.clone(), arg.clone());
        }

        match interpreter.execute_block(&self.body, environment) {
            Ok(_) => Ok(Object::Nill),
            Err(NZErrors::Return(value)) => Ok(value),
            Err(err) => Err(err),
        }
    }
}
