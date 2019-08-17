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

struct LetStatement<'a, T: Expression> {
    token: Token,
    name: &'a Identifier,
    value: &'a T,
}

impl<'a, T> Node for LetStatement<'a, T>
where
    T: Expression,
{
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}
impl<'a, T, U> Statement<T> for LetStatement<'a, U> where U: Expression {}

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
