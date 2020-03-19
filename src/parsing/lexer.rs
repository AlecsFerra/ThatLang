use std::collections::HashMap;
use std::iter::Peekable;
use std::str::Chars;

use lazy_static::lazy_static;

use crate::parsing::token::{Operator, Token, TokenType};

lazy_static! {
    static ref RESERVED_KEYWORDS: HashMap<&'static str, TokenType> = {
        let mut m = HashMap::new();
        m.insert("Fn", TokenType::Fn);
        m.insert("true", TokenType::BooleanLiteral(true));
        m.insert("false", TokenType::BooleanLiteral(false));
        m.insert("if", TokenType::If);
        m.insert("while", TokenType::While);
        m.insert("for", TokenType::For);
        m
    };
}


pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
    current_line: u32,
    current_char: u32,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a String) -> Self {
        Self {
            chars: input.chars().peekable(),
            current_line: 0,
            current_char: 0,
        }
    }
}

impl Lexer<'_> {
    pub fn lex(&mut self) -> Result<Vec<Token>, String> {
        let mut lexeems = Vec::new();
        loop {
            match self.next() {
                Ok(token) => lexeems.push(token),
                Err(err) => return if err == "No tokens left to parse" { Ok(lexeems) } else { Err(err) }
            }
        }
    }

    fn next(&mut self) -> Result<Token, String> {
        match self.chars.peek() {
            None => Err("No tokens left to parse".to_string()),
            Some(next_char) => {
                match next_char {
                    ' ' | '\t' | '\n' => {
                        self.skip_char();
                        self.next()
                    }
                    ',' => self.skip_and_construct(TokenType::Comma),
                    ';' => self.skip_and_construct(TokenType::Semicolon),
                    '{' => self.skip_and_construct(TokenType::LCurlyBracket),
                    '}' => self.skip_and_construct(TokenType::RCurlyBracket),
                    '(' => self.skip_and_construct(TokenType::LRoundBracket),
                    ')' => self.skip_and_construct(TokenType::RRoundBracket),
                    '>' => self.skip_and_construct(TokenType::Operator(Operator::Gt, 0, false)),
                    '<' => self.skip_and_construct(TokenType::Operator(Operator::Lt, 0, false)),
                    '=' => self.skip_and_construct(TokenType::Operator(Operator::Eq, 0, false)),
                    '+' => self.skip_and_construct(TokenType::Operator(Operator::Add, 1, true)),
                    '-' => self.skip_and_construct(TokenType::Operator(Operator::Sub, 1, true)),
                    '&' => self.skip_and_construct(TokenType::Operator(Operator::And, 1, true)),
                    '|' => self.skip_and_construct(TokenType::Operator(Operator::Or, 1, true)),
                    '*' => self.skip_and_construct(TokenType::Operator(Operator::Mul, 2, true)),
                    '/' => self.skip_and_construct(TokenType::Operator(Operator::Div, 2, true)),
                    '^' => self.skip_and_construct(TokenType::Operator(Operator::Pow, 3, true)),
                    ':' => self.next_assignment(),
                    '0'..='9' => self.next_number(),
                    'a'..='z' | 'A'..='Z' => self.next_id(),
                    unexpected => Err(format!("Unexpected character '{}' at line {} char {}",
                                              unexpected, self.current_line, self.current_char))
                }
            }
        }
    }

    fn next_id(&mut self) -> Result<Token, String> {
        let mut identifier = "".to_string();
        while self.chars.peek().map_or(false, |c| c.is_alphabetic() || c.is_numeric()) {
            self.current_char += 1;
            identifier.push(self.chars.next().unwrap());
        }
        match RESERVED_KEYWORDS.get(identifier.as_str()) {
            Some(token_type) => self.construct_token(token_type.clone()),
            None => self.construct_token(TokenType::Id(identifier))
        }
    }

    fn next_number(&mut self) -> Result<Token, String> {
        let mut number = "".to_string();
        while self.chars.peek().map_or(false, |c| c.is_numeric()) {
            self.current_char += 1;
            number.push(self.chars.next().unwrap());
        }
        if self.chars.peek().map_or(false, |c| *c == '.') {
            self.chars.next();
            let mut mantissa = ".".to_string();
            while self.chars.peek().map_or(false, |c| c.is_numeric()) {
                self.current_char += 1;
                mantissa.push(self.chars.next().unwrap())
            }
            number.push_str(mantissa.as_ref());
            self.construct_token(TokenType::FloatLiteral(number.parse().unwrap()))
        } else {
            self.construct_token(TokenType::IntegerLiteral(number.parse().unwrap()))
        }
    }

    fn next_assignment(&mut self) -> Result<Token, String> {
        self.skip_char();
        match self.chars.next() {
            Some('=') => self.construct_token(TokenType::Assignment),
            Some(unexpected) => Err(format!("Unexpected '{}' while expecting '=' on line {} char {}",
                                            unexpected, self.current_line, self.current_char)),
            None => Err(format!("Reached EOF while expecting '=' on line {} char {}",
                                self.current_line, self.current_char))
        }
    }

    fn construct_token(&self, t_type: TokenType) -> Result<Token, String> {
        Ok(Token::new(t_type, self.current_line, self.current_char))
    }

    fn skip_and_construct(&mut self, t_type: TokenType) -> Result<Token, String> {
        self.skip_char();
        self.construct_token(t_type)
    }

    fn skip_char(&mut self) {
        match self.chars.next() {
            None => panic!("Lexer bad state, called skip_char but there are no chars left to consume"),
            Some(next_char) => {
                match next_char {
                    '\n' => {
                        self.current_char = 0;
                        self.current_line += 1;
                    }
                    _ => self.current_char += 1
                }
            }
        }
    }
}
