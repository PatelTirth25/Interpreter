use std::process::exit;

use crate::token::{token_types::TokenType, Token};

#[derive(Debug)]
pub enum NZErrors {
    ParseError(Token, String),
    RuntimeError(Token, String),
    FileReadError(String),
}

fn print_error(errtype: &str, token: &Token, message: &str) {
    if token.token_type == TokenType::EOF {
        println!("{} Error at end: {}", errtype, message);
    } else {
        println!("{} at '{}': {}", errtype, token.line, message);
    }
    if errtype == "Runtime Error" {
        exit(69);
    }
    exit(1);
}

impl NZErrors {
    pub fn report_error(&self) {
        match self {
            NZErrors::ParseError(token, message) => print_error("Parse Error", token, message),
            NZErrors::RuntimeError(token, message) => print_error("Runtime Error", token, message),
            NZErrors::FileReadError(message) => println!("{}", message),
        }
    }
}
