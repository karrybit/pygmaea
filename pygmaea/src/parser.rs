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
        self.current_token = std::mem::replace(&mut self.peek_token, None);
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

        let let_token = match std::mem::replace(&mut self.current_token, None) {
            Some(token) => token,
            None => {
                self.errors.push(ParseError::NoneToken);
                return Err(ParseError::Statement(ParseStatementError::Let));
            }
        };

        self.next_token();
        let identifier = Identifier::new(match std::mem::replace(&mut self.current_token, None) {
            Some(token) => token,
            None => {
                self.errors.push(ParseError::NoneToken);
                return Err(ParseError::Statement(ParseStatementError::Let));
            }
        });

        self.next_token();

        if self.current_token_is(TokenType::Assign) {
            self.next_token();
            let expression = match self.parse_expression(Precedence::Lowest) {
                Ok(expression) => expression,
                Err(e) => {
                    self.errors.push(e);
                    return Err(ParseError::Statement(ParseStatementError::Let));
                }
            };
            Ok(Statement::Let(LetStatement::new(
                let_token, identifier, expression,
            )))
        } else {
            self.peek_error(TokenType::Assign);
            Err(ParseError::Statement(ParseStatementError::Let))
        }
    }

    fn parse_return_statement(&mut self) -> Result<Statement, ParseError> {
        let token = match std::mem::replace(&mut self.current_token, None) {
            Some(token) => token,
            None => {
                self.errors.push(ParseError::NoneToken);
                return Err(ParseError::Statement(ParseStatementError::Return));
            }
        };

        self.next_token();
        let expression = match self.parse_expression(Precedence::Lowest) {
            Ok(expression) => expression,
            Err(e) => {
                self.errors.push(e);
                return Err(ParseError::Statement(ParseStatementError::Return));
            }
        };

        Ok(Statement::Return(ReturnStatement::new(token, expression)))
    }

    fn parse_expression_statement(&mut self) -> Result<Statement, ParseError> {
        let expression = match self.parse_expression(Precedence::Lowest) {
            Ok(expression) => expression,
            Err(e) => {
                self.errors.push(e);
                return Err(ParseError::Statement(ParseStatementError::Expression));
            }
        };
        let statement = ExpressionStatement::new(expression);
        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }
        Ok(Statement::Expression(statement))
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Result<Box<Expression>, ParseError> {
        let token = match std::mem::replace(&mut self.current_token, None) {
            Some(token) => token,
            None => return Err(ParseError::NoneToken),
        };
        self.next_token();

        let mut left_expression = self.parse_prefix_expression(token)?;

        while !self.current_token_is(TokenType::Semicolon)
            && self.current_token.as_ref().map_or(false, |token| {
                Precedence::look_up_by(token.token_type).map_or(false, |looked_up_precedence| {
                    precedence < looked_up_precedence
                })
            })
        {
            let precedence = match Precedence::look_up_by(
                self.current_token
                    .as_ref()
                    .map_or(TokenType::Illegal, |token| token.token_type),
            ) {
                Some(precedence) => precedence,
                None => return Ok(left_expression),
            };

            let token = match std::mem::replace(&mut self.current_token, None) {
                Some(token) => token,
                None => return Err(ParseError::NoneToken),
            };

            self.next_token();

            left_expression = match self.parse_infix_expression(left_expression, token, precedence)
            {
                Ok(left_expression) => left_expression,
                Err(err) => {
                    self.errors.push(err);
                    return Err(ParseError::Expression(ParseExpressionError::Infix));
                }
            };
        }

        Ok(left_expression)
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
                let right_expresion = match self.parse_expression(Precedence::Prefix) {
                    Ok(right_expresion) => right_expresion,
                    Err(err) => {
                        self.errors.push(err);
                        return Err(ParseError::Expression(ParseExpressionError::Prefix));
                    }
                };
                Ok(Box::new(Expression::Prefix(PrefixExpression::new(
                    token,
                    right_expresion,
                ))))
            }
            TokenType::True | TokenType::False => Ok(Box::new(self.parse_boolean(token))),
            _ => Err(ParseError::Expression(ParseExpressionError::NoPrefix(
                std::mem::replace(&mut self.current_token, None),
            ))),
        }
    }

    fn parse_infix_expression(
        &mut self,
        left_expression: Box<Expression>,
        token: Box<Token>,
        precedence: Precedence,
    ) -> Result<Box<Expression>, ParseError> {
        let right_expresion = match self.parse_expression(precedence) {
            Ok(right_expresion) => right_expresion,
            Err(err) => {
                self.errors.push(err);
                return Err(ParseError::Expression(ParseExpressionError::Infix));
            }
        };

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
        self.errors.push(ParseError::PeekToken(
            token_type,
            std::mem::replace(&mut self.peek_token, None),
        ))
    }
}
