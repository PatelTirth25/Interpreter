use core::fmt;

pub mod token_types;

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Number(n) => write!(f, "{}", n),
            Literal::String(s) => write!(f, "{}", s),
            Literal::Boolean(b) => write!(f, "{}", b),
            Literal::Nil => write!(f, "nil"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: token_types::TokenType,
    pub lexeme: String,
    pub literal: Literal,
    pub line: u32,
}

impl Token {
    pub fn new(
        token_type: token_types::TokenType,
        lexeme: String,
        literal: Literal,
        line: u32,
    ) -> Token {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }

    pub fn default() -> Token {
        Token {
            token_type: token_types::TokenType::EOF,
            lexeme: String::new(),
            literal: Literal::Nil,
            line: 0,
        }
    }
}
