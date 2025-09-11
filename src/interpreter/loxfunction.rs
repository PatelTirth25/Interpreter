use core::fmt;
use std::{cell::RefCell, rc::Rc};

use crate::{ast::Stmt, environment::Environment, error::NZErrors, object::Object, token::Token};

use super::{loxcallable::LoxCallable, loxinstance::LoxInstance, Interpreter};

#[derive(Clone, Debug)]
pub struct LoxFunction {
    name: Token,
    params: Vec<Token>,
    body: Vec<Stmt>,
    closure: Rc<RefCell<Environment>>,
    initializer: bool,
}

impl LoxFunction {
    pub fn new(
        name: Token,
        params: Vec<Token>,
        body: Vec<Stmt>,
        closure: Rc<RefCell<Environment>>,
        initializer: bool,
    ) -> Self {
        Self {
            name,
            params,
            body,
            closure,
            initializer,
        }
    }

    pub fn bind(&self, instance: Rc<RefCell<LoxInstance>>) -> LoxFunction {
        let environment = Environment::new(Some(Rc::clone(&self.closure)));
        environment.borrow_mut().define(
            "this".to_string(),
            Object::Instance(Rc::new(RefCell::new(instance.borrow().clone()))),
        );
        LoxFunction {
            name: self.name.clone(),
            params: self.params.clone(),
            body: self.body.clone(),
            closure: environment,
            initializer: self.initializer,
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
            Ok(_) => {
                if self.initializer {
                    match self.closure.borrow().get_at(0, "this") {
                        Some(x) => Ok(x),
                        None => Ok(Object::Nill),
                    }
                } else {
                    Ok(Object::Nill)
                }
            }
            Err(NZErrors::Return(value)) => Ok(value),
            Err(err) => Err(err),
        }
    }
}
