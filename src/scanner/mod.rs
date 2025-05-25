use std::collections::HashMap;

use crate::{
    report_error,
    token::{token_types::TokenType, Literal, Token},
};

pub struct Scanner<'a> {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    keywords: HashMap<&'a str, TokenType>,
}

impl<'a> Scanner<'a> {
    pub fn new(source: String) -> Scanner<'a> {
        Scanner {
            source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
            keywords: HashMap::from([
                ("and", TokenType::AND),
                ("class", TokenType::CLASS),
                ("else", TokenType::ELSE),
                ("false", TokenType::FALSE),
                ("for", TokenType::FOR),
                ("fun", TokenType::FUN),
                ("if", TokenType::IF),
                ("nil", TokenType::NIL),
                ("or", TokenType::OR),
                ("print", TokenType::PRINT),
                ("return", TokenType::RETURN),
                ("super", TokenType::SUPER),
                ("this", TokenType::THIS),
                ("true", TokenType::TRUE),
                ("var", TokenType::VAR),
                ("while", TokenType::WHILE),
            ]),
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(
            TokenType::EOF,
            "".to_string(),
            Literal::Nil,
            self.line.try_into().unwrap(),
        ));
        self.tokens.clone()
    }

    fn scan_token(&mut self) {
        let c = self.next();

        match c {
            Some(c) => match c {
                '(' => self.add_token(TokenType::LEFTPAREN, Literal::Nil),
                ')' => self.add_token(TokenType::RIGHTPAREN, Literal::Nil),
                '{' => self.add_token(TokenType::LEFTBRACE, Literal::Nil),
                '}' => self.add_token(TokenType::RIGHTBRACE, Literal::Nil),
                ',' => self.add_token(TokenType::COMMA, Literal::Nil),
                '.' => self.add_token(TokenType::DOT, Literal::Nil),
                '-' => self.add_token(TokenType::MINUS, Literal::Nil),
                '+' => self.add_token(TokenType::PLUS, Literal::Nil),
                ';' => self.add_token(TokenType::SEMICOLON, Literal::Nil),
                '*' => self.add_token(TokenType::STAR, Literal::Nil),
                '/' => {
                    if self.match_char('/') {
                        while self.peek() != Some('\n') && !self.is_end() {
                            self.next();
                        }
                    } else {
                        self.add_token(TokenType::SLASH, Literal::Nil)
                    }
                }
                '!' => {
                    let p = self.match_char('=');
                    self.add_token(
                        if let true = p {
                            TokenType::BANGEQUAL
                        } else {
                            TokenType::BANG
                        },
                        Literal::Nil,
                    )
                }

                '<' => {
                    let p = self.match_char('=');
                    self.add_token(
                        if let true = p {
                            TokenType::LESSEQUAL
                        } else {
                            TokenType::LESS
                        },
                        Literal::Nil,
                    )
                }

                '>' => {
                    let p = self.match_char('=');
                    self.add_token(
                        if let true = p {
                            TokenType::GREATEREQUAL
                        } else {
                            TokenType::GREATER
                        },
                        Literal::Nil,
                    )
                }

                '=' => {
                    let p = self.match_char('=');
                    self.add_token(
                        if let true = p {
                            TokenType::EQUALEQUAL
                        } else {
                            TokenType::EQUAL
                        },
                        Literal::Nil,
                    )
                }

                '\n' => self.line += 1,
                '\t' | '\r' | ' ' => {}
                '"' => self.string(),

                _ => {
                    if Scanner::is_digit(c) {
                        self.number();
                    } else if Scanner::is_alpha(c) {
                        self.identifier()
                    } else {
                        report_error(&self.tokens[self.tokens.len() - 1], "Unexpected character.");
                    }
                }
            },
            None => report_error(&self.tokens[self.tokens.len() - 1], "Character not found."),
        }
    }

    fn is_alpha(c: char) -> bool {
        return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_';
    }

    fn is_digit(c: char) -> bool {
        return c >= '0' && c <= '9';
    }

    fn is_alphanumeric(c: char) -> bool {
        return Scanner::is_digit(c) || Scanner::is_alpha(c);
    }

    fn next(&mut self) -> Option<char> {
        self.current += 1;
        self.source.chars().nth(self.current - 1)
    }

    fn peek(&self) -> Option<char> {
        self.source.chars().nth(self.current)
    }

    fn peek_next(&self) -> Option<char> {
        self.source.chars().nth(self.current + 1)
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_end() {
            return false;
        }
        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn is_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn add_token(&mut self, token_type: TokenType, literal: Literal) {
        self.tokens.push(Token::new(
            token_type,
            self.source[self.start..self.current].to_string(),
            literal,
            self.line.try_into().unwrap(),
        ));
    }

    fn string(&mut self) {
        while self.peek() != Some('"') && !self.is_end() {
            if self.peek() == Some('\n') {
                self.line += 1;
            }
            self.next();
        }
        if self.is_end() {
            report_error(&self.tokens[self.tokens.len() - 1], "Unterminated string.");
            return;
        }
        self.next();
        let value = self.source[self.start + 1..self.current - 1].to_string();
        println!("Value from string: {}", value);
        self.add_token(TokenType::STRING, Literal::String(value));
    }

    fn number(&mut self) {
        while Scanner::is_digit(self.peek().unwrap()) {
            self.next();
        }
        if self.peek() == Some('.') && Scanner::is_digit(self.peek_next().unwrap()) {
            self.next();
            while Scanner::is_digit(self.peek().unwrap()) {
                self.next();
            }
        }
        let value = self.source[self.start..self.current].to_string();
        self.add_token(TokenType::NUMBER, Literal::Number(value.parse().unwrap()));
    }

    fn identifier(&mut self) {
        while Scanner::is_alphanumeric(self.peek().unwrap()) {
            self.next();
        }

        let value = &self.source[self.start..self.current];
        let token = if let Some(token) = self.keywords.get(value) {
            token.to_owned()
        } else {
            TokenType::IDENTIFIER
        };
        self.add_token(token, Literal::Nil);
    }
}
