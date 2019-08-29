use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub(crate) enum TokenType {
    Plus,
    Minus,
    Asterisk,
    Slash,
    Assign,
    Bang,
    LT,
    GT,
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
}

impl TokenType {
    pub(crate) fn is_keyword(self) -> bool {
        match self {
            TokenType::True
            | TokenType::False
            | TokenType::Let
            | TokenType::Function
            | TokenType::If
            | TokenType::Else
            | TokenType::Return
            | TokenType::Ident => true,
            _ => false,
        }
    }

    pub(crate) fn is_int(self) -> bool {
        self == TokenType::Int
    }

    pub(crate) fn is_eof(self) -> bool {
        self == TokenType::EOF
    }
}

impl Default for TokenType {
    fn default() -> Self {
        TokenType::Illegal
    }
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TokenType::Plus => "Plus",
                TokenType::Minus => "Minus",
                TokenType::Asterisk => "Asterisk",
                TokenType::Slash => "Slash",
                TokenType::Assign => "Assign",
                TokenType::Bang => "Bang",
                TokenType::LT => "Less Than",
                TokenType::GT => "Greater Than",
                TokenType::Equal => "Equal",
                TokenType::NotEqual => "Not Equal",
                TokenType::LParen => "LParen",
                TokenType::RParen => "RParen",
                TokenType::LBrace => "LBrace",
                TokenType::RBrace => "RBrace",
                TokenType::Comma => "Comma",
                TokenType::Semicolon => "Semicolon",
                TokenType::True => "True",
                TokenType::False => "False",
                TokenType::Let => "Let",
                TokenType::Function => "Function",
                TokenType::If => "If",
                TokenType::Else => "Else",
                TokenType::Return => "Return",
                TokenType::Int => "Int",
                TokenType::Ident => "Ident",
                TokenType::EOF => "EOF",
                TokenType::Illegal => "Illegal",
            }
        )
    }
}

lazy_static! {
    pub(crate) static ref KEYWORDS: HashMap<&'static str, TokenType> = [
        ("true", TokenType::True),
        ("false", TokenType::False),
        ("let", TokenType::Let),
        ("fn", TokenType::Function),
        ("if", TokenType::If),
        ("else", TokenType::Else),
        ("return", TokenType::Return),
    ]
    .iter()
    .cloned()
    .collect();
}

#[cfg(test)]
mod tests {
    use super::*;
    fn setup_token_types() -> Vec<TokenType> {
        use TokenType::*;
        vec![
            Plus, Minus, Asterisk, Slash, Assign, Bang, LT, GT, Equal, NotEqual, Comma, Semicolon,
            LParen, RParen, LBrace, RBrace, True, False, Let, Function, If, Else, Return, Int,
            Ident, EOF, Illegal,
        ]
    }

    fn setup_keyword_token_types() -> Vec<TokenType> {
        use TokenType::*;
        vec![True, False, Let, Function, If, Else, Return, Ident]
    }

    #[test]
    fn test_is_keyword() {
        let keyword_token_types = setup_keyword_token_types();
        setup_token_types().into_iter().for_each(|token_type| {
            assert_eq!(
                keyword_token_types.contains(&token_type),
                token_type.is_keyword()
            );
        })
    }

    #[test]
    fn test_is_int() {
        use TokenType::*;
        setup_token_types().into_iter().for_each(|token_type| {
            assert_eq!(token_type == Int, token_type.is_int());
        })
    }

    #[test]
    fn test_is_eof() {
        use TokenType::*;
        setup_token_types().into_iter().for_each(|token_type| {
            assert_eq!(token_type == EOF, token_type.is_eof());
        })
    }

    #[test]
    fn test_default() {
        use TokenType::*;
        let token_type: TokenType = TokenType::default();
        assert_eq!(token_type, Illegal);
    }

    #[test]
    fn test_keyword() {
        let keyword_token_types = setup_keyword_token_types();
        KEYWORDS.values().for_each(|keyword| {
            assert!(keyword_token_types.contains(keyword));
        })
    }
}
