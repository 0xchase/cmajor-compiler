use crate::ast::*;
use crate::vm::bytecode::Interpreter;
use crate::Compile;

struct Eval;

impl Eval {
    pub fn new() -> Self {
        Self
    }

    pub fn eval(&self, node: &Node) -> i32 {
        match node {
            Node::Int(n) => *n,
            Node::UnaryExpr { op, child } => {
                let child = self.eval(child);
                match op {
                    Operator::Plus => child,
                    Operator::Minus => -child,
                }
            }
            Node::BinaryExpr { op, lhs, rhs } => {
                let lhs_ret = self.eval(lhs);
                let rhs_ret = self.eval(rhs);

                match op {
                    Operator::Plus => lhs_ret + rhs_ret,
                    Operator::Minus => lhs_ret - rhs_ret,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        /*assert_eq!(Interpreter::from_source("1 + 2").unwrap() as i32, 3);
        // assert_eq!(Interpreter::source("(1 + 2)").unwrap() as i32, 3);
        assert_eq!(Interpreter::from_source("2 + (2 - 1)").unwrap() as i32, 3);
        assert_eq!(Interpreter::from_source("(2 + 3) - 1").unwrap() as i32, 4);
        assert_eq!(
            Interpreter::from_source("1 + ((2 + 3) - (2 + 3))").unwrap() as i32,
            1
        );*/
    }
}