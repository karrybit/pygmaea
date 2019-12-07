#[cfg(test)]
mod tests {
    use pygmaea::token_type::TokenType::*;
    use pygmaea::token_type::*;

    const TOKEN_TYPES: [TokenType; 27] = [
        Plus,
        Minus,
        Asterisk,
        Slash,
        Assign,
        Bang,
        LessThan,
        GreaterThan,
        Equal,
        NotEqual,
        Comma,
        Semicolon,
        LParen,
        RParen,
        LBrace,
        RBrace,
        True,
        False,
        Let,
        Function,
        If,
        Else,
        Return,
        Int,
        Ident,
        EOF,
        Illegal,
    ];

    const KEYWORD_TOKEN_TYPES: [TokenType; 8] =
        [True, False, Let, Function, If, Else, Return, Ident];

    #[test]
    fn test_is_keyword() {
        TOKEN_TYPES.iter().for_each(|token_type| {
            assert_eq!(
                KEYWORD_TOKEN_TYPES.contains(token_type),
                token_type.is_keyword()
            );
        })
    }

    #[test]
    fn test_is_int() {
        TOKEN_TYPES.iter().for_each(|token_type| {
            assert_eq!(token_type == &Int, token_type.is_int());
        })
    }

    #[test]
    fn test_is_eof() {
        TOKEN_TYPES.iter().for_each(|token_type| {
            assert_eq!(token_type == &EOF, token_type.is_eof());
        })
    }

    #[test]
    fn test_default() {
        let token_type: TokenType = TokenType::default();
        assert_eq!(token_type, Illegal);
    }

    #[test]
    fn test_keyword() {
        KEYWORDS.values().for_each(|keyword| {
            assert!(KEYWORD_TOKEN_TYPES.contains(keyword));
        })
    }

    #[test]
    fn test_display() {
        TOKEN_TYPES.iter().for_each(|token_type| match token_type {
            Plus => assert_eq!("Plus", format!("{}", token_type)),
            Minus => assert_eq!("Minus", format!("{}", token_type)),
            Asterisk => assert_eq!("Asterisk", format!("{}", token_type)),
            Slash => assert_eq!("Slash", format!("{}", token_type)),
            Assign => assert_eq!("Assign", format!("{}", token_type)),
            Bang => assert_eq!("Bang", format!("{}", token_type)),
            LessThan => assert_eq!("LessThan", format!("{}", token_type)),
            GreaterThan => assert_eq!("GreaterThan", format!("{}", token_type)),
            Equal => assert_eq!("Equal", format!("{}", token_type)),
            NotEqual => assert_eq!("NotEqual", format!("{}", token_type)),
            LParen => assert_eq!("LParen", format!("{}", token_type)),
            RParen => assert_eq!("RParen", format!("{}", token_type)),
            LBrace => assert_eq!("LBrace", format!("{}", token_type)),
            RBrace => assert_eq!("RBrace", format!("{}", token_type)),
            Comma => assert_eq!("Comma", format!("{}", token_type)),
            Semicolon => assert_eq!("Semicolon", format!("{}", token_type)),
            True => assert_eq!("True", format!("{}", token_type)),
            False => assert_eq!("False", format!("{}", token_type)),
            Let => assert_eq!("Let", format!("{}", token_type)),
            Function => assert_eq!("Function", format!("{}", token_type)),
            If => assert_eq!("If", format!("{}", token_type)),
            Else => assert_eq!("Else", format!("{}", token_type)),
            Return => assert_eq!("Return", format!("{}", token_type)),
            Int => assert_eq!("Int", format!("{}", token_type)),
            Ident => assert_eq!("Ident", format!("{}", token_type)),
            EOF => assert_eq!("EOF", format!("{}", token_type)),
            Illegal => assert_eq!("Illegal", format!("{}", token_type)),
        });
    }
}
