use crate::ast::*;
use crate::error::*;
use crate::lexer::Lexer;
use crate::precedence::Precedence;
use crate::token::Token;
use crate::token_type::TokenType;

pub struct Parser {
    lexer: Lexer,
    current_token: Option<Box<Token>>,
    peek_token: Option<Box<Token>>,
    pub errors: Vec<ParseError>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut parser = Self {
            lexer,
            current_token: Default::default(),
            peek_token: Default::default(),
            errors: vec![],
        };
        parser.next_token();
        parser.next_token();
        parser
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.take();
        self.peek_token = Some(Box::new(self.lexer.next_token()));
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program::new();
        while self
            .current_token
            .as_ref()
            .map_or(false, |token| !token.token_type.is_eof())
        {
            if let Ok(statement) = self.parse_statement() {
                program.push(statement);
            }
            self.next_token();
        }
        program
    }

    fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        match self.current_token {
            Some(ref token) if token.token_type == TokenType::Let => self.parse_let_statement(),
            Some(ref token) if token.token_type == TokenType::Return => {
                self.parse_return_statement()
            }
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_let_statement(&mut self) -> Result<Statement, ParseError> {
        if !self.peek_token_is(TokenType::Ident) {
            self.peek_error(TokenType::Ident);
            return Err(ParseError::Statement(ParseStatementError::Let));
        }

        let let_token = self.current_token.take().unwrap();
        self.next_token();

        let identifier_token = self.current_token.take().ok_or_else(|| {
            self.errors.push(ParseError::NoneToken);
            ParseError::Statement(ParseStatementError::Let)
        })?;
        let identifier = Identifier::new(identifier_token);
        self.next_token();

        if !self.current_token_is(TokenType::Assign) {
            self.peek_error(TokenType::Assign);
            return Err(ParseError::Statement(ParseStatementError::Let));
        }
        self.next_token();
        let expression = self.parse_expression(Precedence::Lowest).map_err(|e| {
            self.errors.push(e);
            ParseError::Statement(ParseStatementError::Let)
        })?;
        Ok(Statement::Let(LetStatement::new(
            let_token, identifier, expression,
        )))
    }

    fn parse_return_statement(&mut self) -> Result<Statement, ParseError> {
        let token = self.current_token.take().unwrap();
        self.next_token();

        let expression = self.parse_expression(Precedence::Lowest).map_err(|e| {
            self.errors.push(e);
            ParseError::Statement(ParseStatementError::Return)
        })?;

        Ok(Statement::Return(ReturnStatement::new(token, expression)))
    }

    fn parse_expression_statement(&mut self) -> Result<Statement, ParseError> {
        let expression = self.parse_expression(Precedence::Lowest).map_err(|e| {
            self.errors.push(e);
            ParseError::Statement(ParseStatementError::Expression)
        })?;
        let statement = ExpressionStatement::new(expression);
        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }
        Ok(Statement::Expression(statement))
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Result<Box<Expression>, ParseError> {
        let token = self.current_token.take().ok_or(ParseError::NoneToken)?;
        self.next_token();

        let mut expression = self.parse_prefix_expression(token)?;

        while !self.current_token_is(TokenType::Semicolon)
            && self
                .current_token
                .as_ref()
                .map(|token| token.token_type)
                .and_then(Precedence::look_up_by)
                .map_or(false, |looked_up_precedence| {
                    precedence < looked_up_precedence
                })
        {
            let token = self.current_token.clone().unwrap();
            let precedence = Precedence::look_up_by(token.token_type).unwrap();

            self.next_token();

            expression = self
                .parse_infix_expression(expression, token, precedence)
                .map_err(|e| {
                    self.errors.push(e);
                    ParseError::Expression(ParseExpressionError::Infix)
                })?;
        }

        Ok(expression)
    }

    fn parse_prefix_expression(
        &mut self,
        token: Box<Token>,
    ) -> Result<Box<Expression>, ParseError> {
        match token.token_type {
            TokenType::Ident => {
                let expression = Expression::Identifier(Identifier::new(token));
                Ok(Box::new(expression))
            }
            TokenType::Int => {
                let expression = Expression::Integer(IntegerLiteral::new(token));
                Ok(Box::new(expression))
            }
            TokenType::Bang | TokenType::Minus => {
                let expression = self.parse_expression(Precedence::Prefix).map_err(|e| {
                    self.errors.push(e);
                    ParseError::Expression(ParseExpressionError::Prefix)
                })?;
                Ok(Box::new(Expression::Prefix(PrefixExpression::new(
                    token, expression,
                ))))
            }
            TokenType::True | TokenType::False => Ok(Box::new(self.parse_boolean(token))),
            _ => Err(ParseError::Expression(ParseExpressionError::NoPrefix(
                token,
            ))),
        }
    }

    fn parse_infix_expression(
        &mut self,
        left_expression: Box<Expression>,
        token: Box<Token>,
        precedence: Precedence,
    ) -> Result<Box<Expression>, ParseError> {
        let right_expresion = self.parse_expression(precedence).map_err(|e| {
            self.errors.push(e);
            ParseError::Expression(ParseExpressionError::Infix)
        })?;

        Ok(Box::new(Expression::Infix(InfixExpression::new(
            token,
            left_expression,
            right_expresion,
        ))))
    }

    fn parse_boolean(&mut self, token: Box<Token>) -> Expression {
        let value = token.token_type == TokenType::True;
        Expression::Boolean(Boolean::new(token, value))
    }
}

// utility functions
impl Parser {
    fn current_token_is(&self, token_type: TokenType) -> bool {
        self.current_token
            .as_ref()
            .map_or(false, |token| token.token_type == token_type)
    }

    fn peek_token_is(&mut self, token_type: TokenType) -> bool {
        self.peek_token
            .as_ref()
            .map_or(false, |token| token.token_type == token_type)
    }

    fn peek_error(&mut self, token_type: TokenType) {
        self.errors
            .push(ParseError::PeekToken(token_type, self.peek_token.take()))
    }
}
