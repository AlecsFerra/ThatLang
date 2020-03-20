use std::collections::HashMap;

use crate::parsing::ast::{AST, Expression, Type};
use crate::parsing::token::Operator;

type STable = HashMap<String, Type>;

pub fn analyze(node: AST) -> Option<String> {
    let symbol_table: STable = HashMap::new();
    match analyze_ast(node, symbol_table){
        Err(err) =>  Some(err),
        Ok(_) => None
    }
}

fn analyze_ast(node: AST, symbol_table: STable) -> Result<STable, String> {
    match node {
        AST::Block(nodes) => {
            let mut n_symbol_table = symbol_table.clone();
            for node in nodes {
                //print!("Analyzing statement in block - ");
                n_symbol_table = match analyze_ast(node, n_symbol_table) {
                    Err(err) => return Err(err),
                    Ok(table) => table
                }
            }
            Ok(symbol_table)
        }
        AST::VarDeclaration(d_type, id) => {
            //println!("Analyzing declaration: {}", id);
            if symbol_table.contains_key(id.as_str()) {
                Err(format!("Variable {} was already declared.", id))
            } else {
                let mut n_symbol_table = symbol_table.clone();
                n_symbol_table.insert(id, d_type.clone());
                Ok(n_symbol_table)
            }
        }
        AST::Assign(id, expr) => {
            //println!("Analyzing assignment: {}", id);
            let var_type = match symbol_table.get(id.as_str()) {
                None => return Err(format!("Cannot assign to variable {} not declared in this scope.", id)),
                Some(exp_type) => exp_type
            };
            let exp_type = match analyze_expression(expr, symbol_table.clone()) {
                Err(err) => return Err(err),
                Ok(e_type) => e_type
            };
            if var_type.clone() != exp_type {
                Err(format!("{} type and {} type don't match", var_type, exp_type))
            } else {
                let mut n_symbol_table = symbol_table.clone();
                n_symbol_table.insert(id, var_type.clone());
                Ok(n_symbol_table)
            }
        }
        AST::VarDeclarationAndAssignment(d_type, id, expr) => {
            //println!("Analyzing declaration + assign: {}", id);
            if symbol_table.contains_key(id.as_str()) {
                return Err(format!("Variable {} was already declared.", id));
            }
            let exp_type = match analyze_expression(expr, symbol_table.clone()) {
                Err(err) => return Err(err),
                Ok(e_type) => e_type
            };
            if d_type.clone() != exp_type {
                Err(format!("{} type and {} type don't match", d_type, exp_type))
            } else {
                let mut n_symbol_table = symbol_table.clone();
                n_symbol_table.insert(id, d_type.clone());
                Ok(n_symbol_table)
            }
        },
        AST::Print(expr) => {
            match analyze_expression(expr, symbol_table.clone()) {
                Err(err) => Err(err),
                Ok(_) => Ok(symbol_table)
            }
        }
        AST::IfStatement(cond, block) => {
            //println!("Analyzing declaration if");
            let n_symbol = symbol_table.clone();
            match analyze_expression(cond, symbol_table) {
                Err(err) => Err(err),
                Ok(Type::Boolean) => match analyze_ast(*block, n_symbol.clone()) {
                    Err(err) => Err(err),
                    Ok(_) => Ok(n_symbol.clone())
                },
                Ok(f_type) => Err(format!("If statement's condition require boolean type expression but found {}",
                                          f_type))
            }
        },
        AST::WhileStatement(cond, block) => {
            //println!("Analyzing declaration while");
            let n_symbol = symbol_table.clone();
            match analyze_expression(cond, symbol_table) {
                Err(err) => Err(err),
                Ok(Type::Boolean) => match analyze_ast(*block, n_symbol.clone()) {
                    Err(err) => Err(err),
                    Ok(_) => Ok(n_symbol.clone())
                },
                Ok(f_type) => Err(format!("While statement's condition require boolean type expression but found {}",
                                          f_type))
            }
        },
        AST::ForStatement(ass, cond, inc, block) => {
            //println!("Analyzing declaration for");
            let old_table = symbol_table.clone();
            let n_symbol = match analyze_ast(*ass, old_table.clone()) {
                Err(err) => return Err(err),
                Ok(table) => table
            };
            match analyze_expression(cond, n_symbol.clone()) {
                Err(err) => return Err(err),
                Ok(Type::Boolean) => (),
                Ok(f_type) => return Err(format!("For statement's condition require boolean type expression but found {}",
                                                 f_type))
            };
            let n_symbol = match analyze_ast(*inc, n_symbol.clone()) {
                Err(err) => return Err(err),
                Ok(table) => table
            };
            match analyze_ast(*block, n_symbol) {
                Err(err) => Err(err),
                Ok(_) => Ok(old_table)
            }
        }
    }
}

fn analyze_expression(exp: Expression, symbol_table: STable) -> Result<Type, String> {
    match exp {
        Expression::IntegerLiteral(_) => Ok(Type::Integer),
        Expression::FloatLiteral(_) => Ok(Type::FloatingPoint),
        Expression::BooleanLiteral(_) => Ok(Type::Boolean),
        Expression::Variable(id) => match symbol_table.get(id.as_str()) {
            None => Err(format!("Use of undeclared variable {}", id)),
            Some(v_type) => Ok(v_type.clone())
        },
        Expression::BinaryOperation(left, op, right) => {
            let l_type = match analyze_expression(*left, symbol_table.clone()) {
                Err(msg) => return Err(msg),
                Ok(l_type) => l_type
            };
            let r_type = match analyze_expression(*right, symbol_table.clone()) {
                Err(msg) => return Err(msg),
                Ok(l_type) => l_type
            };
            analyze_binary(l_type, op, r_type)
        }
    }
}

fn analyze_binary(l_type: Type, op: Operator, r_type: Type) -> Result<Type, String> {
    if l_type == Type::Unit || l_type.is_custom() {
        return Err(format!("Left operand cannot be subject of operator {}", op));
    }
    if r_type == Type::Unit || r_type.is_custom() {
        return Err(format!("Right operand cannot be subject of operator {}", op));
    }
    if r_type != l_type {
        return Err(format!("Unmatched values ({}, {}) in binary operator {}", l_type, r_type, op))
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
        Operator::Add | Operator::Sub | Operator::Mul | Operator::Div  => {
            if l_type == Type::Boolean {
                Err(format!("Could not perform mathematical operations on boolean"))
            } else {
                Ok(l_type)
            }
        }
    }
}