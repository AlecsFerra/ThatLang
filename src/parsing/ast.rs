use crate::parsing::token::Operator;
use std::fmt::{Display, Formatter, Result};

#[derive(Clone, PartialEq)]
pub enum Type {
    Integer,
    FloatingPoint,
    Unit,
    Boolean,
    Custom(String),
}

impl Type {
    pub fn is_custom(&self) -> bool {
        match self {
            Type::Custom(_) => true,
            _ => false
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Type::Integer => write!(f, "Integer"),
            Type::FloatingPoint => write!(f, "FloatingPoint"),
            Type::Unit => write!(f, "Unit"),
            Type::Boolean => write!(f, "Boolean"),
            Type::Custom(_) => write!(f, "Custom")
        }
    }
}

#[derive(Clone)]
pub enum Expression {
    IntegerLiteral(i32),
    FloatLiteral(f32),
    BooleanLiteral(bool),
    BinaryOperation(Box<Expression>, Operator, Box<Expression>),
    Variable(String),
}

#[derive(Clone)]
pub enum AST {
    Block(Vec<AST>),
    Assign(String, Expression),
    VarDeclaration(Type, String),
    Print(Expression),
    VarDeclarationAndAssignment(Type, String, Expression),
    IfStatement(Expression, Box<AST>),
    WhileStatement(Expression, Box<AST>),
    ForStatement(Box<AST>, Expression, Box<AST>, Box<AST>)
}