use rustic::lexer::Lexer;

#[test]
fn test_lexer_tokenize() {
    let test_cases = vec![
        (
            "1 + 2",
            vec![Token::Number(1.0), Token::Plus, Token::Number(2.0)],
        ),
        (
            "3 * 4",
            vec![Token::Number(3.0), Token::Asterisk, Token::Number(4.0)],
        ),
        (
            "5 / 6",
            vec![Token::Number(5.0), Token::Slash, Token::Number(6.0)],
        ),
        (
            "7 - 8",
            vec![Token::Number(7.0), Token::Minus, Token::Number(8.0)],
        ),
    ];

    for (input, expected) in test_cases {
        let lexer = Lexer::new(input);
        let tokens: Vec<Token> = lexer.collect();
        assert_eq!(tokens, expected);
    }
}
