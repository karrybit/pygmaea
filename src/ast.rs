use crate::token::Token;

pub(crate) trait Node {
    fn token_literal(&self) -> String;
}

pub(crate) enum Statement {
    Let(LetStatement),
    Return(ReturnStatement),
}

impl Node for Statement {
    fn token_literal(&self) -> String {
        match self {
            Statement::Let(statement) => statement.token_literal(),
            Statement::Return(statement) => statement.token_literal(),
        }
    }
}

impl std::fmt::Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Statement::Let(statement) => write!(f, "{}", statement),
            Statement::Return(statement) => write!(f, "{}", statement),
        }
    }
}

pub(crate) struct LetStatement {
    pub(crate) token: Box<Token>,
    pub(crate) name: Identifier,
    pub(crate) value: Option<Expression>,
}

impl LetStatement {
    pub(crate) fn new(token: Box<Token>, name: Identifier, value: Option<Expression>) -> Self {
        Self { token, name, value }
    }
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl std::fmt::Display for LetStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "LetStatement")
    }
}

pub(crate) struct ReturnStatement {
    pub(crate) token: Box<Token>,
    pub(crate) return_value: Option<Expression>,
}

impl ReturnStatement {
    pub(crate) fn new(token: Box<Token>) -> Self {
        Self {
            token: token,
            return_value: None,
        }
    }
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl std::fmt::Display for ReturnStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ReturnStatement")
    }
}

pub(crate) enum Expression {
    Identifier(Identifier),
}

impl Node for Expression {
    fn token_literal(&self) -> String {
        match self {
            Expression::Identifier(identifier) => identifier.token_literal(),
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

pub(crate) type Program = Vec<Statement>;
