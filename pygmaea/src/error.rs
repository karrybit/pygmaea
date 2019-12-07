use crate::token::Token;
use crate::token_type::TokenType;

#[derive(Debug)]
pub enum ParseError {
    NoneToken,
    PeekToken(TokenType, Option<Box<Token>>),
    Statement(ParseStatementError),
    Expression(ParseExpressionError),
}

#[derive(Debug)]
pub enum ParseStatementError {
    Let,
    Return,
    Expression,
}

#[derive(Debug)]
pub enum ParseExpressionError {
    NoPrefix(Option<Box<Token>>),
    Prefix,
    Infix,
    Boolean,
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

impl std::error::Error for ParseStatementError {}
impl std::fmt::Display for ParseStatementError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParseStatementError::Let => write!(f, "failed to parse LetStatement."),
            ParseStatementError::Return => write!(f, "failed to parse ReturnStatement."),
            ParseStatementError::Expression => write!(f, "failed to parse ExpressionStatement."),
        }
    }
}

impl std::error::Error for ParseExpressionError {}
impl std::fmt::Display for ParseExpressionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParseExpressionError::NoPrefix(Some(token)) => {
                write!(f, "no prefix parse for {}.", token.token_type)
            }
            ParseExpressionError::Prefix => write!(f, "failed to parse prefix."),
            ParseExpressionError::Infix => write!(f, "failed to parse infix expression."),
            ParseExpressionError::Boolean => write!(f, "failed to parse boolean."),
            _ => write!(
                f,
                "occur something error that does not to be catched any patterns."
            ),
        }
    }
}
