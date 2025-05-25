use crate::token::{Literal, Token};

pub mod ast_generator;
pub mod ast_print;

#[derive(Debug, Clone)]
pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Literal),
    Unary(Token, Box<Expr>),
}

pub trait Visitor<T> {
    fn visit_binary_expr(&mut self, left: &Expr, op: &Token, right: &Expr) -> T;
    fn visit_grouping_expr(&mut self, expression: &Expr) -> T;
    fn visit_literal_expr(&mut self, literal: &Literal) -> T;
    fn visit_unary_expr(&mut self, op: &Token, expr: &Expr) -> T;
}

impl Expr {
    pub fn accept<T>(&self, visitor: &mut impl Visitor<T>) -> T {
        match self {
            Expr::Binary(left, op, right) => visitor.visit_binary_expr(left, op, right),
            Expr::Grouping(expr) => visitor.visit_grouping_expr(expr),
            Expr::Literal(lit) => visitor.visit_literal_expr(lit),
            Expr::Unary(op, expr) => visitor.visit_unary_expr(op, expr),
        }
    }
}
