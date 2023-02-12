use crate::enums::{Expression, Statement, Token};
use crate::lexer::Lexer;

pub struct Parser<'a> {
    pub lexer: &'a mut Lexer<'a>,
    pub found_group: Vec<Expression>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer<'a>) -> Self {
        Parser {
            lexer,
            found_group: vec![],
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Statement>, String> {
        let mut statements = vec![];

        while let Some(token) = self.lexer.next_token() {
            match token {
                Token::End => {}
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
        let mut expression = self.parse_value()?;

        if let Some(token) = self.lexer.next_token() {
            match token {
                Token::Plus => {
                    let term = self.parse_value()?;
                    expression = Expression::Plus(Box::new(expression), Box::new(term));
                }
                Token::Minus => {
                    let term = self.parse_value()?;
                    expression = Expression::Minus(Box::new(expression), Box::new(term));
                }
                Token::Star => {
                    let term = self.parse_value()?;
                    expression = Expression::Asterisk(Box::new(expression), Box::new(term));
                }
                Token::Slash => {
                    let term = self.parse_value()?;
                    expression = Expression::Slash(Box::new(expression), Box::new(term));
                }
                Token::Power => {
                    let term = self.parse_value()?;
                    expression = Expression::Power(Box::new(expression), Box::new(term))
                }
                Token::Factorial => {
                    expression = Expression::Factorial(Box::new(expression));
                }
                Token::RParen => {
                    if let Some(term) = self.found_group.pop() {
                        expression = Expression::Group(Box::new(term));
                    } else {
                        return Err(format!("parse::Unexpected token: {:?}", token));
                    }
                }
                _ => {}
            }
        }

        if self.found_group.len() > 0 {
            return Err("parse::Unbalanced parentheses".to_owned());
        }

        Ok(expression)
    }

    fn parse_value(&mut self) -> Result<Expression, String> {
        let mut value = Expression::None;

        if let Some(token) = self.lexer.next_token() {
            match token {
                Token::Number(number) => {
                    value = Expression::Number(number);
                }
                Token::Identifier(name) => {
                    value = Expression::Variable(name);
                }
                Token::LParen => {
                    let term = self.parse_expression()?;
                    self.found_group.push(term);
                }
                _ => {}
            }
        }

        Ok(value)
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

    // parse variable and Print
    #[test]
    fn test_parse_variable() {
        use super::*;
        let mut lexer = Lexer::new("x = 1\nprint x");
        let mut parser = Parser::new(&mut lexer);
        let statements = parser.parse().unwrap();

        assert_eq!(
            statements,
            vec![
                Statement::Assignment("x".to_string(), Expression::Number(1.0)),
                Statement::Print(Expression::Variable("x".to_string()))
            ]
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
    fn test_parse_expression_asterisk() {
        use super::*;
        let mut lexer = Lexer::new("1 * 2");
        let mut parser = Parser::new(&mut lexer);
        let expression = parser.parse_expression().unwrap();

        assert_eq!(
            expression,
            Expression::Asterisk(
                Box::new(Expression::Number(1.0)),
                Box::new(Expression::Number(2.0))
            )
        );
    }

    #[test]
    fn test_parse_expression_slash() {
        use super::*;
        let mut lexer = Lexer::new("1 / 2");
        let mut parser = Parser::new(&mut lexer);
        let expression = parser.parse_expression().unwrap();

        assert_eq!(
            expression,
            Expression::Slash(
                Box::new(Expression::Number(1.0)),
                Box::new(Expression::Number(2.0))
            )
        );
    }

    // test parse expression with parenthesis
    #[test]
    fn test_parse_expression_parenthesis() {
        use super::*;
        let mut lexer = Lexer::new("(1 + 2)");
        let mut parser = Parser::new(&mut lexer);
        let expression = parser.parse_expression().unwrap();

        assert_eq!(
            expression,
            Expression::Group(Box::new(Expression::Plus(
                Box::new(Expression::Number(1.0)),
                Box::new(Expression::Number(2.0))
            )))
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

    #[test]
    fn test_parse_expression_factorial() {
        use super::*;
        let mut lexer = Lexer::new("1!");
        let mut parser = Parser::new(&mut lexer);
        let expression = parser.parse_expression().unwrap();

        assert_eq!(
            expression,
            Expression::Factorial(Box::new(Expression::Number(1.0)))
        );
    }

    #[test]
    fn test_parse_expression_power() {
        use super::*;
        let mut lexer = Lexer::new("1 ^ 2");
        let mut parser = Parser::new(&mut lexer);
        let expression = parser.parse_expression().unwrap();

        assert_eq!(
            expression,
            Expression::Power(
                Box::new(Expression::Number(1.0)),
                Box::new(Expression::Number(2.0))
            )
        );
    }
}
