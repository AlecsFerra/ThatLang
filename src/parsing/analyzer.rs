use crate::option_propagate_failure_to_option;
use crate::parsing::ast::{AST, Expression, Type};
use crate::parsing::symbol_table::SymbolTable;
use crate::parsing::token::Operator;
use crate::result_propagate_failure_to_option;
use crate::result_propagate_failure_to_result;

pub struct StaticAnalyzer {
    symbol_table: SymbolTable
}

impl StaticAnalyzer {
    pub fn new() -> Self {
        Self {
            symbol_table: SymbolTable::new()
        }
    }

    pub fn analyze(&mut self, ast: AST) -> Option<String> {
        match ast {
            AST::VarDeclaration(d_type, id) => {
                if !self.symbol_table.declare(id.clone(), d_type) {
                    return Some(format!("Cannot declare variable {} because it was already declared in this scope",
                                        id.clone()));
                }
                None
            }
            AST::VarDeclarationAndAssignment(d_type, id, expr) => {
                if !self.symbol_table.declare(id.clone(), d_type.clone()) {
                    return Some(format!("Cannot declare variable {} because it was already declared in this scope",
                                        id.clone()));
                }
                let expr_type = result_propagate_failure_to_option!(self.analyze_expression(expr));
                if d_type.clone() != expr_type {
                    return Some(format!("Mismatched types variable {} was declared {} but assigned {}",
                                        id, d_type.clone(), expr_type));
                }
                None
            }
            AST::Assign(id, expr) => {
                let id_type = result_propagate_failure_to_option!(self.symbol_table.retrieve_type(id.clone()),
                                                                format!("Cannot assign to undeclared variable {}", id.clone()));
                let expr_type = result_propagate_failure_to_option!(self.analyze_expression(expr));
                if id_type != expr_type {
                    return Some(format!("Mismatched types variable {} vas declared {} but assigned {}",
                                        id.clone(), id_type, expr_type));
                }
                None
            }
            AST::Print(expr) => match self.analyze_expression(expr) {
                Err(err) => Some(err),
                Ok(_) => None
            },
            AST::Block(nodes) => {
                for node in nodes {
                    match self.analyze(node) {
                        Some(err) => return Some(err),
                        _ => ()
                    }
                }
                None
            }
            AST::IfStatement(cond, then) => {
                result_propagate_failure_to_option!(self.analyze_expression(cond));
                self.symbol_table.create_frame();
                option_propagate_failure_to_option!(self.analyze(*then));
                self.symbol_table.remove_frame();
                None
            }
            AST::WhileStatement(cond, body) => {
                result_propagate_failure_to_option!(self.analyze_expression(cond));
                self.symbol_table.create_frame();
                option_propagate_failure_to_option!(self.analyze(*body));
                self.symbol_table.remove_frame();
                None
            }
            AST::ForStatement(dec, cond, inc, body) => {
                self.symbol_table.create_frame();
                option_propagate_failure_to_option!(self.analyze(*dec));
                result_propagate_failure_to_option!(self.analyze_expression(cond));
                option_propagate_failure_to_option!(self.analyze(*inc));
                option_propagate_failure_to_option!(self.analyze(*body));
                self.symbol_table.remove_frame();
                None
            }
        }
    }

    fn analyze_expression(&self, expr: Expression) -> Result<Type, String> {
        match expr {
            Expression::IntegerLiteral(_) => Ok(Type::Integer),
            Expression::FloatLiteral(_) => Ok(Type::FloatingPoint),
            Expression::BooleanLiteral(_) => Ok(Type::Boolean),
            Expression::Variable(id) => match self.symbol_table.retrieve_type(id.clone()) {
                Some(t) => Ok(t),
                None => Err(format!("Use of undeclared variable {}", id.clone()))
            },
            Expression::BinaryOperation(left, op, right) => {
                let left = result_propagate_failure_to_result!(self.analyze_expression(*left));
                let right = result_propagate_failure_to_result!(self.analyze_expression(*right));
                self.analyze_operator(left, op, right)
            }
        }
    }

    fn analyze_operator(&self, l_type: Type, op: Operator, r_type: Type) -> Result<Type, String> {
        if l_type == Type::Unit || l_type.is_custom() {
            return Err(format!("Left operand cannot be subject of operator {}", op));
        }
        if r_type == Type::Unit || r_type.is_custom() {
            return Err(format!("Right operand cannot be subject of operator {}", op));
        }
        if r_type != l_type {
            return Err(format!("Unmatched values ({}, {}) in binary operator {}", l_type, r_type, op));
        }
        match op {
            Operator::Eq | Operator::Gt | Operator::Lt => Ok(Type::Boolean),
            Operator::And | Operator::Or => {
                if l_type == Type::FloatingPoint {
                    Err(format!("Could not perform bitwise  operations on floats"))
                } else {
                    Ok(l_type)
                }
            }
            Operator::Pow => Ok(l_type),
            Operator::Add | Operator::Sub | Operator::Mul | Operator::Div => {
                if l_type == Type::Boolean {
                    Err(format!("Could not perform mathematical operations on boolean"))
                } else {
                    Ok(l_type)
                }
            }
        }
    }
}
