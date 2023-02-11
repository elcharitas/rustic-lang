#[derive(Debug, PartialEq)]
pub enum Token {
    Char(char),
    Number(f64),
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
    Identifier(String),
    Equal,
    Semicolon,
    Print,
    Dot,
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Number(f64),
    Plus(Box<Expression>, Box<Expression>),
    Minus(Box<Expression>, Box<Expression>),
    Asterisk(Box<Expression>, Box<Expression>),
    Slash(Box<Expression>, Box<Expression>),
    Variable(String),
    Decimal(Box<Expression>),
    Power(Box<Expression>, Box<Expression>),
    None,
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Expression(Expression),
    Assignment(String, Expression),
}
