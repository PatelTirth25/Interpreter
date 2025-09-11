use core::fmt;

use crate::token::{Literal, Token};

pub mod ast_generator;
pub mod ast_print;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Literal),
    Unary(Token, Box<Expr>),
    Variable(Token),
    Assign(Token, Box<Expr>),
    Logical(Box<Expr>, Token, Box<Expr>),
    Call(Box<Expr>, Token, Vec<Expr>),
    Get(Box<Expr>, Token),
    Set(Box<Expr>, Token, Box<Expr>),
    Super(Token, Token),
    This(Token),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Expression {
        expression: Expr,
    },
    Print {
        expression: Expr,
    },
    Var {
        name: Token,
        initializer: Option<Expr>,
    },
    Return {
        keyword: Token,
        value: Option<Expr>,
    },
    Block {
        statements: Vec<Stmt>,
    },
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    While {
        condition: Expr,
        body: Box<Stmt>,
    },
    Function {
        name: Token,
        params: Vec<Token>,
        body: Vec<Stmt>,
    },
    Class {
        name: Token,
        superclass: Option<Expr>,
        methods: Vec<Stmt>,
    },
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Binary(left, op, right) => write!(f, "({} {} {})", op.lexeme, left, right),
            Expr::Grouping(expr) => write!(f, "(group {})", expr),
            Expr::Literal(lit) => write!(f, "{}", lit),
            Expr::Unary(op, expr) => write!(f, "({} {})", op.lexeme, expr),
            Expr::Variable(name) => write!(f, "{}", name.lexeme),
            Expr::Assign(name, expr) => write!(f, "(assign {} {})", name.lexeme, expr),
            Expr::Logical(left, op, right) => write!(f, "({} {} {})", op.lexeme, left, right),
            Expr::Call(callee, _paren, args) => {
                let arg_str: Vec<String> = args.iter().map(|a| format!("{}", a)).collect();
                write!(f, "({} {})", callee, arg_str.join(" "))
            }
            Expr::Get(object, _token) => write!(f, "{}", object),
            Expr::Set(object, _token, value) => write!(f, "{} = {}", object, value),
            Expr::This(name) => write!(f, "{}", name.lexeme),
            Expr::Super(name, _token) => write!(f, "{}", name.lexeme),
        }
    }
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Stmt::Expression { expression } => write!(f, "{};", expression),
            Stmt::Print { expression } => write!(f, "(print {});", expression),
            Stmt::Var { name, initializer } => {
                if let Some(init) = initializer {
                    write!(f, "(var {} = {});", name.lexeme, init)
                } else {
                    write!(f, "(var {});", name.lexeme)
                }
            }
            Stmt::Block { statements } => {
                write!(f, "{{ ")?;
                for stmt in statements {
                    write!(f, "{} ", stmt)?;
                }
                write!(f, "}}")
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                if let Some(else_b) = else_branch {
                    write!(f, "(if {} {} else {})", condition, then_branch, else_b)
                } else {
                    write!(f, "(if {} {})", condition, then_branch)
                }
            }
            Stmt::While { condition, body } => {
                write!(f, "(while {} {})", condition, body)
            }
            Stmt::Function { name, params, body } => {
                let params_str: Vec<String> = params.iter().map(|p| p.lexeme.clone()).collect();
                write!(f, "(fun {}({}) ", name.lexeme, params_str.join(", "))?;
                for stmt in body {
                    write!(f, "{} ", stmt)?;
                }
                write!(f, ")")
            }
            Stmt::Return { keyword, value } => {
                if let Some(val) = value {
                    write!(f, "(return {} {})", keyword.lexeme, val)
                } else {
                    write!(f, "(return {})", keyword.lexeme)
                }
            }
            Stmt::Class {
                name,
                superclass,
                methods,
            } => {
                write!(f, "(class {} ", name.lexeme)?;
                if let Some(expr) = superclass {
                    write!(f, "superclass {} ", expr)?;
                }
                for stmt in methods {
                    write!(f, "{} ", stmt)?;
                }
                write!(f, ")")
            }
        }
    }
}

pub trait ExprVisitor<T> {
    fn visit_binary_expr(&mut self, left: &Expr, op: &Token, right: &Expr) -> T;
    fn visit_grouping_expr(&mut self, expression: &Expr) -> T;
    fn visit_literal_expr(&mut self, literal: &Literal) -> T;
    fn visit_unary_expr(&mut self, op: &Token, expr: &Expr) -> T;
    fn visit_variable_expr(&mut self, name: &Token) -> T;
    fn visit_assign_expr(&mut self, name: &Token, value: &Expr) -> T;
    fn visit_logical_expr(&mut self, left: &Expr, op: &Token, right: &Expr) -> T;
    fn visit_call_expr(&mut self, callee: &Expr, paren: &Token, arguments: &[Expr]) -> T;
    fn visit_get_expr(&mut self, object: &Expr, _name: &Token) -> T;
    fn visit_set_expr(&mut self, object: &Expr, _name: &Token, value: &Expr) -> T;
    fn visit_this_expr(&mut self, _name: &Token) -> T;
    fn visit_super_expr(&mut self, _name: &Token, _method: &Token) -> T;
}

pub trait StmtVisitor<T> {
    fn visit_expression_stmt(&mut self, expression: &Expr) -> T;
    fn visit_print_stmt(&mut self, expression: &Expr) -> T;
    fn visit_var_stmt(&mut self, name: &Token, initializer: &Option<Expr>) -> T;
    fn visit_block_stmt(&mut self, statements: &[Stmt]) -> T;
    fn visit_if_stmt(
        &mut self,
        condition: &Expr,
        then_branch: &Stmt,
        else_branch: &Option<Box<Stmt>>,
    ) -> T;
    fn visit_while_stmt(&mut self, condition: &Expr, body: &Stmt) -> T;
    fn visit_function_stmt(&mut self, name: &Token, params: &[Token], body: &[Stmt]) -> T;
    fn visit_return_stmt(&mut self, keyword: &Token, value: &Option<Expr>) -> T;
    fn visit_class_stmt(&mut self, name: &Token, superclass: &Option<Expr>, methods: &[Stmt]) -> T;
}

impl Stmt {
    pub fn accept<T>(&self, visitor: &mut impl StmtVisitor<T>) -> T {
        match self {
            Stmt::Expression { expression } => visitor.visit_expression_stmt(expression),
            Stmt::Print { expression } => visitor.visit_print_stmt(expression),
            Stmt::Var { name, initializer } => visitor.visit_var_stmt(name, initializer),
            Stmt::Block { statements } => visitor.visit_block_stmt(statements),
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => visitor.visit_if_stmt(condition, then_branch, else_branch),
            Stmt::While { condition, body } => visitor.visit_while_stmt(condition, body),
            Stmt::Function { name, params, body } => {
                visitor.visit_function_stmt(name, params, body)
            }
            Stmt::Return { keyword, value } => visitor.visit_return_stmt(keyword, value),
            Stmt::Class {
                name,
                superclass,
                methods,
            } => visitor.visit_class_stmt(name, superclass, methods),
        }
    }
}

impl Expr {
    pub fn accept<T>(&self, visitor: &mut impl ExprVisitor<T>) -> T {
        match self {
            Expr::Binary(left, op, right) => visitor.visit_binary_expr(left, op, right),
            Expr::Grouping(expr) => visitor.visit_grouping_expr(expr),
            Expr::Literal(lit) => visitor.visit_literal_expr(lit),
            Expr::Unary(op, expr) => visitor.visit_unary_expr(op, expr),
            Expr::Variable(name) => visitor.visit_variable_expr(name),
            Expr::Assign(name, value) => visitor.visit_assign_expr(name, value),
            Expr::Logical(left, op, right) => visitor.visit_logical_expr(left, op, right),
            Expr::Call(callee, paren, arguments) => {
                visitor.visit_call_expr(callee, paren, arguments)
            }
            Expr::Get(object, name) => visitor.visit_get_expr(object, name),
            Expr::Set(object, name, value) => visitor.visit_set_expr(object, name, value),
            Expr::This(name) => visitor.visit_this_expr(name),
            Expr::Super(keyword, method) => visitor.visit_super_expr(keyword, method),
        }
    }
}
