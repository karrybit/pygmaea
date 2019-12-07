use crate::token_type::TokenType;
use std::cmp::Ordering;

#[derive(Eq)]
pub enum Precedence {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
}

impl Precedence {
    pub fn look_up_by(token_type: TokenType) -> Option<Precedence> {
        match token_type {
            TokenType::Equal => Some(Precedence::Equals),
            TokenType::NotEqual => Some(Precedence::Equals),
            TokenType::LessThan => Some(Precedence::LessGreater),
            TokenType::GreaterThan => Some(Precedence::LessGreater),
            TokenType::Plus => Some(Precedence::Sum),
            TokenType::Minus => Some(Precedence::Sum),
            TokenType::Slash => Some(Precedence::Product),
            TokenType::Asterisk => Some(Precedence::Product),
            _ => None,
        }
    }

    fn priority(&self) -> u32 {
        match self {
            Precedence::Lowest => 0,
            Precedence::Equals => 1,
            Precedence::LessGreater => 2,
            Precedence::Sum => 3,
            Precedence::Product => 4,
            Precedence::Prefix => 5,
            Precedence::Call => 6,
        }
    }
}

impl Ord for Precedence {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority().cmp(&other.priority())
    }
}

impl PartialOrd for Precedence {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Precedence {
    fn eq(&self, other: &Self) -> bool {
        self.priority() == other.priority()
    }
}
