use std::collections::HashMap;

use crate::parsing::ast::{AST, Expression};
use crate::parsing::token::Operator;
use crate::execution::value::Value;

type MTable = HashMap<String, Value>;

pub fn eval(ast: AST) -> () {
    let memory = HashMap::new();
    eval_ast(ast, memory);
}

fn eval_ast(ast: AST, memory: MTable) -> MTable {
    match ast {
        AST::Block(nodes) => {
            let mut n_mem = memory.clone();
            for node in nodes {
                n_mem = eval_ast(node, n_mem);
            }
            n_mem
        },
        AST::Assign(id, expr) => memory.clone().insert_and_self(id, eval_expression(expr, memory)),
        AST::VarDeclarationAndAssignment(_, id, expr) => memory.clone().insert_and_self(id, eval_expression(expr, memory)),
        AST::VarDeclaration(_, _) => memory,
        AST::Print(expr) => {
            let to_print = match eval_expression(expr, memory.clone()) {
                Value::BooleanValue(val) => val.to_string(),
                Value::FloatValue(val) => format!("{}", val),
                Value::IntegerValue(val) => format!("{}", val)
            };
            println!("{}", to_print);
            memory
        }
        AST::IfStatement(clause, then) => {
            if eval_expression(clause, memory.clone()).expect_bool() {
                eval_ast(*then, memory.clone());
            }
            memory.clone()
        },
        AST::WhileStatement(clause, body) => {
            let mut clause_value = eval_expression(clause.clone(), memory.clone()).expect_bool();
            let mut n_mem = memory.clone();
            while clause_value {
                n_mem = eval_ast(*body.clone(), n_mem.clone());
                clause_value = eval_expression(clause.clone(), n_mem.clone()).expect_bool();
            }
            memory.clone()
        },
        AST::ForStatement(init, clause, inc, body) => {
            let mut n_mem = eval_ast(*init, memory.clone());
            let mut clause_value = eval_expression(clause.clone(), n_mem.clone()).expect_bool();
            while clause_value {
                n_mem = eval_ast(*body.clone(), n_mem.clone());
                n_mem = eval_ast(*inc.clone(), n_mem.clone());
                clause_value = eval_expression(clause.clone(), n_mem.clone()).expect_bool();
            }
            memory.clone()
        }
    }
}

fn eval_expression(exp: Expression, memory: MTable) -> Value {
    match exp {
        Expression::IntegerLiteral(val) => Value::IntegerValue(val),
        Expression::FloatLiteral(val) => Value::FloatValue(val),
        Expression::BooleanLiteral(val) => Value::BooleanValue(val),
        Expression::Variable(id) => memory.get(id.as_str()).unwrap().clone(),
        Expression::BinaryOperation(left, op, right) => {
            let left = eval_expression(*left, memory.clone());
            let right = eval_expression(*right, memory.clone());
            match op {
                Operator::Add => left + right,
                Operator::Sub => left - right,
                Operator::Mul => left * right,
                Operator::Div => left / right,
                Operator::Pow => left ^ right,
                Operator::And => left & right,
                Operator::Or => left | right,
                Operator::Eq => Value::BooleanValue(left == right),
                Operator::Gt => Value::BooleanValue(left > right),
                Operator::Lt => Value::BooleanValue(left < right),
            }
        }
    }
}

trait InsertAndSelf {
    fn insert_and_self(self, key: String, value: Value) -> Self;
}

impl InsertAndSelf for MTable {
    fn insert_and_self(mut self, key: String, value: Value) -> Self {
        self.insert(key, value);
        self
    }
}