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
    Illegal,
    EOF,
    Ident,
    Int,
    Assign,
    Plus,
    Comma,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Function,
    Let,
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Illegal => "Illegal",
                EOF => "EOF",
                Ident => "Ident",
                Int => "Int",
                Assign => "Assign",
                Plus => "Plus",
                Comma => "Comma",
                Semicolon => "Semicolon",
                LParen => "LParen",
                RParen => "RParen",
                LBrace => "LBrace",
                RBrace => "RBrace",
                Function => "Function",
                Let => "Let",
            }
        )
    }
}

lazy_static! {
    static ref TOKEN_TYPE_VALUE: HashMap<&'static str, TokenType> = [
        ("ILLEGAL", Illegal),
        ("EOF", EOF),
        ("IDENT", Ident),
        ("INT", Int),
        ("=", Assign),
        ("+", Plus),
        (",", Comma),
        (";", Semicolon),
        ("(", LParen),
        (")", RParen),
        ("{", LBrace),
        ("}", RBrace),
        ("FUNCTION", Function),
        ("LET", Let),
    ]
    .iter()
    .cloned()
    .collect();
    pub(crate) static ref KEYWORDS: HashMap<&'static str, TokenType> =
        [("fn", Function), ("let", Let),].iter().cloned().collect();
}
