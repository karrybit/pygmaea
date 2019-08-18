use crate::token::Token;

pub(crate) trait Node {
    fn token_literal(&self) -> String;
}

pub(crate) enum StatementKind {
    LetStatement(LetStatement),
}

impl Node for StatementKind {
    fn token_literal(&self) -> String {
        match self {
            StatementKind::LetStatement(statement) => statement.token_literal(),
        }
    }
}

pub(crate) struct LetStatement {
    pub(crate) token: Token,
    pub(crate) name: Identifier,
    pub(crate) value: Option<ExpressionKind>,
}

impl LetStatement {
    pub(crate) fn new(token: Token, name: Identifier, value: Option<ExpressionKind>) -> Self {
        Self { token, name, value }
    }
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

pub(crate) enum ExpressionKind {
    IdentifierExpression(Identifier),
}

impl Node for ExpressionKind {
    fn token_literal(&self) -> String {
        match self {
            ExpressionKind::IdentifierExpression(identifier) => identifier.token_literal(),
        }
    }
}

pub(crate) struct Identifier {
    pub(crate) token: Box<Token>,
    pub(crate) value: String,
}

impl Identifier {
    pub(crate) fn new(token: Box<Token>) -> Self {
        let value = token.literal.clone();
        Self { token, value }
    }
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

type Program = Vec<StatementKind>;
