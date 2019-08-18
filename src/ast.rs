use crate::token::Token;

pub(crate) trait Node {
    fn token_literal(&self) -> String;
}

trait Statement: Node {}
trait Expression: Node {}

pub(crate) enum StatementType {
    Let(LetStatement),
}

pub(crate) enum ExpressionType {
    Ident(Identifier),
}

pub(crate) struct Program {
    pub(crate) statements: Vec<StatementType>,
}

impl Program {
    fn token_literal(&self) -> String {
        self.statements
            .get(0)
            .map_or("".to_string(), |statement| match statement {
                StatementType::Let(statement) => statement.token_literal(),
            })
    }
}

pub(crate) struct LetStatement {
    pub(crate) token: Token,
    pub(crate) name: Box<Identifier>,
    pub(crate) value: Box<ExpressionType>,
}

impl Statement for LetStatement {}
impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

pub(crate) struct Identifier {
    pub(crate) token: Token,
    pub(crate) value: String,
}

impl Expression for Identifier {}
impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}
