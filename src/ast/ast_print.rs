use crate::token::Token;

use super::{Expr, Literal, Visitor};

pub struct AstPrint;

impl AstPrint {
    pub fn new() -> Self {
        AstPrint
    }
    pub fn print(&mut self, expr: &Expr) -> String {
        expr.accept(self)
    }
    fn parenthesize(&mut self, name: &str, exprs: &[&Expr]) -> String {
        let mut builder = format!("({}", name);
        for expr in exprs {
            builder.push(' ');
            builder.push_str(&expr.accept(self));
        }
        builder.push(')');
        builder
    }
}

impl Visitor<String> for AstPrint {
    fn visit_binary_expr(&mut self, left: &Expr, op: &Token, right: &Expr) -> String {
        self.parenthesize(&op.lexeme, &[left, right])
    }

    fn visit_grouping_expr(&mut self, expression: &Expr) -> String {
        self.parenthesize("group", &[expression])
    }

    fn visit_literal_expr(&mut self, literal: &Literal) -> String {
        match literal {
            Literal::Number(n) => n.to_string(),
            Literal::String(s) => format!("\"{}\"", s),
            Literal::Boolean(b) => b.to_string(),
            Literal::Nil => "nil".to_string(),
        }
    }

    fn visit_unary_expr(&mut self, op: &Token, expr: &Expr) -> String {
        self.parenthesize(&op.lexeme, &[expr])
    }
}
