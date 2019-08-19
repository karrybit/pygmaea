use crate::token_type::TokenType;

#[derive(Debug, Eq, PartialEq, Default, Clone)]
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

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[Type:{}, Literal: {}]", self.token_type, self.literal)
    }
}
