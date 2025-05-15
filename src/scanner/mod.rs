use std::{collections::HashMap, process::exit};

use crate::token::{token_types::Token_Type, Literal, Token};

pub struct Scanner<'a> {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    keywords: HashMap<&'a str, Token_Type>,
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
                ("and", Token_Type::AND),
                ("class", Token_Type::CLASS),
                ("else", Token_Type::ELSE),
                ("false", Token_Type::FALSE),
                ("for", Token_Type::FOR),
                ("fun", Token_Type::FUN),
                ("if", Token_Type::IF),
                ("nil", Token_Type::NIL),
                ("or", Token_Type::OR),
                ("print", Token_Type::PRINT),
                ("return", Token_Type::RETURN),
                ("super", Token_Type::SUPER),
                ("this", Token_Type::THIS),
                ("true", Token_Type::TRUE),
                ("var", Token_Type::VAR),
                ("while", Token_Type::WHILE),
            ]),
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_end() {
            self.start = self.current;
            self.scan_tokens();
        }

        self.tokens.push(Token::new(
            Token_Type::EOF,
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
                '(' => self.add_token(Token_Type::LEFT_PAREN, Literal::Nil),
                ')' => self.add_token(Token_Type::RIGHT_PAREN, Literal::Nil),
                '{' => self.add_token(Token_Type::LEFT_BRACE, Literal::Nil),
                '}' => self.add_token(Token_Type::RIGHT_BRACE, Literal::Nil),
                ',' => self.add_token(Token_Type::COMMA, Literal::Nil),
                '.' => self.add_token(Token_Type::DOT, Literal::Nil),
                '-' => self.add_token(Token_Type::MINUS, Literal::Nil),
                '+' => self.add_token(Token_Type::PLUS, Literal::Nil),
                ';' => self.add_token(Token_Type::SEMICOLON, Literal::Nil),
                '*' => self.add_token(Token_Type::STAR, Literal::Nil),
                '/' => {
                    if self.match_char('/') {
                        while self.peek() != Some('\n') && !self.is_end() {
                            self.next();
                        }
                    } else {
                        self.add_token(Token_Type::SLASH, Literal::Nil)
                    }
                }
                '!' => {
                    let p = self.match_char('=');
                    self.add_token(
                        if let true = p {
                            Token_Type::BANG_EQUAL
                        } else {
                            Token_Type::BANG
                        },
                        Literal::Nil,
                    )
                }

                '<' => {
                    let p = self.match_char('=');
                    self.add_token(
                        if let true = p {
                            Token_Type::LESS_EQUAL
                        } else {
                            Token_Type::LESS
                        },
                        Literal::Nil,
                    )
                }

                '>' => {
                    let p = self.match_char('=');
                    self.add_token(
                        if let true = p {
                            Token_Type::GREATER_EQUAL
                        } else {
                            Token_Type::GREATER
                        },
                        Literal::Nil,
                    )
                }

                '=' => {
                    let p = self.match_char('=');
                    self.add_token(
                        if let true = p {
                            Token_Type::EQUAL_EQUAL
                        } else {
                            Token_Type::EQUAL
                        },
                        Literal::Nil,
                    )
                }

                'o' => {
                    let p = self.match_char('r');
                    self.add_token(Token_Type::OR, Literal::Nil);
                }

                '\n' => self.line += 1,
                '\t' => {}
                '\r' => {}
                ' ' => {}
                '"' => self.string(),

                _ => {
                    if Scanner::is_digit(c) {
                        self.number();
                    } else if Scanner::is_alpha(c) {
                        self.identifier()
                    } else {
                        self.report_error("Unexpected character")
                    }
                }
            },
            None => self.report_error("Character not found"),
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

    fn add_token(&mut self, token_type: Token_Type, literal: Literal) {
        self.tokens.push(Token::new(
            token_type,
            self.source[self.start..self.current].to_string(),
            literal,
            self.line.try_into().unwrap(),
        ));
    }

    fn report_error(&self, message: &str) {
        println!("Line: {}, Error: {}", self.line, message);
        exit(1);
    }

    fn string(&mut self) {
        while self.peek() != Some('"') && !self.is_end() {
            if self.peek() == Some('\n') {
                self.line += 1;
            }
            self.next();
        }
        if self.is_end() {
            self.report_error("Unterminated string.");
            return;
        }
        self.next();
        let value = self.source[self.start + 1..self.current - 1].to_string();
        println!("Value from string: {}", value);
        self.add_token(Token_Type::STRING, Literal::String(value));
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
        self.add_token(Token_Type::NUMBER, Literal::Number(value.parse().unwrap()));
    }

    fn identifier(&mut self) {
        while Scanner::is_alphanumeric(self.peek().unwrap()) {
            self.next();
        }

        let value = &self.source[self.start..self.current];
        let token = if let Some(token) = self.keywords.get(value) {
            token.to_owned()
        } else {
            Token_Type::IDENTIFIER
        };
        self.add_token(token, Literal::Nil);
    }
}
