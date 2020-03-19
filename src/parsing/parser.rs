use std::collections::HashMap;
use std::iter::Peekable;
use std::slice::Iter;

use lazy_static::lazy_static;

use crate::parsing::ast::{AST, Expression, Type};
use crate::parsing::token::{Token, TokenType};

lazy_static! {
    static ref PREDEFINED_TYPES: HashMap<&'static str, Type> = {
        let mut m = HashMap::new();
        m.insert("int",Type::Integer);
        m.insert("float",Type::FloatingPoint);
        m.insert("unit", Type::Unit);
        m.insert("bool", Type::Boolean);
        m
    };
}

pub struct Parser<'a> {
    tokens: Peekable<Iter<'a, Token>>
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Self {
            tokens: tokens.iter().peekable()
        }
    }
}

impl Parser<'_> {
    pub fn parse(&mut self) -> Result<AST, String> {
        self.parse_block()
    }

    fn parse_block(&mut self) -> Result<AST, String> {
        let mut statements: Vec<AST> = Vec::new();
        while let Some(parsed_statement) = self.parse_statement() {
            match parsed_statement {
                Err(msg) => return Err(msg),
                Ok(parsed) => statements.push(parsed)
            }
        }
        Ok(AST::Block(statements))
    }

    fn parse_statement(&mut self) -> Option<Result<AST, String>> {
        match self.tokens.peek() {
            None => None,
            Some(token) => {
                match &token.t_type {
                    TokenType::RCurlyBracket => None,
                    TokenType::Fn => Some(self.parse_function()),
                    TokenType::Id(_) => Some(self.parse_assignment_or_declaration()),
                    TokenType::If => Some(self.parse_if()),
                    TokenType::While => Some(self.parse_while()),
                    TokenType::For => Some(self.parse_for()),
                    _ => Some(Err(format!("Expected fn, identifier, if but found '{}' on line {} char {}",
                                          token.t_type, token.line, token.char)))
                }
            }
        }
    }

    fn parse_assignment_or_declaration(&mut self) -> Result<AST, String> {
        let type_or_id = match self.tokens.next() {
            Some(token) => match &token.t_type {
                TokenType::Id(type_or_id) => type_or_id.to_string(),
                _ => panic!("Parser bad state called expected identifier")
            },
            _ => panic!("Parser bad state called expected identifier")
        };
        match self.tokens.peek() {
            None => Err(format!("Expected ':=' or 'identifier' but EOF reached")),
            Some(token) => match &token.t_type {
                //Assignment
                TokenType::Assignment => self.parse_assignment(type_or_id),
                //Declaration
                TokenType::Id(id) => self.parse_declaration(type_or_id),
                unexpected => Err(format!("Expected ':=' or 'identifier' but {} found on line {} char {}",
                                          unexpected, token.line, token.char))
            }
        }
    }

    fn parse_assignment(&mut self, id: String) -> Result<AST, String> {
        self.tokens.next();
        let expr = match self.parse_expression() {
            Err(msg) => return Err(msg),
            Ok(expr) => expr
        };
        Ok(AST::Assign(id, expr))
    }

    fn parse_declaration(&mut self, type_name: String) -> Result<AST, String> {
        let (id, line, char) = match self.tokens.next() {
            Some(token) => match &token.t_type {
                TokenType::Id(id) => (id, token.line, token.char),
                _ => panic!("Parser bad state called expected identifier")
            },
            _ => panic!("Parser bad state called expected identifier")
        };
        let found_type = match PREDEFINED_TYPES.get(type_name.as_str()) {
            Some(found_type) => found_type.clone(),
            None => Type::Custom(type_name)
        };
        //Simple Declaration or Declaration + Assignment
        match self.tokens.peek() {
            None => Err(format!("Expected ':=' or ';' but EOF reached")),
            Some(token) => match &token.t_type {
                //Assignment declaration
                TokenType::Assignment => {
                    self.tokens.next();
                    let expr = match self.parse_expression() {
                        Err(msg) => return Err(msg),
                        Ok(expr) => expr
                    };
                    // self.expect(TokenType::Semicolon);
                    Ok(AST::VarDeclarationAndAssignment(found_type, id.to_string(), expr))
                },
                //Simple declaration
                _ => Ok(AST::VarDeclaration(found_type, id.to_string()))
            }
        }
    }

    fn parse_expression(&mut self) -> Result<Expression, String> {
        let mut output: Vec<Expression> = Vec::new();
        let mut operators: Vec<Token> = Vec::new();
        while let Some(token) = self.tokens.next() {
            match &token.t_type {
                TokenType::Semicolon | TokenType::LCurlyBracket => break,
                TokenType::Id(id) => output.push(Expression::Variable(id.to_string())),
                TokenType::BooleanLiteral(val) => output.push(Expression::BooleanLiteral(*val)),
                TokenType::IntegerLiteral(val) => output.push(Expression::IntegerLiteral(*val)),
                TokenType::FloatLiteral(val) => output.push(Expression::FloatLiteral(*val)),
                TokenType::LRoundBracket => operators.push(token.clone()),
                TokenType::RRoundBracket => {
                    if !tilt_until(&mut operators, &mut output, TokenType::LRoundBracket) {
                        return Err(format!("Expected ')' but found '{}' on line {} char {}",
                                           token.t_type, token.line, token.char));
                    }
                }
                TokenType::Operator(op, priority, left_ass) => {
                    while let Some(top) = operators.top() {
                        match top.t_type {
                            TokenType::LRoundBracket => break,
                            TokenType::Operator(op, o_priority, _) => {
                                if o_priority > *priority || (o_priority == *priority && *left_ass) {
                                    operators.pop();
                                    let right = output.pop().unwrap();
                                    let left = output.pop().unwrap();
                                    output.push(Expression::BinaryOperation(Box::new(left), op, Box::new(right)))
                                } else {
                                    break;
                                }
                            }
                            _ => unreachable!("This token must not be on the operator stack!")
                        }
                    }
                    operators.push(token.clone());
                }
                unexpected => return Err(format!("Expression: Unexpected '{}' found on line {} char {}",
                                                 token.t_type, token.line, token.char))
            }
        }
        while let Some(token) = operators.pop() {
            match token.t_type {
                TokenType::LRoundBracket => break,
                TokenType::Operator(op, _, _) => {
                    let right = output.pop().unwrap();
                    let left = output.pop().unwrap();
                    output.push(Expression::BinaryOperation(Box::new(left), op, Box::new(right)))
                }
                _ => unreachable!("This token must not be on the operator stack!")
            }
        }
        assert!(operators.is_empty()); //Ensure that the operator stack is empty as it should be.
        Ok(output.pop().unwrap())
    }

    fn parse_if(&mut self) -> Result<AST, String> {
        self.tokens.next();
        let clause = match self.parse_expression() {
            Err(msg) => return Err(msg),
            Ok(expr) => expr
        };
        let block = match self.parse_block() {
            Err(msg) => return Err(msg),
            Ok(block) => block
        };
        match self.expect(TokenType::RCurlyBracket) {
            Some(err) => return err,
            _ => ()
        };
        Ok(AST::IfStatement(clause, Box::new(block)))
    }

    fn parse_while(&mut self) -> Result<AST, String> {
        self.tokens.next();
        let clause = match self.parse_expression() {
            Err(msg) => return Err(msg),
            Ok(expr) => expr
        };
        let block = match self.parse_block() {
            Err(msg) => return Err(msg),
            Ok(block) => block
        };
        match self.expect(TokenType::RCurlyBracket) {
            Some(err) => return err,
            _ => ()
        };
        Ok(AST::WhileStatement(clause, Box::new(block)))
    }

    fn parse_for(&mut self) -> Result<AST, String> {
        self.tokens.next();
        let init = match self.parse_assignment_or_declaration() {
            Err(msg) => return Err(msg),
            Ok(expr) => expr
        };
        let clause = match self.parse_expression() {
            Err(msg) => return Err(msg),
            Ok(expr) => expr
        };
        let inc = match self.parse_assignment_or_declaration() {
            Err(msg) => return Err(msg),
            Ok(expr) => expr
        };
        let block = match self.parse_block() {
            Err(msg) => return Err(msg),
            Ok(block) => block
        };
        match self.expect(TokenType::RCurlyBracket) {
            Some(err) => return err,
            _ => ()
        };
        Ok(AST::ForStatement(Box::new(init), clause, Box::new(inc), Box::new(block)))
    }

    fn parse_function(&mut self) -> Result<AST, String> {
        unimplemented!()
    }

    fn expect(&mut self, expected: TokenType) -> Option<Result<AST, String>> {
        match self.tokens.next() {
            Some(token) => if token.t_type == expected { None } else {
                Some(Err(format!("Expected '{}' but '{}' found on line {} char {}",
                                 expected, token.t_type, token.line, token.char)))
            },
            _ => Some(Err(format!("Expected '{}' but EOF reached", expected)))
        }
    }
}

fn tilt_until(operators: &mut Vec<Token>, output: &mut Vec<Expression>, stop: TokenType) -> bool {
    while let Some(token) = operators.pop() {
        if token.t_type == stop {
            return true;
        }
        match token.t_type {
            TokenType::Operator(op, _, _) => {
                let r = output.pop().unwrap();
                let l = output.pop().unwrap();
                output.push(Expression::BinaryOperation(Box::new(l),
                                                        op,
                                                        Box::new(r)));
            }
            _ => unreachable!("Only operators should be on the operator stack")
        }
    }
    false
}

trait Stack<T> {
    fn top(&self) -> Option<T>;
}

impl<T: Clone> Stack<T> for Vec<T> {
    fn top(&self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        self.get(self.len() - 1).map(|value| value.clone())
    }
}
