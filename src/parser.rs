use crate::ast::*;
use crate::error::*;
use crate::lexer::Lexer;
use crate::precedence::Precedence;
use crate::token::Token;
use crate::token_type::TokenType;

struct Parser {
    lexer: Lexer,
    current_token: Option<Box<Token>>,
    peek_token: Option<Box<Token>>,
    errors: Vec<ParseError>,
}

impl Parser {
    fn new(lexer: Lexer) -> Self {
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

    fn parse_program(&mut self) -> Program {
        let mut program = Program::new();
        while self
            .current_token
            .as_ref()
            .map_or(false, |token| !token.token_type.is_eof())
        {
            if let Some(statement) = self.parse_statement() {
                program.push(statement);
            }
            self.next_token();
        }
        program
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.current_token {
            Some(ref token) if token.token_type == TokenType::Let => self.parse_let_statement(),
            Some(ref token) if token.token_type == TokenType::Return => {
                self.parse_return_statement()
            }
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_let_statement(&mut self) -> Option<Statement> {
        if !self.peek_token_is(TokenType::Ident) {
            self.peek_error(TokenType::Ident);
            return None;
        }

        let let_token = match std::mem::replace(&mut self.current_token, None) {
            Some(token) => token,
            None => {
                self.errors.push(ParseError::NoneToken);
                return None;
            }
        };

        let identifier = Identifier::new(match std::mem::replace(&mut self.peek_token, None) {
            Some(token) => token,
            None => {
                self.errors.push(ParseError::NoneToken);
                return None;
            }
        });

        self.next_token();

        if self.peek_token_is(TokenType::Assign) {
            while !self.current_token_is(TokenType::Semicolon) {
                self.next_token();
            }
            Some(Statement::Let(LetStatement::new(
                let_token, identifier, None,
            )))
        } else {
            self.peek_error(TokenType::Assign);
            None
        }
    }

    fn parse_return_statement(&mut self) -> Option<Statement> {
        let statement =
            ReturnStatement::new(match std::mem::replace(&mut self.current_token, None) {
                Some(token) => token,
                None => {
                    self.errors.push(ParseError::NoneToken);
                    return None;
                }
            });

        self.next_token();
        while !self.current_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        Some(Statement::Return(statement))
    }

    fn parse_expression_statement(&mut self) -> Option<Statement> {
        let expression = match self.parse_expression(Precedence::Lowest) {
            Ok(expression) => expression,
            Err(e) => {
                self.errors.push(e);
                return None;
            }
        };
        let statement = ExpressionStatement::new(expression);
        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }
        Some(Statement::Expression(statement))
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
                    return Err(ParseError::Expression(
                        ParseExpressionError::InfixExpression,
                    ));
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
                        return Err(ParseError::Expression(
                            ParseExpressionError::InfixExpression,
                        ));
                    }
                };
                Ok(Box::new(Expression::Prefix(PrefixExpression::new(
                    token,
                    right_expresion,
                ))))
            }
            _ => Err(ParseError::Expression(ParseExpressionError::NoPrefixParse(
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
                return Err(ParseError::Expression(
                    ParseExpressionError::InfixExpression,
                ));
            }
        };

        Ok(Box::new(Expression::Infix(InfixExpression::new(
            token,
            left_expression,
            right_expresion,
        ))))
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

#[cfg(test)]
mod tests {
    use super::Parser;
    use crate::ast::*;
    use crate::lexer::Lexer;

    fn setup_let_statement_input() -> String {
        "
        let x = 5;
        let y = 10;
        let foobar = 838383;
        "
        .to_string()
    }

    fn setup_let_statement_expects() -> Vec<String> {
        vec!["x", "y", "foobar"]
            .into_iter()
            .map(str::to_string)
            .collect()
    }

    #[test]
    fn test_let_statement() {
        let input = setup_let_statement_input();
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        check_parser_errors(&parser);

        assert_eq!(
            3,
            program.len(),
            "program does not contains 3 statements. got={}",
            program.len()
        );

        setup_let_statement_expects()
            .into_iter()
            .zip(program.iter())
            .for_each(|(expect, statement)| {
                assert_let_statement(statement, expect);
            });
    }

    fn assert_let_statement(statement: &Statement, expect_name: String) {
        match statement {
            Statement::Let(ref statement) => {
                assert_eq!(
                    "let".to_string(),
                    statement.token_literal(),
                    "statement.token_literal not 'let'. got={}",
                    statement.token_literal()
                );
                assert_eq!(
                    expect_name, statement.name.value,
                    "let_statement.name.value not '{}'. got={}",
                    expect_name, statement.name.value
                );
                assert_eq!(
                    expect_name,
                    statement.name.token_literal(),
                    "let_statement.name.token_literal not '{}'. got={}",
                    expect_name,
                    statement.name.token_literal()
                );
            }
            other_statement => panic!(format!(
                "statement not ReturnStatement. got={}",
                other_statement
            )),
        };
    }

    fn setup_return_statement_input() -> String {
        "
        return 5;
        return 10;
        return 993322;
        "
        .to_string()
    }

    #[test]
    fn test_return_statement() {
        let input = setup_return_statement_input();
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        check_parser_errors(&parser);

        assert_eq!(
            3,
            program.len(),
            "program does not contain 3 statements. got={}",
            program.len()
        );

        program.iter().for_each(|statement| match statement {
            Statement::Return(statement) => assert_eq!(
                "return",
                statement.token_literal(),
                "statement.token_literal not 'return'. got={}",
                statement.token_literal()
            ),
            other_statement => panic!(format!(
                "statement not ReturnStatement. got={}",
                other_statement
            )),
        })
    }

    #[test]
    fn test_identifier_expression() {
        let input = "foobar;".to_string();
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        check_parser_errors(&parser);

        assert_eq!(
            1,
            program.len(),
            "program has not enough etatements. got={}",
            program.len()
        );

        if let Statement::Expression(statement) = program.get(0).unwrap() {
            let expression: &Expression = statement.expression.as_ref();
            match expression {
                Expression::Identifier(identifier) => {
                    assert_eq!(
                        "foobar", identifier.value,
                        "identifier value not {}. got={}",
                        "foobar", identifier.value
                    );
                    assert_eq!(
                        "foobar",
                        identifier.token_literal(),
                        "identifier token_literal() not {}. got={}",
                        "foobar",
                        identifier.token_literal()
                    );
                }
                _ => {
                    panic!("expression not Identifier. got={}", expression);
                }
            }
        } else {
            panic!(
                "program statement is not ExpressionStatement. got={}",
                program.get(0).unwrap()
            );
        }
    }

    #[test]
    fn test_integer_literal_expression() {
        let input = "5;".to_string();
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        check_parser_errors(&parser);

        assert_eq!(
            1,
            program.len(),
            "program has not enough statements. got={}",
            program.len()
        );

        if let Statement::Expression(statement) = program.get(0).unwrap() {
            let expression: &Expression = statement.expression.as_ref();
            match expression {
                Expression::Integer(literal) => {
                    assert_eq!(
                        5, literal.value,
                        "literal value not 5. got={}",
                        literal.value
                    );
                    assert_eq!(
                        "5".to_string(),
                        literal.token_literal(),
                        "literal token_literal not '5'. got={}",
                        literal.token_literal()
                    );
                }
                _ => panic!("expression not IntegerLiteral. got={}", expression),
            }
        } else {
            panic!(
                "program statements is not ExpressionStatement. got={}",
                program.get(0).unwrap()
            );
        }
    }

    fn setup_parsing_prefix_expression_input() -> Vec<String> {
        vec!["!5", "-15"].into_iter().map(String::from).collect()
    }

    fn setup_parsing_prefix_expression_expect() -> Vec<(String, i64)> {
        vec![("!", 5), ("-", 15)]
            .into_iter()
            .map(|(prefix, value)| (prefix.to_string(), value))
            .collect()
    }

    #[test]
    fn test_parsing_prefix_expression() {
        let inputs = setup_parsing_prefix_expression_input();
        let expects = setup_parsing_prefix_expression_expect();
        inputs
            .into_iter()
            .zip(expects.into_iter())
            .for_each(|(input, expect)| {
                let lexer = Lexer::new(input);
                let mut parser = Parser::new(lexer);
                let program = parser.parse_program();
                check_parser_errors(&parser);

                assert_eq!(
                    1,
                    program.len(),
                    "program statements does not contain 1 statements. got={}",
                    program.len()
                );

                if let Statement::Expression(statement) = program.get(0).unwrap() {
                    let expression: &Expression = statement.expression.as_ref();
                    match expression {
                        Expression::Prefix(prefix) => {
                            assert_eq!(
                                expect.0, prefix.operator,
                                "expression operator is not {}. got={}",
                                expect.0, prefix.operator
                            );
                            assert_integer_literal(&prefix.right, expect.1);
                        }
                        _ => panic!("expression is not PrefixExpression. got={}", expression),
                    }
                } else {
                    panic!(
                        "program statements is not ExpressionStatement. got={}",
                        program.get(0).unwrap()
                    );
                }
            });
    }

    fn setup_parsing_infix_expression_input() -> Vec<String> {
        vec![
            "5 + 5;", "5 - 5;", "5 * 5;", "5 / 5;", "5 > 5;", "5 < 5;", "5 == 5;", "5 != 5;",
        ]
        .into_iter()
        .map(String::from)
        .collect()
    }

    fn setup_parsing_infix_expression_expect() -> Vec<(i64, String, i64)> {
        vec![
            (5, "+", 5),
            (5, "-", 5),
            (5, "*", 5),
            (5, "/", 5),
            (5, ">", 5),
            (5, "<", 5),
            (5, "==", 5),
            (5, "!=", 5),
        ]
        .into_iter()
        .map(|(left, op, right)| (left, op.to_string(), right))
        .collect()
    }

    #[test]
    fn test_parsing_infix_expression() {
        let inputs = setup_parsing_infix_expression_input();
        let expects = setup_parsing_infix_expression_expect();

        inputs
            .into_iter()
            .zip(expects.into_iter())
            .for_each(|(input, expect)| {
                let mut parser = Parser::new(Lexer::new(input));
                let program = parser.parse_program();

                assert_eq!(
                    1,
                    program.len(),
                    "program statements does not contain 1 statements. got={}",
                    program.len()
                );
                let statement = match program.get(0).unwrap() {
                    Statement::Expression(statement) => statement,
                    _ => panic!("program statement is not ExpressionStatement."),
                };
                assert_infix_expression(
                    &statement.expression,
                    Wrapped::int(expect.0),
                    expect.1,
                    Wrapped::int(expect.2),
                );
            });
    }

    fn assert_integer_literal(expression: &Expression, value: i64) {
        match expression {
            Expression::Integer(literal) => {
                assert_eq!(
                    value, literal.value,
                    "literal value not {}. got={}",
                    value, literal.value
                );
                assert_eq!(
                    value.to_string(),
                    literal.token_literal(),
                    "literal token_literal not {}. got={}",
                    value.to_string(),
                    literal.token_literal()
                );
            }
            _ => panic!("expression not IntegerLiteral. got={}", expression),
        }
    }

    fn assert_identifier(expression: &Expression, value: String) {
        match expression {
            Expression::Identifier(identifier) => {
                assert_eq!(
                    identifier.value, value,
                    "identifier.value not {}. got={}",
                    value, identifier.value
                );
                assert_eq!(
                    identifier.token_literal(),
                    value,
                    "identifier.token_literal() not {}. got={}",
                    value,
                    identifier.token_literal()
                );
            }
            _ => panic!("expression not Identifier. got={}", expression),
        }
    }

    fn assert_literal_expression(expression: &Expression, expected: Wrapped) {
        match expected {
            Wrapped::int(v) => assert_integer_literal(expression, v),
            Wrapped::string(v) => assert_identifier(expression, v),
        };
    }

    fn assert_infix_expression(
        expression: &Expression,
        left: Wrapped,
        operator: String,
        right: Wrapped,
    ) {
        match expression {
            Expression::Infix(expression) => {
                assert_literal_expression(&expression.left, left);
                assert_eq!(
                    expression.operator, operator,
                    "expression.operator is not {}. got={}",
                    operator, expression.operator
                );
                assert_literal_expression(&expression.right, right);
            }
            _ => panic!("expression is not InfixExpression. got={}", expression),
        }
    }

    fn setup_operator_precedence_parsing_input() -> Vec<String> {
        vec![
            "-a * b",
            "!-a",
            "a + b + c",
            "a + b - c",
            "a * b * c",
            "a * b / c",
            "a + b / c",
            "a + b * c + d / e - f",
            "3 + 4; -5 * 5",
            "5 > 4 == 3 < 4",
            "5 < 4 != 3 > 4",
            "3 + 4 * 5 == 3 * 1 + 4 * 5",
        ]
        .into_iter()
        .map(String::from)
        .collect()
    }

    fn setup_operator_precedence_parsing_expect() -> Vec<String> {
        vec![
            "((-a) * b)",
            "(!(-a))",
            "((a + b) + c)",
            "((a + b) - c)",
            "((a * b) * c)",
            "((a * b) / c)",
            "(a + (b / c))",
            "(((a + (b * c)) + (d / e)) - f)",
            "(3 + 4)((-5) * 5)",
            "((5 > 4) == (3 < 4))",
            "((5 < 4) != (3 > 4))",
            "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
        ]
        .into_iter()
        .map(String::from)
        .collect()
    }

    #[test]
    fn test_operator_precedence_parsing() {
        let inputs = setup_operator_precedence_parsing_input();
        let expects = setup_operator_precedence_parsing_expect();
        inputs
            .into_iter()
            .zip(expects.into_iter())
            .for_each(|(input, expect)| {
                let mut parser = Parser::new(Lexer::new(input));
                let program = parser.parse_program();
                check_parser_errors(&parser);
                assert!(program.first().is_some(), "first is none");
                assert_eq!(
                    expect,
                    string(&program),
                    "expected={}, got={}",
                    expect,
                    string(&program)
                );
            })
    }

    fn check_parser_errors(parser: &Parser) {
        if parser.errors.is_empty() {
            return;
        }

        eprintln!("parser has {} errors", parser.errors.len());
        parser
            .errors
            .iter()
            .for_each(|err| eprintln!("parser error: {}", err));
    }

    enum Wrapped {
        int(i64),
        string(String),
    }
}
