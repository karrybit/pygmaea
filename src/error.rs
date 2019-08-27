use crate::token::Token;
use crate::token_type::TokenType;

#[derive(Debug)]
pub(crate) enum ParseError {
    NoneToken,
    NonePrecedence,
    PeekToken(TokenType, Option<Box<Token>>),
    NoPrefixParse(Option<Box<Token>>),
    FailedToParsePrefixExpression,
    FailedToParseInfixExpression,
}

impl std::error::Error for ParseError {}
impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParseError::NoneToken => write!(f, "expected token to be exist. got None instead."),
            ParseError::NonePrecedence => {
                write!(f, "expected precedence to be exist. got None instead.")
            }
            ParseError::PeekToken(expect, Some(actual)) => write!(
                f,
                "expected next token to be {}, got {} instead.",
                expect, actual.token_type
            ),
            ParseError::NoPrefixParse(Some(token)) => {
                write!(f, "no prefix parse for {}.", token.token_type)
            }
            ParseError::FailedToParsePrefixExpression => {
                write!(f, "failed to parse prefix expression.")
            }
            ParseError::FailedToParseInfixExpression => {
                write!(f, "failed to parse infix expression.")
            }
            _ => write!(
                f,
                "occur something error that does not to be catched any patterns."
            ),
        }
    }
}
