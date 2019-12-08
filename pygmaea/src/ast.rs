use crate::token::Token;

pub trait Node {
    fn token_literal(&self) -> String;
}

#[derive(Debug)]
pub enum Statement {
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

#[derive(Debug)]
pub struct LetStatement {
    pub token: Box<Token>,
    pub name: Identifier,
    pub value: Box<Expression>,
}

impl LetStatement {
    pub fn new(token: Box<Token>, name: Identifier, value: Box<Expression>) -> Self {
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
        )
    }
}

#[derive(Debug)]
pub struct ReturnStatement {
    pub token: Box<Token>,
    pub return_value: Box<Expression>,
}

impl ReturnStatement {
    pub fn new(token: Box<Token>, return_value: Box<Expression>) -> Self {
        Self {
            token,
            return_value,
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
        write!(f, "{} {};", self.token_literal(), self.return_value)
    }
}

#[derive(Debug)]
pub struct ExpressionStatement {
    pub expression: Box<Expression>,
}

impl ExpressionStatement {
    pub fn new(expression: Box<Expression>) -> Self {
        Self { expression }
    }
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> String {
        self.expression.token_literal()
    }
}

impl std::fmt::Display for ExpressionStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.expression)
    }
}

#[derive(Debug)]
pub enum Expression {
    Identifier(Identifier),
    Integer(IntegerLiteral),
    Prefix(PrefixExpression),
    Infix(InfixExpression),
    Boolean(Boolean),
}

impl Node for Expression {
    fn token_literal(&self) -> String {
        match self {
            Expression::Identifier(identifier) => identifier.token_literal(),
            Expression::Integer(integer_literal) => integer_literal.token_literal(),
            Expression::Prefix(prefix) => prefix.token_literal(),
            Expression::Infix(infix) => infix.token_literal(),
            Expression::Boolean(boolean) => boolean.token_literal(),
        }
    }
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Expression::Identifier(identifier) => write!(f, "{}", identifier),
            Expression::Integer(integer_literal) => write!(f, "{}", integer_literal),
            Expression::Prefix(prefix) => write!(f, "{}", prefix),
            Expression::Infix(infix) => write!(f, "{}", infix),
            Expression::Boolean(boolean) => write!(f, "{}", boolean),
        }
    }
}

#[derive(Debug)]
pub struct Identifier {
    pub token: Box<Token>,
    pub value: String,
}

impl Identifier {
    pub fn new(token: Box<Token>) -> Self {
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

#[derive(Debug)]
pub struct IntegerLiteral {
    pub token: Box<Token>,
    pub value: i64,
}

impl IntegerLiteral {
    pub fn new(token: Box<Token>) -> Self {
        let value = token
            .literal
            .parse::<i64>()
            .unwrap_or_else(|e| panic!("could not parse {} as integer", e));
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

#[derive(Debug)]
pub struct PrefixExpression {
    pub token: Box<Token>,
    pub operator: String,
    pub right: Box<Expression>,
}

impl PrefixExpression {
    pub fn new(token: Box<Token>, right: Box<Expression>) -> Self {
        let operator = token.literal.clone();
        Self {
            token,
            operator,
            right,
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
        write!(f, "({}{})", self.operator, self.right,)
    }
}

#[derive(Debug)]
pub struct InfixExpression {
    pub token: Box<Token>,
    pub left: Box<Expression>,
    pub operator: String,
    pub right: Box<Expression>,
}

impl InfixExpression {
    pub fn new(token: Box<Token>, left: Box<Expression>, right: Box<Expression>) -> Self {
        let operator = token.literal.clone();
        Self {
            token,
            left,
            operator,
            right,
        }
    }
}

impl Node for InfixExpression {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl std::fmt::Display for InfixExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({} {} {})", self.left, self.operator, self.right)
    }
}

#[derive(Debug)]
pub struct Boolean {
    pub token: Box<Token>,
    pub value: bool,
}

impl Boolean {
    pub fn new(token: Box<Token>, value: bool) -> Self {
        Self { token, value }
    }
}

impl std::fmt::Display for Boolean {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Node for Boolean {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

pub type Program = Vec<Statement>;

pub fn string(program: &[Statement]) -> String {
    program
        .iter()
        .fold(String::new(), |string, ast| format!("{}{}", string, ast))
}
