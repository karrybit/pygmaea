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

#[derive(Debug, Eq, PartialEq, Clone)]
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
}
