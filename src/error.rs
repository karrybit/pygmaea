use crate::token::Token;
use crate::token_type::TokenType;

#[derive(Debug)]
pub(crate) enum ParseError {
    NoneTokenError,
    PeekTokenError(TokenType, Option<Box<Token>>),
    NoPrefixParseError(Option<Box<Token>>),
}

impl std::error::Error for ParseError {}
impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParseError::NoneTokenError => {
                write!(f, "expected token to be exist. got None instead.",)
            }
            ParseError::PeekTokenError(expect, Some(actual)) => write!(
                f,
                "expected next token to be {}, got {} instead.",
                expect, actual.token_type
            ),
            ParseError::NoPrefixParseError(Some(token)) => {
                write!(f, "no prefix parse for {}.", token.token_type)
            }
            _ => write!(
                f,
                "occur something error that does not to be catched any patterns"
            ),
        }
    }
}
