/*#[macro_use]
use pest_derive::*;*/

mod ast;
mod parser;
// pub mod interpreter;
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

    println!("AST:");
    println!("{:?}", parser::parser().parse(source));

    // let jit = Jit::from_source(&source).unwrap();
    // println!("Got value {}", jit);
}

pub type Result<T> = anyhow::Result<T>;

/*pub trait Compile {
    type Output;

    fn from_ast(ast: Vec<Node>) -> Self::Output;

    fn from_source(source: &str) -> Self::Output {
        parser::parse(source);


        panic!("JIT not implemented");
        /*if let Ok(ast) = parser::parse(source) {
            println!("{:?}", ast);
            Self::from_ast(ast)
        } else {
            panic!("Failed to parse source");
        }*/
    }
}*/