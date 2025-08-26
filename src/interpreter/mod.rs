use crate::{
    ast::{Expr, Visitor},
    error::NZErrors,
    object::Object,
    token::{token_types::TokenType, Literal, Token},
};

pub struct Interpreter;

impl Interpreter {
    pub fn interpret(&mut self, expr: &Expr) -> Result<String, NZErrors> {
        match self.evaluate(expr) {
            Ok(obj) => Ok(obj.to_string()),
            Err(err) => Err(err),
        }
    }
    fn evaluate(&mut self, expr: &Expr) -> Result<Object, NZErrors> {
        expr.accept(self)
    }
    fn istrusthy(&self, obj: &Object) -> bool {
        match obj {
            Object::Boolean(b) => *b,
            Object::Nill => false,
            _ => true,
        }
    }

    fn issub(&self, right: &Object, left: &Object, op: &Token) -> Result<Object, NZErrors> {
        match (left, right) {
            (Object::Number(l), Object::Number(r)) => Ok(Object::Number(l - r)),
            _ => Err(NZErrors::RuntimeError(
                op.clone(),
                "Cannot subtract two different types".to_string(),
            )),
        }
    }

    fn ismul(&self, right: &Object, left: &Object, op: &Token) -> Result<Object, NZErrors> {
        match (left, right) {
            (Object::Number(l), Object::Number(r)) => Ok(Object::Number(l * r)),
            _ => Err(NZErrors::RuntimeError(
                op.clone(),
                "Cannot multiply two different types".to_string(),
            )),
        }
    }

    fn isdiv(&self, right: &Object, left: &Object, op: &Token) -> Result<Object, NZErrors> {
        match (left, right) {
            (Object::Number(l), Object::Number(r)) => Ok(Object::Number(l / r)),
            _ => Err(NZErrors::RuntimeError(
                op.clone(),
                "Cannot divide two different types".to_string(),
            )),
        }
    }

    fn isadd(&self, right: &Object, left: &Object, op: &Token) -> Result<Object, NZErrors> {
        match (left, right) {
            (Object::Number(l), Object::Number(r)) => Ok(Object::Number(l + r)),
            (Object::String(l), Object::String(r)) => Ok(Object::String(format!("{}{}", l, r))),
            (Object::Number(l), Object::String(r)) => Ok(Object::String(format!("{}{}", l, r))),
            (Object::String(l), Object::Number(r)) => Ok(Object::String(format!("{}{}", l, r))),
            _ => Err(NZErrors::RuntimeError(
                op.clone(),
                "Cannot add two different types".to_string(),
            )),
        }
    }

    fn isgreaterequal(
        &self,
        right: &Object,
        left: &Object,
        op: &Token,
    ) -> Result<Object, NZErrors> {
        match (left, right) {
            (Object::Number(l), Object::Number(r)) => Ok(Object::Boolean(l >= r)),
            _ => Err(NZErrors::RuntimeError(
                op.clone(),
                "Cannot compare two different types".to_string(),
            )),
        }
    }

    fn islessequal(&self, right: &Object, left: &Object, op: &Token) -> Result<Object, NZErrors> {
        match (left, right) {
            (Object::Number(l), Object::Number(r)) => Ok(Object::Boolean(l <= r)),
            _ => Err(NZErrors::RuntimeError(
                op.clone(),
                "Cannot compare two different types".to_string(),
            )),
        }
    }

    fn isless(&self, right: &Object, left: &Object, op: &Token) -> Result<Object, NZErrors> {
        match (left, right) {
            (Object::Number(l), Object::Number(r)) => Ok(Object::Boolean(l < r)),
            _ => Err(NZErrors::RuntimeError(
                op.clone(),
                "Cannot compare two different types".to_string(),
            )),
        }
    }

    fn isgreater(&self, right: &Object, left: &Object, op: &Token) -> Result<Object, NZErrors> {
        match (left, right) {
            (Object::Number(l), Object::Number(r)) => Ok(Object::Boolean(l > r)),
            _ => Err(NZErrors::RuntimeError(
                op.clone(),
                "Cannot compare two different types".to_string(),
            )),
        }
    }

    fn isequal(&self, right: &Object, left: &Object, op: &Token) -> Result<Object, NZErrors> {
        match (left, right) {
            (Object::Number(l), Object::Number(r)) => Ok(Object::Boolean(l == r)),
            (Object::String(l), Object::String(r)) => Ok(Object::Boolean(l == r)),
            (Object::Nill, Object::Nill) => Ok(Object::Boolean(false)),
            _ => Err(NZErrors::RuntimeError(
                op.clone(),
                "Cannot compare two different types".to_string(),
            )),
        }
    }

    fn isnotequal(&self, right: &Object, left: &Object, op: &Token) -> Result<Object, NZErrors> {
        match (left, right) {
            (Object::Number(l), Object::Number(r)) => Ok(Object::Boolean(l != r)),
            (Object::String(l), Object::String(r)) => Ok(Object::Boolean(l != r)),
            (Object::Nill, Object::Nill) => Ok(Object::Boolean(false)),
            _ => Err(NZErrors::RuntimeError(
                op.clone(),
                "Cannot compare two different types".to_string(),
            )),
        }
    }
}

impl Visitor<Result<Object, NZErrors>> for Interpreter {
    fn visit_binary_expr(
        &mut self,
        left: &Expr,
        op: &Token,
        right: &Expr,
    ) -> Result<Object, NZErrors> {
        let right = self.evaluate(right)?;
        let left = self.evaluate(left)?;

        match op.token_type {
            TokenType::PLUS => self.isadd(&right, &left, op),
            TokenType::MINUS => self.issub(&right, &left, op),
            TokenType::STAR => self.ismul(&right, &left, op),
            TokenType::SLASH => self.isdiv(&right, &left, op),
            TokenType::EQUALEQUAL => self.isequal(&right, &left, op),
            TokenType::BANGEQUAL => self.isnotequal(&right, &left, op),
            TokenType::GREATER => self.isgreater(&right, &left, op),
            TokenType::GREATEREQUAL => self.isgreaterequal(&right, &left, op),
            TokenType::LESS => self.isless(&right, &left, op),
            TokenType::LESSEQUAL => self.islessequal(&right, &left, op),
            _ => Err(NZErrors::RuntimeError(
                op.clone(),
                "Unsupported binary operator".to_string(),
            )),
        }
    }

    fn visit_grouping_expr(&mut self, expr: &Expr) -> Result<Object, NZErrors> {
        return self.evaluate(expr);
    }

    fn visit_literal_expr(&mut self, literal: &Literal) -> Result<Object, NZErrors> {
        match literal {
            Literal::Number(n) => Ok(Object::Number(*n)),
            Literal::String(s) => Ok(Object::String(s.to_string())),
            Literal::Boolean(b) => Ok(Object::Boolean(*b)),
            Literal::Nil => Ok(Object::Nill),
        }
    }

    fn visit_unary_expr(&mut self, op: &Token, expr: &Expr) -> Result<Object, NZErrors> {
        let right = self.evaluate(expr)?;
        match op.token_type {
            TokenType::MINUS => match right {
                Object::Number(n) => Ok(Object::Number(-n)),
                _ => {
                    return Err(NZErrors::RuntimeError(
                        op.clone(),
                        "Operand must be a number for unary minus.".to_string(),
                    ))
                }
            },
            TokenType::BANG => Ok(Object::Boolean(self.istrusthy(&right))),
            _ => {
                return Err(NZErrors::RuntimeError(
                    op.clone(),
                    "Unknown operator.".to_string(),
                ))
            }
        }
    }
}
