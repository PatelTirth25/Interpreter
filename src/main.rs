mod ast;
mod parser;
mod scanner;
mod token;

use std::{
    env::args,
    fs::read_to_string,
    io::{self},
    process::exit,
};

use ast::ast_print;
use token::{token_types::TokenType, Token};

fn main() {
    let args = args().collect::<Vec<String>>();
    if args.len() < 2 {
        eprintln!("Usage: cargo run -- (.nz file)");
        return;
    }
    let buffer = read_file(&args[1]).expect("Error Reading file!");
    println!("Content: {:?}", buffer);

    let tokens = scanner::Scanner::new(buffer).scan_tokens();
    println!("Tokens: {:#?}", tokens);

    let mut parser = parser::Parser::new(tokens);
    let expr = parser.parse();
    println!("Expr: {:#?}", expr);

    let mut asd = ast_print::AstPrint::new();
    println!("Ast: {}", asd.print(&expr.unwrap()));
}

fn read_file(path: &str) -> Result<String, io::Error> {
    let buffer = read_to_string(path)?;
    Ok(buffer)
}

pub fn report_error(token: &Token, message: &str) {
    if token.token_type == TokenType::EOF {
        println!("Error at end: {}", message);
    } else {
        println!("Error at '{}': {}", token.line, message);
    }
    exit(1);
}
