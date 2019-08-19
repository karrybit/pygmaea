use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
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
        match self {
            TokenType::Int => true,
            _ => false,
        }
    }

    pub(crate) fn is_eof(self) -> bool {
        match self {
            TokenType::EOF => true,
            _ => false,
        }
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
