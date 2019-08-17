use crate::token::Token;

trait Node {
    fn token_literal(&self) -> String;
}
trait Statement<T>: Node {}
trait Expression: Node {}

pub(crate) struct Program<T> {
    statements: Vec<Box<dyn Statement<T>>>,
}

impl<T> Program<T> {
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
impl<T, U> Statement<T> for LetStatement<U> where U: Expression {}

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
