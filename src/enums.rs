#[derive(Debug, PartialEq)]
pub enum Token {
    Char(char),
    Number(f64),
    Identifier(String),
    Plus,
    Minus,
    Star,
    Power,
    Slash,
    LParen,
    RParen,
    Equal,
    End,
    Print,
    Dot,
    Factorial,
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
    Factorial(Box<Expression>),
    Group(Box<Expression>),
    None,
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Expression(Expression),
    Assignment(String, Expression),
    Print(Expression),
}
