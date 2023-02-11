use crate::enums::{Expression, Statement, Token};
use crate::lexer::Lexer;

pub struct Parser<'a> {
    pub lexer: &'a mut Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer<'a>) -> Self {
        Parser { lexer }
    }

    pub fn parse(&mut self) -> Result<Vec<Statement>, String> {
        let mut statements = vec![];

        while let Some(token) = self.lexer.next_token() {
            match token {
                Token::Print => {
                    let expression = self.parse_expression()?;
                    statements.push(Statement::Print(expression));
                }
                Token::Identifier(name) => {
                    if let Some(token) = self.lexer.next_token() {
                        match token {
                            Token::Equal => {
                                let expression = self.parse_expression()?;
                                statements.push(Statement::Assignment(name, expression));
                            }
                            _ => {
                                return Err(format!("parse::Unexpected token: {:?}", token));
                            }
                        }
                    }
                }
                _ => {
                    let expression = self.parse_expression()?;
                    statements.push(Statement::Expression(expression));
                }
            }
        }
        Ok(statements)
    }

    fn parse_expression(&mut self) -> Result<Expression, String> {
        let mut expression = self.parse_number()?;

        if let Some(token) = self.lexer.next_token() {
            match token {
                Token::Plus => {
                    let term = self.parse_number()?;
                    println!("plus::term: {:?}", term);
                    expression = Expression::Plus(Box::new(expression), Box::new(term));
                }
                Token::Minus => {
                    let term = self.parse_number()?;
                    println!("minus::term: {:?}", term);
                    expression = Expression::Minus(Box::new(expression), Box::new(term));
                }
                Token::Star => {
                    let term = self.parse_number()?;
                    expression = Expression::Asterisk(Box::new(expression), Box::new(term));
                }
                Token::Slash => {
                    let term = self.parse_number()?;
                    expression = Expression::Slash(Box::new(expression), Box::new(term));
                }
                Token::LParen => {
                    expression = self.parse_expression()?;
                    if let Some(token) = self.lexer.next_token() {
                        match token {
                            Token::RParen => {}
                            _ => {
                                return Err(format!(
                                    "parse_expression::LParen::Unexpected token: {:?}",
                                    token
                                ));
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(expression)
    }

    fn parse_number(&mut self) -> Result<Expression, String> {
        let mut number = Expression::None;

        if let Some(token) = self.lexer.next_token() {
            println!("parse_number::token: {:?}", token);
            match token {
                Token::Number(value) => {
                    number = Expression::Number(value);
                }
                Token::Dot => {
                    number = Expression::Decimal(Box::new(number));
                }
                _ => {}
            }
        }

        Ok(number)
    }
}

mod tests {

    #[test]
    fn test_parse() {
        use super::*;
        let mut lexer = Lexer::new("x = 1");
        let mut parser = Parser::new(&mut lexer);
        let statements = parser.parse().unwrap();

        assert_eq!(
            statements,
            vec![Statement::Assignment(
                "x".to_string(),
                Expression::Number(1.0)
            )]
        );
    }

    #[test]
    fn test_parse_expression() {
        use super::*;
        let mut lexer = Lexer::new("1 + 2");
        let mut parser = Parser::new(&mut lexer);
        let expression = parser.parse_expression().unwrap();

        assert_eq!(
            expression,
            Expression::Plus(
                Box::new(Expression::Number(1.0)),
                Box::new(Expression::Number(2.0))
            )
        );
    }

    #[test]
    fn test_parse_expression_minus() {
        use super::*;
        let mut lexer = Lexer::new("1 - 2");
        let mut parser = Parser::new(&mut lexer);
        let expression = parser.parse_expression().unwrap();

        assert_eq!(
            expression,
            Expression::Minus(
                Box::new(Expression::Number(1.0)),
                Box::new(Expression::Number(2.0))
            )
        );
    }

    #[test]
    fn test_parse_print() {
        use super::*;
        let mut lexer = Lexer::new("print 1");
        let mut parser = Parser::new(&mut lexer);
        let statements = parser.parse().unwrap();

        assert_eq!(statements, vec![Statement::Print(Expression::Number(1.0))]);
    }
}
