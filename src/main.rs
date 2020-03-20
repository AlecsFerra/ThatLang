use std::fs::File;
use std::io::Read;

use crate::parsing::lexer::Lexer;
use crate::parsing::parser::Parser;
use crate::parsing::analyzer::analyze;
use crate::execution::interpreter::Interpreter;

mod parsing;
mod execution;

fn main() {
    let mut file = File::open("example/fib.mps").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    let mut lexer = Lexer::new(&contents);
    match lexer.lex() {
        Err(error) => println!("ERROR while lexing: {}", error),
        Ok(tokens) => {
            let mut parser = Parser::new(&tokens);
            match parser.parse() {
                Err(error) => println!("ERROR while parsing: {}", error),
                Ok(ast) => match analyze(ast.clone()) {
                    Some(error) => println!("ERROR while analyzing: {}", error),
                    None => {
                        let mut interpreter = Interpreter::new();
                        interpreter.eval(ast)
                    }
                }
            }
        }
    }
}
