use crate::token::Token;
use crate::token_type::TokenType;

#[derive(Debug)]
pub(crate) enum ParseError {
    NoneToken,
    PeekToken(TokenType, Option<Box<Token>>),
    Expression(ParseExpressionError),
}

#[derive(Debug)]
pub(crate) enum ParseExpressionError {
    NoPrefixParse(Option<Box<Token>>),
    InfixExpression,
}

impl std::error::Error for ParseError {}
impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParseError::NoneToken => write!(f, "expected token to be exist. got None instead."),
            ParseError::PeekToken(expect, Some(actual)) => write!(
                f,
                "expected next token to be {}, got {} instead.",
                expect, actual.token_type
            ),
            ParseError::Expression(e) => write!(f, "{}", e),
            _ => write!(
                f,
                "occur something error that does not to be catched any patterns."
            ),
        }
    }
}

impl std::error::Error for ParseExpressionError {}
impl std::fmt::Display for ParseExpressionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParseExpressionError::NoPrefixParse(Some(token)) => {
                write!(f, "no prefix parse for {}.", token.token_type)
            }
            ParseExpressionError::InfixExpression => write!(f, "failed to parse infix expression."),
            _ => write!(
                f,
                "occur something error that does not to be catched any patterns."
            ),
        }
    }
}
