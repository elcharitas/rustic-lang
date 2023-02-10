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
                    statements.push(Statement::Expression(expression));
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
                    return Err(format!("parse::Unexpected token: {:?}", token));
                }
            }
        }
        Ok(statements)
    }

    fn parse_expression(&mut self) -> Result<Expression, String> {
        let mut expression = Expression::None;

        while let Some(token) = self.lexer.next_token() {
            match token {
                Token::Plus => {
                    let term = self.parse_term()?;
                    expression = Expression::Plus(Box::new(expression), Box::new(term));
                }
                Token::Minus => {
                    let term = self.parse_term()?;
                    expression = Expression::Minus(Box::new(expression), Box::new(term));
                }
                Token::LParen => {
                    expression = self.parse_expression()?;
                    if let Some(token) = self.lexer.next_token() {
                        match token {
                            Token::RParen => {
                                return Ok(expression);
                            }
                            _ => {
                                return Err(format!(
                                    "parse_expression::LParen::Unexpected token: {:?}",
                                    token
                                ));
                            }
                        }
                    }
                }
                _ => {
                    return self.parse_term();
                }
            }
        }

        Ok(expression)
    }

    fn parse_term(&mut self) -> Result<Expression, String> {
        let mut term = self.parse_factor()?;

        while let Some(token) = self.lexer.next_token() {
            match token {
                Token::Star => {
                    let factor = self.parse_factor()?;
                    term = Expression::Asterisk(Box::new(term), Box::new(factor));
                }
                Token::Slash => {
                    let factor = self.parse_factor()?;
                    term = Expression::Slash(Box::new(term), Box::new(factor));
                }
                _ => {
                    return Err(format!("parse_term::Unexpected token: {:?}", token));
                }
            }
        }

        Ok(term)
    }

    fn parse_factor(&mut self) -> Result<Expression, String> {
        let mut factor = self.parse_primary()?;

        while let Some(token) = self.lexer.next_token() {
            match token {
                Token::Char('^') => {
                    let primary = self.parse_primary()?;
                    factor = Expression::Power(Box::new(factor), Box::new(primary));
                }
                _ => {
                    return Err(format!("parse_factor::Unexpected token: {:?}", token));
                }
            }
        }

        Ok(factor)
    }

    fn parse_primary(&mut self) -> Result<Expression, String> {
        let mut primary = self.parse_number()?;

        while let Some(token) = self.lexer.next_token() {
            match token {
                Token::LParen => {
                    primary = self.parse_primary()?;
                    if let Some(token) = self.lexer.next_token() {
                        match token {
                            Token::RParen => {}
                            _ => {
                                return Err(format!(
                                    "parse_primary::LParen::Unexpected token: {:?}",
                                    token
                                ));
                            }
                        }
                    }
                }
                _ => {
                    return Err(format!("parse_primary::Unexpected token: {:?}", token));
                }
            }
        }

        Ok(primary)
    }

    fn parse_number(&mut self) -> Result<Expression, String> {
        let mut number = self.parse_digit()?;

        while let Some(token) = self.lexer.next_token() {
            match token {
                Token::Char('.') => {
                    let digit = self.parse_digit()?;
                    number = Expression::Decimal(Box::new(number), Box::new(digit));
                }
                _ => number = Expression::None,
            }
        }

        Ok(number)
    }

    fn parse_digit(&mut self) -> Result<Expression, String> {
        let mut digit = Expression::Number(0.0);

        while let Some(token) = self.lexer.next_token() {
            match token {
                Token::Number(value) => digit = Expression::Number(value),
                _ => digit = Expression::None,
            }
        }

        Ok(digit)
    }
}

mod tests {

    #[test]
    fn test_parse() {
        use super::*;
        let mut lexer = Lexer::new("x = 1 + 3");
        let mut parser = Parser::new(&mut lexer);
        let statements = parser.parse().unwrap();

        assert_eq!(
            statements,
            vec![Statement::Assignment(
                "x".to_string(),
                Expression::Plus(
                    Box::new(Expression::Number(1.0)),
                    Box::new(Expression::Asterisk(
                        Box::new(Expression::Number(2.0)),
                        Box::new(Expression::Number(3.0))
                    ))
                )
            )]
        );
    }
}
