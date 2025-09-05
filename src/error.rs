use std::process::exit;

use crate::{
    object::Object,
    token::{token_types::TokenType, Token},
};

pub enum NZErrors {
    ParseError(Token, String),
    RuntimeError(Token, String),
    FileReadError(String),
    Return(Object),
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
            NZErrors::Return(_) => panic!("Return should never be reported as an error!"),
        }
    }
}
