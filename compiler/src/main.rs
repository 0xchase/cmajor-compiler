/*#[macro_use]
use pest_derive::*;*/

mod ast;
mod parser;
// pub mod interpreter;
pub mod jit;
pub mod vm;

use ast::*;
use jit::Jit;

fn main() {
    /*let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("No input file was provided");
        std::process::exit(-1);
    }*/

    let source = std::fs::read_to_string("test.cmajor").unwrap();
    let jit = Jit::from_source(&source).unwrap();

    println!("Got value {}", jit);
}

pub type Result<T> = anyhow::Result<T>;

pub trait Compile {
    type Output;

    fn from_ast(ast: Vec<Node>) -> Self::Output;

    fn from_source(source: &str) -> Self::Output {
        println!("Compiling the source: {}", source);
        let ast: Vec<Node> = parser::parse(source).unwrap();
        println!("{:?}", ast);
        Self::from_ast(ast)
    }
}