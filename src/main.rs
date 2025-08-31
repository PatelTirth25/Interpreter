mod ast;
mod environment;
mod error;
mod interpreter;
mod object;
mod parser;
mod scanner;
mod token;

use std::{env::args, fs::read_to_string};

use error::NZErrors;
use interpreter::Interpreter;

fn main() {
    let args = args().collect::<Vec<String>>();
    if args.len() < 2 {
        eprintln!("Usage: cargo run -- (.nz file)");
        return;
    }
    let buffer = read_file(&args[1]).map_err(|e| e.report_error()).unwrap();
    println!("Content: {:?}", buffer);

    let tokens = scanner::Scanner::new(buffer)
        .scan_tokens()
        .map_err(|e| e.report_error())
        .unwrap();
    println!("Tokens: {:#?}", tokens);

    let mut parser = parser::Parser::new(tokens);
    let expr = parser.parse().map_err(|e| e.report_error()).unwrap();
    println!("Expr: {:#?}", expr);

    let mut interpreter = Interpreter::new();
    interpreter
        .interpret(&expr)
        .map_err(|e| e.report_error())
        .unwrap();

    // let mut asd = ast_print::AstPrint::new();
    // println!("Ast: {}", asd.print(&expr));

    // ast::ast_generator::AstGenerator::define_ast(
    //     "/home/tirth/code/interpreter/src/",
    //     "ast",
    //     &["Expr"],
    // )
    // .expect("Ast Generator Error");
}

fn read_file(path: &str) -> Result<String, NZErrors> {
    let buffer = read_to_string(path).map_err(|e| NZErrors::FileReadError(e.to_string()));
    buffer
}
