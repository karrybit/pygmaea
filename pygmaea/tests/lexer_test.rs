#[cfg(test)]
mod tests {
    use pygmaea::token_type::TokenType;
    fn setup_input() -> String {
        "let five = 5;
        let ten = 10;

        let add = fn(x, y){
            x + y;
        };

        let result = add(five, ten);
        !-/*5;
        5 < 10 > 5;

        if (5 < 10) {
            return true;
        } else {
            return false;
        }

        10 == 10;
        10 !=9;
        "
        .to_string()
    }

    fn setup_expects() -> Vec<(TokenType, &'static str)> {
        use TokenType::*;
        vec![
            (Let, "let"),
            (Ident, "five"),
            (Assign, "="),
            (Int, "5"),
            (Semicolon, ";"),
            (Let, "let"),
            (Ident, "ten"),
            (Assign, "="),
            (Int, "10"),
            (Semicolon, ";"),
            (Let, "let"),
            (Ident, "add"),
            (Assign, "="),
            (Function, "fn"),
            (LParen, "("),
            (Ident, "x"),
            (Comma, ","),
            (Ident, "y"),
            (RParen, ")"),
            (LBrace, "{"),
            (Ident, "x"),
            (Plus, "+"),
            (Ident, "y"),
            (Semicolon, ";"),
            (RBrace, "}"),
            (Semicolon, ";"),
            (Let, "let"),
            (Ident, "result"),
            (Assign, "="),
            (Ident, "add"),
            (LParen, "("),
            (Ident, "five"),
            (Comma, ","),
            (Ident, "ten"),
            (RParen, ")"),
            (Semicolon, ";"),
            (Bang, "!"),
            (Minus, "-"),
            (Slash, "/"),
            (Asterisk, "*"),
            (Int, "5"),
            (Semicolon, ";"),
            (Int, "5"),
            (LessThan, "<"),
            (Int, "10"),
            (GreaterThan, ">"),
            (Int, "5"),
            (Semicolon, ";"),
            (If, "if"),
            (LParen, "("),
            (Int, "5"),
            (LessThan, "<"),
            (Int, "10"),
            (RParen, ")"),
            (LBrace, "{"),
            (Return, "return"),
            (True, "true"),
            (Semicolon, ";"),
            (RBrace, "}"),
            (Else, "else"),
            (LBrace, "{"),
            (Return, "return"),
            (False, "false"),
            (Semicolon, ";"),
            (RBrace, "}"),
            (Int, "10"),
            (Equal, "=="),
            (Int, "10"),
            (Semicolon, ";"),
            (Int, "10"),
            (NotEqual, "!="),
            (Int, "9"),
            (Semicolon, ";"),
            (EOF, ""),
        ]
    }

    fn exact_expect(input: &str, expect: &[(TokenType, &'static str)]) -> (String, String) {
        (
            input
                .chars()
                .partition::<String, _>(char::is_ascii_whitespace)
                .1,
            expect.iter().map(|expect| expect.1).collect(),
        )
    }

    #[test]
    fn test_next_token() {
        use pygmaea::lexer::Lexer;

        let input = setup_input();
        let expects = setup_expects();
        let (exact_input, exact_expect) = exact_expect(&input, &expects);
        assert_eq!(exact_input, exact_expect);

        let mut lexer = Lexer::new(input);

        expects.iter().enumerate().for_each(|(i, expect)| {
            let token = lexer.next_token();
            assert_eq!(
                expect.0, token.token_type,
                "tests[{}] - tokentype wrong. expected={}, got={}",
                i, expect.0, token.token_type
            );
            assert_eq!(
                expect.1, token.literal,
                "tests[{}] - literal wrong. expected={}, got={}",
                i, expect.1, token.literal
            );
        });
    }
}
