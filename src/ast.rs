use crate::token::Token;

pub(crate) trait Node {
    fn token_literal(&self) -> String;
}
pub(crate) trait Statement: Node {}
trait Expression: Node {}

pub(crate) struct Program<T>
where
    T: Statement,
{
    statements: Vec<T>,
}

impl<T> Program<T>
where
    T: Statement,
{
    fn token_literal(&self) -> String {
        self.statements
            .get(0)
            .map_or("".to_string(), |statement| statement.token_literal())
    }
}

struct LetStatement<T: Expression> {
    token: Token,
    name: Box<Identifier>,
    value: Box<T>,
}

impl<T> Node for LetStatement<T>
where
    T: Expression,
{
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}
impl<T> Statement for LetStatement<T> where T: Expression {}

struct Identifier {
    token: Token,
    value: String,
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Expression for Identifier {}
