use std::fmt::{Display, Formatter, Result};

#[derive(Clone, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    And,
    Or,
    Eq,
    Gt,
    Lt
}

impl Display for Operator {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Operator::Add => write!(f, "+"),
            Operator::Sub => write!(f, "-"),
            Operator::Mul => write!(f, "*"),
            Operator::Div => write!(f, "/"),
            Operator::Pow => write!(f, "^"),
            Operator::And => write!(f, "&"),
            Operator::Or => write!(f, "|"),
            Operator::Eq => write!(f, "="),
            Operator::Gt => write!(f, ">"),
            Operator::Lt => write!(f, "<")
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum TokenType {
    Id(String),
    LRoundBracket,
    RRoundBracket,
    LCurlyBracket,
    RCurlyBracket,
    IntegerLiteral(i32),
    FloatLiteral(f32),
    BooleanLiteral(bool),
    Operator(Operator, u8, bool),
    Print,
    Semicolon,
    Fn,
    Comma,
    Assignment,
    If,
    While,
    For
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            TokenType::Id(_) => write!(f, "identifier"),
            TokenType::LRoundBracket => write!(f, "("),
            TokenType::RRoundBracket => write!(f, ")"),
            TokenType::LCurlyBracket => write!(f, "{{"),
            TokenType::RCurlyBracket => write!(f, "}}"),
            TokenType::IntegerLiteral(_) => write!(f, "integer literal"),
            TokenType::FloatLiteral(_) => write!(f, "float literal"),
            TokenType::BooleanLiteral(_) => write!(f, "boolean literal"),
            TokenType::Operator(_, _, _) => write!(f, "operator"),
            TokenType::Semicolon => write!(f, ";"),
            TokenType::Fn => write!(f, "fn"),
            TokenType::Comma => write!(f, ","),
            TokenType::Assignment => write!(f, ":="),
            TokenType::If => write!(f, "if"),
            TokenType::While => write!(f, "while"),
            TokenType::For => write!(f, "for"),
            TokenType::Print =>  write!(f, "print"),
        }
    }
}

#[derive(Clone)]
pub struct Token {
    pub t_type: TokenType,
    pub line: u32,
    pub char: u32,
}

impl Token {
    pub fn new(t_type: TokenType, line: u32, char: u32) -> Self {
        Self {
            t_type,
            line,
            char,
        }
    }
}
