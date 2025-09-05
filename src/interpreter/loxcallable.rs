use super::Interpreter;
use crate::{error::NZErrors, object::Object};
use core::fmt;

pub trait LoxCallable: fmt::Display + fmt::Debug {
    fn arity(&self) -> usize;
    fn call(&self, interpreter: &mut Interpreter, args: &Vec<Object>) -> Result<Object, NZErrors>;
}
