use crate::token_type::TokenType;

#[derive(Debug)]
pub(crate) enum ParseError {
    PeekTokenError(TokenType, TokenType),
    NoPrefixParseError(TokenType),
}

impl std::error::Error for ParseError {}
impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParseError::PeekTokenError(expect, actual) => write!(
                f,
                "expected next token to be {}, got {} instead",
                expect, actual
            ),
            ParseError::NoPrefixParseError(token_type) => {
                write!(f, "no prefix parse for {}.", token_type)
            }
        }
    }
}
