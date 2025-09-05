// use crate::token::Token;
//
// use super::{Expr, ExprVisitor, Literal};
//
// pub struct AstPrint;
//
// impl AstPrint {
//     pub fn new() -> Self {
//         AstPrint
//     }
//     pub fn print(&mut self, expr: &Expr) -> String {
//         expr.accept(self)
//     }
//     fn parenthesize(&mut self, name: &str, exprs: &[&Expr]) -> String {
//         let mut builder = format!("({}", name);
//         for expr in exprs {
//             builder.push(' ');
//             builder.push_str(&expr.accept(self));
//         }
//         builder.push(')');
//         builder
//     }
// }
//
// impl ExprVisitor<String> for AstPrint {
//     fn visit_binary_expr(&mut self, left: &Expr, op: &Token, right: &Expr) -> String {
//         self.parenthesize(&op.lexeme, &[left, right])
//     }
//
//     fn visit_grouping_expr(&mut self, expression: &Expr) -> String {
//         self.parenthesize("group", &[expression])
//     }
//
//     fn visit_literal_expr(&mut self, literal: &Literal) -> String {
//         match literal {
//             Literal::Number(n) => n.to_string(),
//             Literal::String(s) => format!("\"{}\"", s),
//             Literal::Boolean(b) => b.to_string(),
//             Literal::Nil => "nil".to_string(),
//         }
//     }
//
//     fn visit_unary_expr(&mut self, op: &Token, expr: &Expr) -> String {
//         self.parenthesize(&op.lexeme, &[expr])
//     }
//
//     fn visit_assign_expr(&mut self, name: &Token, value: &Expr) -> String {
//         self.parenthesize(&name.lexeme, &[value])
//     }
//
//     fn visit_variable_expr(&mut self, name: &Token) -> String {
//         name.lexeme.to_string()
//     }
//     fn visit_logical_expr(&mut self, left: &Expr, op: &Token, right: &Expr) -> String {
//         self.parenthesize(&op.lexeme, &[left, right])
//     }
//     fn visit_call_expr(&mut self, callee: &Expr, _paren: &Token, arguments: &[Expr]) -> String {
//         let mut builder = self.parenthesize("call", &[callee]);
//         for arg in arguments {
//             builder.push(' ');
//             builder.push_str(&arg.accept(self));
//         }
//         builder
//     }
// }
