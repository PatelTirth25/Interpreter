pub mod token_types;

#[derive(Debug, Clone)]
pub enum Literal {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

#[derive(Debug, Clone)]
pub struct Token {
    token_type: token_types::Token_Type,
    lexeme: String,
    literal: Literal,
    line: u32,
}

impl Token {
    pub fn new(
        token_type: token_types::Token_Type,
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
