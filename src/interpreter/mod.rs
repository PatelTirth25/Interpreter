mod clockfn;
pub mod loxcallable;
mod loxfunction;
use std::{cell::RefCell, rc::Rc};

use clockfn::ClockFn;
use loxfunction::LoxFunction;

use crate::{
    ast::{Expr, ExprVisitor, Stmt, StmtVisitor},
    environment::Environment,
    error::NZErrors,
    object::Object,
    token::{token_types::TokenType, Literal, Token},
};

pub struct Interpreter {
    globals: Rc<RefCell<Environment>>,
    environment: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub fn new() -> Self {
        let globals = Environment::new(None);

        globals
            .borrow_mut()
            .define("clock".to_string(), Object::Callable(Rc::new(ClockFn)));

        Self {
            environment: Rc::clone(&globals),
            globals: Rc::clone(&globals),
        }
    }
    pub fn interpret(&mut self, stmplist: &[Stmt]) -> Result<(), NZErrors> {
        for stmt in stmplist {
            self.execute(stmt)?;
        }
        Ok(())
    }
    fn execute(&mut self, stmt: &Stmt) -> Result<(), NZErrors> {
        stmt.accept(self)
    }

    fn execute_block(
        &mut self,
        stmtlist: &[Stmt],
        environment: Rc<RefCell<Environment>>,
    ) -> Result<(), NZErrors> {
        let previous = Rc::clone(&self.environment);
        self.environment = environment;

        let result = (|| {
            for stmt in stmtlist {
                self.execute(stmt)?;
            }
            Ok(())
        })();

        self.environment = previous;
        result
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

impl ExprVisitor<Result<Object, NZErrors>> for Interpreter {
    fn visit_binary_expr(
        &mut self,
        left: &Expr,
        op: &Token,
        right: &Expr,
    ) -> Result<Object, NZErrors> {
        let left = self.evaluate(left)?;
        let right = self.evaluate(right)?;

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

    fn visit_variable_expr(&mut self, name: &Token) -> Result<Object, NZErrors> {
        self.environment.borrow().get(name)
    }

    fn visit_assign_expr(&mut self, name: &Token, value: &Expr) -> Result<Object, NZErrors> {
        let value = self.evaluate(value)?;
        self.environment.borrow_mut().assign(name, value.clone())?;
        Ok(value)
    }

    fn visit_logical_expr(
        &mut self,
        left: &Expr,
        op: &Token,
        right: &Expr,
    ) -> Result<Object, NZErrors> {
        let left = self.evaluate(left)?;
        if op.token_type == TokenType::OR {
            if self.istrusthy(&left) {
                return Ok(left);
            }
        } else {
            if !self.istrusthy(&left) {
                return Ok(left);
            }
        }
        self.evaluate(right)
    }

    fn visit_call_expr(
        &mut self,
        callee: &Expr,
        paren: &Token,
        arguments: &[Expr],
    ) -> Result<Object, NZErrors> {
        let call = self.evaluate(callee)?;
        let mut args = Vec::new();

        for arg in arguments {
            args.push(self.evaluate(arg)?);
        }

        if let Object::Callable(function) = call {
            if args.len() != function.arity() {
                return Err(NZErrors::RuntimeError(
                    paren.clone(),
                    format!(
                        "Expected {} arguments but got {}.",
                        function.arity(),
                        args.len()
                    ),
                ));
            }

            return Ok(function.call(self, &args)?);
        }

        Err(NZErrors::RuntimeError(
            paren.clone(),
            "Can only call functions and classes.".to_string(),
        ))
    }
}

impl StmtVisitor<Result<(), NZErrors>> for Interpreter {
    fn visit_expression_stmt(&mut self, expr: &Expr) -> Result<(), NZErrors> {
        self.evaluate(expr)?;
        Ok(())
    }

    fn visit_print_stmt(&mut self, expr: &Expr) -> Result<(), NZErrors> {
        let obj = self.evaluate(expr)?;
        println!("{}", obj.to_string());
        Ok(())
    }

    fn visit_var_stmt(&mut self, name: &Token, initializer: &Option<Expr>) -> Result<(), NZErrors> {
        let value = match initializer {
            Some(expr) => self.evaluate(expr)?,
            None => Object::Nill,
        };
        self.environment
            .borrow_mut()
            .define(name.lexeme.clone(), value);
        Ok(())
    }

    fn visit_block_stmt(&mut self, statements: &[Stmt]) -> Result<(), NZErrors> {
        self.execute_block(
            statements,
            Environment::new(Some(Rc::clone(&self.environment))),
        )
    }

    fn visit_if_stmt(
        &mut self,
        condition: &Expr,
        then_branch: &Stmt,
        else_branch: &Option<Box<Stmt>>,
    ) -> Result<(), NZErrors> {
        let eval = self.evaluate(condition)?;
        if self.istrusthy(&eval) {
            self.execute(then_branch)?;
        } else if let Some(else_branch) = else_branch {
            self.execute(else_branch)?;
        }
        Ok(())
    }

    fn visit_while_stmt(&mut self, condition: &Expr, body: &Stmt) -> Result<(), NZErrors> {
        let mut eval = self.evaluate(condition)?;
        while self.istrusthy(&eval) {
            self.execute(body)?;
            eval = self.evaluate(condition)?;
        }
        Ok(())
    }

    fn visit_function_stmt(
        &mut self,
        name: &Token,
        params: &[Token],
        body: &[Stmt],
    ) -> Result<(), NZErrors> {
        let function = LoxFunction::new(
            name.clone(),
            params.to_vec(),
            body.to_vec(),
            self.environment.clone(),
        );
        self.environment
            .borrow_mut()
            .define(name.lexeme.clone(), Object::Callable(Rc::new(function)));
        Ok(())
    }

    fn visit_return_stmt(
        &mut self,
        _keyword: &Token,
        value: &Option<Expr>,
    ) -> Result<(), NZErrors> {
        let value = match value {
            Some(expr) => self.evaluate(expr)?,
            None => Object::Nill,
        };
        Err(NZErrors::Return(value))
    }
}
