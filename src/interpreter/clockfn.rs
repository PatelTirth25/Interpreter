use crate::{error::NZErrors, object::Object, token::Token};

use super::{loxcallable::LoxCallable, Interpreter};

#[derive(Debug, Clone)]
pub struct ClockFn;

impl LoxCallable for ClockFn {
    fn arity(&self) -> usize {
        0
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _args: &Vec<Object>,
    ) -> Result<Object, NZErrors> {
        use std::time::{SystemTime, UNIX_EPOCH};
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| NZErrors::RuntimeError(Token::default(), format!("{}", e)))?
            .as_millis();
        Ok(Object::Number((now as f64) / 1000.0))
    }
}

impl std::fmt::Display for ClockFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn>")
    }
}
