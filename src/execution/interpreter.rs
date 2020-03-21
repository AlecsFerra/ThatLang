use crate::execution::memory::Memory;
use crate::execution::value::Value;
use crate::parsing::ast::{AST, Expression};
use crate::parsing::token::Operator;

pub struct Interpreter {
    memory: Memory
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            memory: Memory::new()
        }
    }

    pub fn eval(&mut self, ast: AST) {
        match ast {
            AST::VarDeclaration(_, id) => self.memory.declare(id),
            AST::VarDeclarationAndAssignment(_, id, expr) => self.memory.declare_assign(id, self.eval_expression(expr)),
            AST::Assign(id, expr) => self.memory.assign(id, self.eval_expression(expr)),
            AST::Print(expr) => {
                let to_print = match self.eval_expression(expr) {
                    Value::BooleanValue(val) => val.to_string(),
                    Value::FloatValue(val) => format!("{}", val),
                    Value::IntegerValue(val) => format!("{}", val),
                    Value::Unit => "unit".to_string()
                };
                println!("{}", to_print);
            }
            AST::Block(nodes) => {
                for node in nodes {
                    self.eval(node)
                }
            }
            AST::IfStatement(cond, then_clause) => {
                if self.eval_expression(cond).expect_bool() {
                    self.memory.create_frame();
                    self.eval(*then_clause);
                    self.memory.remove_frame();
                }
            }
            AST::WhileStatement(cond, body) => {
                self.memory.create_frame();
                while self.eval_expression(cond.clone()).expect_bool() {
                    self.eval(*body.clone())
                }
                self.memory.remove_frame();
            }
            AST::ForStatement(dec, cond, inc, body) => {
                self.memory.create_frame();
                self.eval(*dec);
                while self.eval_expression(cond.clone()).expect_bool() {
                    self.eval(*body.clone());
                    self.eval(*inc.clone());
                }
                self.memory.remove_frame();
            }
        }
    }

    fn eval_expression(&self, expr: Expression) -> Value {
        match expr {
            Expression::IntegerLiteral(val) => Value::IntegerValue(val),
            Expression::FloatLiteral(val) => Value::FloatValue(val),
            Expression::BooleanLiteral(val) => Value::BooleanValue(val),
            Expression::Variable(id) => self.memory.retrieve_val(id),
            Expression::BinaryOperation(left, op, right) => {
                let left = self.eval_expression(*left);
                let right = self.eval_expression(*right);
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
}
