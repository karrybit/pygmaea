use lazy_static::lazy_static;
use std::collections::HashMap;
use TokenType::*;

#[derive(Debug, Eq, PartialEq)]
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
    Comma,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Let,
    Function,
    Int,
    Ident,
    EOF,
    Illegal,
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
                LParen => "LParen",
                RParen => "RParen",
                LBrace => "LBrace",
                RBrace => "RBrace",
                Comma => "Comma",
                Semicolon => "Semicolon",
                Let => "Let",
                Function => "Function",
                Int => "Int",
                Ident => "Ident",
                EOF => "EOF",
                Illegal => "Illegal",
            }
        )
    }
}

lazy_static! {
    static ref TOKEN_TYPE_VALUE: HashMap<&'static str, TokenType> = [
        ("+", Plus),
        ("-", Minus),
        ("*", Asterisk),
        ("/", Slash),
        ("=", Assign),
        ("!", Bang),
        ("<", LT),
        (">", GT),
        ("(", LParen),
        (")", RParen),
        ("{", LBrace),
        ("}", RBrace),
        (",", Comma),
        (";", Semicolon),
        ("LET", Let),
        ("FUNCTION", Function),
        ("INT", Int),
        ("IDENT", Ident),
        ("EOF", EOF),
        ("ILLEGAL", Illegal),
    ]
    .iter()
    .cloned()
    .collect();
    pub(crate) static ref KEYWORDS: HashMap<&'static str, TokenType> =
        [("fn", Function), ("let", Let),].iter().cloned().collect();
}
