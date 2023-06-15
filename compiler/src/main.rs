/*#[macro_use]
use pest_derive::*;*/

mod ast;
mod parser;
pub mod interpreter;
pub mod jit;

use chumsky::Parser;

use ast::Expr;
// use jit::Jit;

fn main() {
    /*let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("No input file was provided");
        std::process::exit(-1);
    }*/

    let source = std::fs::read_to_string("test.cmajor").unwrap();

    println!("Parsing the source:");
    println!("{}", source);

    match parser::parser().parse(source) {
        Ok(ast) => {
            println!("AST:");
            println!("{:?}", &ast);
            match interpreter::eval(&ast) {
                Ok(v) => println!("Eval: {}", v),
                Err(e) => println!("Eval error: {}", e)
            }
        },
        Err(errs) => {
            for e in errs {
                println!("{}", e);
            }
        }
    }

    // let jit = Jit::from_source(&source).unwrap();
    // println!("Got value {}", jit);
}
