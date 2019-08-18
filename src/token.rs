use lazy_static::lazy_static;
use std::collections::HashMap;
use TokenType::*;

#[derive(Debug, Eq, PartialEq, Default, Clone)]
pub(crate) struct Token {
    pub(crate) token_type: TokenType,
    pub(crate) literal: String,
}

impl Token {
    pub(crate) fn new(token_type: TokenType, literal: String) -> Self {
        Self {
            token_type,
            literal,
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[Type:{}, Literal: {}]", self.token_type, self.literal)
    }
}

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
            True | False | Let | Function | If | Else | Return | Ident => true,
            _ => false,
        }
    }

    pub(crate) fn is_int(self) -> bool {
        match self {
            Int => true,
            _ => false,
        }
    }

    pub(crate) fn is_eof(self) -> bool {
        match self {
            EOF => true,
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
                Plus => "Plus",
                Minus => "Minus",
                Asterisk => "Asterisk",
                Slash => "Slash",
                Assign => "Assign",
                Bang => "Bang",
                LT => "Less Than",
                GT => "Greater Than",
                Equal => "Equal",
                NotEqual => "Not Equal",
                LParen => "LParen",
                RParen => "RParen",
                LBrace => "LBrace",
                RBrace => "RBrace",
                Comma => "Comma",
                Semicolon => "Semicolon",
                True => "True",
                False => "False",
                Let => "Let",
                Function => "Function",
                If => "If",
                Else => "Else",
                Return => "Return",
                Int => "Int",
                Ident => "Ident",
                EOF => "EOF",
                Illegal => "Illegal",
            }
        )
    }
}

lazy_static! {
    pub(crate) static ref KEYWORDS: HashMap<&'static str, TokenType> = [
        ("true", True),
        ("false", False),
        ("let", Let),
        ("fn", Function),
        ("if", If),
        ("else", Else),
        ("return", Return),
    ]
    .iter()
    .cloned()
    .collect();
}
