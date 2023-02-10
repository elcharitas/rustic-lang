use rustic::src::lexer::Lexer;
use rustic::src::parser::{Expression, Parser, Statement};

#[test]
fn test_parser_integer_expression() {
    let input = "5";

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    let expected = Expression::Integer(5);
    let result = parser.parse_expression().unwrap();
    assert_eq!(expected, result);
}

#[test]
fn test_parser_addition_expression() {
    let input = "5 + 3";

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    let expected = Expression::Addition(
        Box::new(Expression::Integer(5)),
        Box::new(Expression::Integer(3)),
    );
    let result = parser.parse_expression().unwrap();
    assert_eq!(expected, result);
}

#[test]
fn test_parser_subtraction_expression() {
    let input = "5 - 3";

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    let expected = Expression::Subtraction(
        Box::new(Expression::Integer(5)),
        Box::new(Expression::Integer(3)),
    );
    let result = parser.parse_expression().unwrap();
    assert_eq!(expected, result);
}

#[test]
fn test_parser_let_statement() {
    let input = "let x = 5;";

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    let expected = Statement::Let {
        name: "x".to_string(),
        value: Expression::Integer(5),
    };
    let result = parser.parse_statement().unwrap();
    assert_eq!(expected, result);
}
