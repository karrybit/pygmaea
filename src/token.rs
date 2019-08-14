use lazy_static::lazy_static;
use std::collections::HashMap;
use TokenType::*;

pub(crate) struct Token {
    token_type: TokenType,
    literal: String,
}

#[derive(Clone)]
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
