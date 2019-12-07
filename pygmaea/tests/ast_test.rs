#[cfg(test)]
mod tests {
    use pygmaea::ast::*;
    use pygmaea::token::Token;
    use pygmaea::token_type::TokenType;

    #[test]
    fn test_string() {
        let program: Program = vec![Statement::Let(LetStatement::new(
            Box::new(Token::new(TokenType::Let, "let".to_string())),
            Identifier::new(Box::new(Token::new(TokenType::Ident, "myVar".to_string()))),
            Box::new(Expression::Identifier(Identifier::new(Box::new(
                Token::new(TokenType::Ident, "anotherVar".to_string()),
            )))),
        ))];

        assert_eq!(
            "let myVar = anotherVar;".to_string(),
            format!("{}", program.get(0).unwrap())
        );
    }
}
