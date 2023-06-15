use inkwell::{
    builder::Builder, context::Context, execution_engine::JitFunction,
    types::IntType, types::FloatType,
    values::{AnyValue, FloatValue}, values::IntValue, OptimizationLevel,
};

use crate::ast::Expr;

type JitFunc = unsafe extern "C" fn() -> i32;

pub struct Jit;

impl Jit {
    fn from_ast(ast: Vec<Expr>) -> Result<i32, String> {
        let context = Context::create();
        let module = context.create_module("calculator");

        let builder = context.create_builder();

        let execution_engine = module
            .create_jit_execution_engine(OptimizationLevel::None)
            .unwrap();

        let i32_type = context.i32_type();
        let f64_type = context.f64_type();
        let fn_type = i32_type.fn_type(&[], false);

        let function = module.add_function("jit", fn_type, None);
        let basic_block = context.append_basic_block(function, "entry");

        builder.position_at_end(basic_block);

        for node in ast {
            let recursive_builder = RecursiveBuilder::new(i32_type, f64_type, &builder);
            let return_value = recursive_builder.build(&node);
            builder.build_return(Some(&return_value));
        }

        println!(
            "Generated LLVM IR: {}",
            function.print_to_string().to_string()
        );

        unsafe {
            let jit_function: JitFunction<JitFunc> = execution_engine.get_function("jit").unwrap();

            Ok(jit_function.call())
        }
    }
}

struct RecursiveBuilder<'a> {
    i32_type: IntType<'a>,
    f64_type: FloatType<'a>,
    builder: &'a Builder<'a>,
}

impl<'a> RecursiveBuilder<'a> {
    pub fn new(i32_type: IntType<'a>, f64_type: FloatType<'a>, builder: &'a Builder) -> Self {
        Self { i32_type, f64_type, builder }
    }

    pub fn build(&self, ast: &Expr) -> FloatValue<'a> {
        let value = match ast {
            Expr::Num(f) => self.f64_type.const_float(*f),
            Expr::Var(n) => todo!(),
            Expr::Neg(n) => todo!(),
            Expr::Add(a, b) => todo!(),
            Expr::Sub(a, b) => todo!(),
            Expr::Mul(a, b) => todo!(),
            Expr::Div(a, b) => todo!(),
            Expr::Call(n, e) => todo!(),
            Expr::Let { name, rhs, then } => todo!(),
            Expr::Fn { name, args, body, then } => todo!(),
        };

        return FloatValue::from(value);

        /*match ast {
            Node::Int(n) => self.i32_type.const_int(*n as u64, true),
            Node::Bool(b) => self.i32_type.const_int(if *b { 1 } else { 0 }, false),
            Node::UnaryExpr { op, child } => {
                let child = self.build(child);
                match op {
                    Operator::Minus => child.const_neg(),
                    Operator::Plus => child,
                    Operator::Multiply => child,
                    Operator::Divide => child,
                    Operator::And => child,
                    Operator::Or => child
                }
            }
            Node::BinaryExpr { op, lhs, rhs } => {
                let left = self.build(lhs);
                let right = self.build(rhs);

                match op {
                    Operator::Plus => self.builder.build_int_add(left, right, "plus_temp"),
                    Operator::Minus => self.builder.build_int_sub(left, right, "minus_temp"),
                    Operator::Multiply => self.builder.build_int_mul(left, right, "multiply_temp"),
                    Operator::Divide => self.builder.build_int_signed_div(left, right, "divide_temp"),
                    Operator::And => self.builder.build_and(left, right, "and_temp"),
                    Operator::Or => self.builder.build_or(left, right, "or_temp"),
                }
            }
        }*/
    }
}
