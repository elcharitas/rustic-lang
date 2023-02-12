use crate::enums::{Expression, Statement};
use crate::parser::Parser;
use std::collections::HashMap;

pub struct Interpreter<'a> {
    parser: &'a mut Parser<'a>,
    pub variables: HashMap<String, f64>,
}

impl<'a> Interpreter<'a> {
    pub fn new(parser: &'a mut Parser<'a>) -> Self {
        Interpreter {
            parser,
            variables: HashMap::new(),
        }
    }

    pub fn interpret(&mut self) -> Result<(), String> {
        let statements = self.parser.parse()?;
        for statement in statements {
            match statement {
                Statement::Expression(expression) => {
                    self.evaluate_expression(expression)?;
                }
                Statement::Assignment(name, expression) => {
                    let value = self.evaluate_expression(expression)?;
                    self.variables.insert(name, value);
                }
                Statement::Print(expression) => {
                    let value = self.evaluate_expression(expression)?;
                    println!("{}", value);
                }
            }
        }
        Ok(())
    }

    fn evaluate_expression(&mut self, expression: Expression) -> Result<f64, String> {
        match expression {
            Expression::Group(expression) => self.evaluate_expression(*expression),
            Expression::Number(value) => Ok(value),
            Expression::Variable(name) => {
                if let Some(value) = self.variables.get(&name) {
                    Ok(*value)
                } else {
                    Err(format!("Undefined variable: {}", name))
                }
            }
            Expression::Plus(left, right) => {
                let left = self.evaluate_expression(*left)?;
                let right = self.evaluate_expression(*right)?;
                Ok(left + right)
            }
            Expression::Minus(left, right) => {
                let left = self.evaluate_expression(*left)?;
                let right = self.evaluate_expression(*right)?;
                Ok(left - right)
            }
            Expression::Asterisk(left, right) => {
                let left = self.evaluate_expression(*left)?;
                let right = self.evaluate_expression(*right)?;
                Ok(left * right)
            }
            Expression::Slash(left, right) => {
                let left = self.evaluate_expression(*left)?;
                let right = self.evaluate_expression(*right)?;
                Ok(left / right)
            }
            Expression::Factorial(factor) => {
                let factor = self.evaluate_expression(*factor)? as i32;
                let result: i32 = (1..=factor).product();
                Ok(result as f64)
            }
            Expression::Power(_, _) => todo!(),
            Expression::None => Ok(0.0),
        }
    }
}

mod tests {
    #[test]
    fn test_interpreter() {
        use super::*;
        use crate::lexer::Lexer;

        let mut lexer = Lexer::new("a = 1 + 2 * 3 - 4 / 5");
        let mut parser = Parser::new(&mut lexer);
        let mut interpreter = Interpreter::new(&mut parser);
        interpreter.interpret().unwrap();
        assert_eq!(interpreter.variables.get("a"), Some(&3.0));
    }

    #[test]
    fn test_interpreter_print() {
        use super::*;
        use crate::lexer::Lexer;

        let mut lexer = Lexer::new("print 1");
        let mut parser = Parser::new(&mut lexer);
        let mut interpreter = Interpreter::new(&mut parser);
        interpreter.interpret().unwrap();
        assert_eq!(interpreter.variables.get("a"), None);
    }

    #[test]
    fn test_interpreter_factorial() {
        use super::*;
        use crate::lexer::Lexer;

        let mut lexer = Lexer::new("a = 5!");
        let mut parser = Parser::new(&mut lexer);
        let mut interpreter = Interpreter::new(&mut parser);
        interpreter.interpret().unwrap();
        assert_eq!(interpreter.variables.get("a"), Some(&120.0));
    }
}
