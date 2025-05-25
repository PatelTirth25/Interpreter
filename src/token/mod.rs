pub mod token_types;

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
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
}
