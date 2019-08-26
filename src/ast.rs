use crate::token::Token;

pub(crate) trait Node {
    fn token_literal(&self) -> String;
}

pub(crate) enum Statement {
    Let(LetStatement),
    Return(ReturnStatement),
    Expression(ExpressionStatement),
}

impl Node for Statement {
    fn token_literal(&self) -> String {
        match self {
            Statement::Let(statement) => statement.token_literal(),
            Statement::Return(statement) => statement.token_literal(),
            Statement::Expression(statement) => statement.token_literal(),
        }
    }
}

impl std::fmt::Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Statement::Let(statement) => write!(f, "{}", statement),
            Statement::Return(statement) => write!(f, "{}", statement),
            Statement::Expression(statement) => write!(f, "{}", statement),
        }
    }
}

pub(crate) struct LetStatement {
    pub(crate) token: Box<Token>,
    pub(crate) name: Identifier,
    pub(crate) value: Option<Box<Expression>>,
}

impl LetStatement {
    pub(crate) fn new(token: Box<Token>, name: Identifier, value: Option<Box<Expression>>) -> Self {
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
        write!(
            f,
            "{} {} = {};",
            self.token_literal(),
            self.name,
            self.value
                .as_ref()
                .map_or("".to_string(), |v| format!("{}", v))
        )
    }
}

pub(crate) struct ReturnStatement {
    pub(crate) token: Box<Token>,
    pub(crate) return_value: Option<Box<Expression>>,
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
        write!(
            f,
            "{} {};",
            self.token_literal(),
            self.return_value
                .as_ref()
                .map_or("".to_string(), |v| format!("{}", v))
        )
    }
}

pub(crate) struct ExpressionStatement {
    pub(crate) expression: Option<Box<Expression>>,
}

impl ExpressionStatement {
    pub(crate) fn new(expression: Option<Box<Expression>>) -> Self {
        Self { expression }
    }
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> String {
        self.expression
            .as_ref()
            .map_or("".to_string(), |expression| expression.token_literal())
    }
}

impl std::fmt::Display for ExpressionStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.expression
                .as_ref()
                .map_or("".to_string(), |v| format!("{}", v))
        )
    }
}

pub(crate) enum Expression {
    Identifier(Identifier),
    Integer(IntegerLiteral),
    Prefix(PrefixExpression),
}

impl Node for Expression {
    fn token_literal(&self) -> String {
        match self {
            Expression::Identifier(identifier) => identifier.token_literal(),
            Expression::Integer(integer_literal) => integer_literal.token_literal(),
            Expression::Prefix(prefix) => prefix.token_literal(),
        }
    }
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Expression::Identifier(identifier) => write!(f, "{}", identifier),
            Expression::Integer(integer_literal) => write!(f, "{}", integer_literal),
            Expression::Prefix(prefix) => write!(f, "{}", prefix),
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

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

pub(crate) struct IntegerLiteral {
    pub(crate) token: Box<Token>,
    pub(crate) value: i64,
}

impl IntegerLiteral {
    pub(crate) fn new(token: Box<Token>) -> Self {
        let value = token
            .literal
            .parse::<i64>()
            .expect(format!("could not parse {} as integer", token.literal).as_str());
        Self { token, value }
    }
}

impl Node for IntegerLiteral {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl std::fmt::Display for IntegerLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

pub(crate) struct PrefixExpression {
    pub(crate) token: Box<Token>,
    pub(crate) operator: String,
    pub(crate) right: Option<Box<Expression>>,
}

impl PrefixExpression {
    pub(crate) fn new(token: Box<Token>) -> Self {
        let operator = token.literal.clone();
        Self {
            token,
            operator,
            right: None,
        }
    }
}

impl Node for PrefixExpression {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl std::fmt::Display for PrefixExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "({}{})",
            self.operator,
            self.right
                .as_ref()
                .map_or("".to_string(), |right| format!("{}", right).to_string())
        )
    }
}

pub(crate) type Program = Vec<Statement>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::Token;
    use crate::token_type::TokenType;

    #[test]
    fn test_string() {
        let program: Program = vec![Statement::Let(LetStatement::new(
            Box::new(Token::new(TokenType::Let, "let".to_string())),
            Identifier::new(Box::new(Token::new(TokenType::Ident, "myVar".to_string()))),
            Some(Box::new(Expression::Identifier(Identifier::new(Box::new(
                Token::new(TokenType::Ident, "anotherVar".to_string()),
            ))))),
        ))];

        assert_eq!(
            "let myVar = anotherVar;".to_string(),
            format!("{}", program.get(0).unwrap())
        );
    }
}
