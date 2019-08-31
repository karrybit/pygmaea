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

#[cfg(test)]
mod tests {
    use super::Parser;
    use crate::ast::*;
    use crate::lexer::Lexer;

    fn setup_let_statement_input() -> Vec<String> {
        vec![
            "
        let x = 5;",
            "let y = 10;",
            "let foobar = 838383;
        ",
        ]
        .into_iter()
        .map(str::to_string)
        .collect()
    }

    fn setup_let_statement_expects() -> Vec<String> {
        vec!["x", "y", "foobar"]
            .into_iter()
            .map(str::to_string)
            .collect()
    }

    #[test]
    fn test_let_statement() {
        let inputs = setup_let_statement_input();
        let expects = setup_let_statement_expects();
        inputs
            .into_iter()
            .zip(expects.into_iter())
            .enumerate()
            .for_each(|(i, (input, expect))| {
                let lexer = Lexer::new(input);
                let mut parser = Parser::new(lexer);
                let program = parser.parse_program();
                check_parser_errors(&parser, i);

                assert_eq!(
                    1,
                    program.len(),
                    "[{}] program does not contains 1 statements. got={}",
                    i,
                    program.len()
                );
                assert_let_statement(program.get(0).unwrap(), expect, i);
            });
    }

    fn assert_let_statement(statement: &Statement, expect_name: String, i: usize) {
        match statement {
            Statement::Let(ref statement) => {
                assert_eq!(
                    "let".to_string(),
                    statement.token_literal(),
                    "[{}] statement.token_literal not 'let'. got={}",
                    i,
                    statement.token_literal()
                );
                assert_eq!(
                    expect_name, statement.name.value,
                    "[{}] let_statement.name.value not '{}'. got={}",
                    i, expect_name, statement.name.value
                );
                assert_eq!(
                    expect_name,
                    statement.name.token_literal(),
                    "[{}] let_statement.name.token_literal not '{}'. got={}",
                    i,
                    expect_name,
                    statement.name.token_literal()
                );
            }
            other_statement => panic!(
                "[{}] statement not ReturnStatement. got={}",
                i, other_statement
            ),
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
        let inputs = vec![setup_return_statement_input()];
        inputs.into_iter().enumerate().for_each(|(i, input)| {
            let lexer = Lexer::new(input);
            let mut parser = Parser::new(lexer);

            let program = parser.parse_program();
            check_parser_errors(&parser, i);

            assert_eq!(
                3,
                program.len(),
                "[{}] program does not contain 3 statements. got={}",
                i,
                program.len()
            );

            program.iter().for_each(|statement| match statement {
                Statement::Return(statement) => assert_eq!(
                    "return",
                    statement.token_literal(),
                    "[{}] statement.token_literal not 'return'. got={}",
                    i,
                    statement.token_literal()
                ),
                other_statement => panic!(
                    "[{}] statement not ReturnStatement. got={}",
                    i, other_statement
                ),
            })
        });
    }

    #[test]
    fn test_identifier_expression() {
        let inputs = vec!["foobar;".to_string()];
        inputs.into_iter().enumerate().for_each(|(i, input)| {
            let lexer = Lexer::new(input);
            let mut parser = Parser::new(lexer);
            let program = parser.parse_program();

            check_parser_errors(&parser, i);

            assert_eq!(
                1,
                program.len(),
                "[{}] program has not enough etatements. got={}",
                i,
                program.len()
            );

            if let Statement::Expression(statement) = program.get(0).unwrap() {
                let expression: &Expression = statement.expression.as_ref();
                match expression {
                    Expression::Identifier(identifier) => {
                        assert_eq!(
                            "foobar", identifier.value,
                            "[{}] identifier value not {}. got={}",
                            i, "foobar", identifier.value
                        );
                        assert_eq!(
                            "foobar",
                            identifier.token_literal(),
                            "[{}] identifier token_literal() not {}. got={}",
                            i,
                            "foobar",
                            identifier.token_literal()
                        );
                    }
                    _ => {
                        panic!("[{}] expression not Identifier. got={}", i, expression);
                    }
                }
            } else {
                panic!(
                    "[{}] program statement is not ExpressionStatement. got={}",
                    i,
                    program.get(0).unwrap()
                );
            }
        });
    }

    #[test]
    fn test_integer_literal_expression() {
        let inputs = vec!["5;".to_string()];
        inputs.into_iter().enumerate().for_each(|(i, input)| {
            let lexer = Lexer::new(input);
            let mut parser = Parser::new(lexer);
            let program = parser.parse_program();
            check_parser_errors(&parser, i);

            assert_eq!(
                1,
                program.len(),
                "[{}] program has not enough statements. got={}",
                i,
                program.len()
            );

            if let Statement::Expression(statement) = program.get(0).unwrap() {
                let expression: &Expression = statement.expression.as_ref();
                match expression {
                    Expression::Integer(literal) => {
                        assert_eq!(
                            5, literal.value,
                            "[{}] literal value not 5. got={}",
                            i, literal.value
                        );
                        assert_eq!(
                            "5".to_string(),
                            literal.token_literal(),
                            "[{}] literal token_literal not '5'. got={}",
                            i,
                            literal.token_literal()
                        );
                    }
                    _ => panic!("[{}] expression not IntegerLiteral. got={}", i, expression),
                }
            } else {
                panic!(
                    "[{}] program statements is not ExpressionStatement. got={}",
                    i,
                    program.get(0).unwrap()
                );
            }
        });
    }

    fn setup_parsing_prefix_expression_input() -> Vec<String> {
        vec!["!5", "-15", "!true", "!false"]
            .into_iter()
            .map(str::to_string)
            .collect()
    }

    fn setup_parsing_prefix_expression_expect() -> Vec<(String, Concrete)> {
        vec![
            ("!", Concrete::Integer(5)),
            ("-", Concrete::Integer(15)),
            ("!", Concrete::Boolean(true)),
            ("!", Concrete::Boolean(false)),
        ]
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
            .enumerate()
            .for_each(|(i, (input, expect))| {
                let lexer = Lexer::new(input);
                let mut parser = Parser::new(lexer);
                let program = parser.parse_program();
                check_parser_errors(&parser, i);

                assert_eq!(
                    1,
                    program.len(),
                    "[{}] program statements does not contain 1 statements. got={}",
                    i,
                    program.len()
                );

                if let Statement::Expression(statement) = program.get(0).unwrap() {
                    let expression: &Expression = statement.expression.as_ref();
                    match expression {
                        Expression::Prefix(prefix) => {
                            assert_eq!(
                                expect.0, prefix.operator,
                                "[{}] expression operator is not {}. got={}",
                                i, expect.0, prefix.operator
                            );
                            expect.1.assert_literal_expression(&prefix.right, i);
                        }
                        _ => panic!(
                            "[{}] expression is not PrefixExpression. got={}",
                            i, expression
                        ),
                    }
                } else {
                    panic!(
                        "[{}] program statements is not ExpressionStatement. got={}",
                        i,
                        program.get(0).unwrap()
                    );
                }
            });
    }

    fn setup_parsing_infix_expression_input() -> Vec<String> {
        vec![
            "5 + 5;",
            "5 - 5;",
            "5 * 5;",
            "5 / 5;",
            "5 > 5;",
            "5 < 5;",
            "5 == 5;",
            "5 != 5;",
            "true == true",
            "true != false",
            "false == false",
        ]
        .into_iter()
        .map(str::to_string)
        .collect()
    }

    fn setup_parsing_infix_expression_expect() -> Vec<(Concrete, String, Concrete)> {
        vec![
            (Concrete::Integer(5), "+", Concrete::Integer(5)),
            (Concrete::Integer(5), "-", Concrete::Integer(5)),
            (Concrete::Integer(5), "*", Concrete::Integer(5)),
            (Concrete::Integer(5), "/", Concrete::Integer(5)),
            (Concrete::Integer(5), ">", Concrete::Integer(5)),
            (Concrete::Integer(5), "<", Concrete::Integer(5)),
            (Concrete::Integer(5), "==", Concrete::Integer(5)),
            (Concrete::Integer(5), "!=", Concrete::Integer(5)),
            (Concrete::Boolean(true), "==", Concrete::Boolean(true)),
            (Concrete::Boolean(true), "!=", Concrete::Boolean(false)),
            (Concrete::Boolean(false), "==", Concrete::Boolean(false)),
        ]
        .into_iter()
        .map(|(left, op, right)| (left, op.to_string(), right))
        .collect()
    }

    #[test]
    fn test_parsing_infix_expression() {
        let inputs = setup_parsing_infix_expression_input();
        let expects = setup_parsing_infix_expression_expect();

        assert_eq!(
            inputs.len(),
            expects.len(),
            "inputs.len and expects.len is mismatch"
        );

        inputs
            .into_iter()
            .zip(expects.into_iter())
            .enumerate()
            .for_each(|(i, (input, expect))| {
                let mut parser = Parser::new(Lexer::new(input));
                let program = parser.parse_program();

                check_parser_errors(&parser, i);

                assert_eq!(
                    1,
                    program.len(),
                    "[{}] program statements does not contain 1 statements. got={}",
                    i,
                    program.len()
                );
                let statement = match program.get(0).unwrap() {
                    Statement::Expression(statement) => statement,
                    _ => panic!("[{}] program statement is not ExpressionStatement.", i),
                };
                assert_infix_expression(&statement.expression, expect.0, expect.1, expect.2, i);
            });
    }

    fn assert_infix_expression(
        expression: &Expression,
        left: impl Wrapped,
        operator: String,
        right: impl Wrapped,
        i: usize,
    ) {
        match expression {
            Expression::Infix(expression) => {
                left.assert_literal_expression(&expression.left, i);
                assert_eq!(
                    expression.operator, operator,
                    "[{}] expression.operator is not {}. got={}",
                    i, operator, expression.operator
                );
                right.assert_literal_expression(&expression.right, i);
            }
            _ => panic!(
                "[{}] expression is not InfixExpression. got={}",
                i, expression
            ),
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
            "true",
            "false",
            "3 > 5 == false",
            "3 < 5 == true",
        ]
        .into_iter()
        .map(str::to_string)
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
            "true",
            "false",
            "((3 > 5) == false)",
            "((3 < 5) == true)",
        ]
        .into_iter()
        .map(str::to_string)
        .collect()
    }

    #[test]
    fn test_operator_precedence_parsing() {
        let inputs = setup_operator_precedence_parsing_input();
        let expects = setup_operator_precedence_parsing_expect();
        assert_eq!(
            inputs.len(),
            expects.len(),
            "inputs.len and expects.len is mismatch"
        );
        inputs
            .into_iter()
            .zip(expects.into_iter())
            .enumerate()
            .for_each(|(i, (input, expect))| {
                let mut parser = Parser::new(Lexer::new(input));
                let program = parser.parse_program();
                check_parser_errors(&parser, i);
                assert!(program.first().is_some(), "[{}] first is none", i);
                assert_eq!(
                    expect,
                    string(&program),
                    "[{}] expected={}, got={}",
                    i,
                    expect,
                    string(&program)
                );
            })
    }

    fn setup_boolean_expression_input() -> Vec<String> {
        vec![
            "true;",
            "false;",
            "let foobar = true;",
            "let barfoo = false;",
        ]
        .into_iter()
        .map(str::to_string)
        .collect()
    }

    fn setup_boolean_expression_expect() -> Vec<bool> {
        vec![true, false, true, false]
    }

    #[test]
    fn test_boolean_expression() {
        let inputs = setup_boolean_expression_input();
        let expects = setup_boolean_expression_expect();

        inputs
            .into_iter()
            .zip(expects.into_iter())
            .enumerate()
            .for_each(|(i, (input, expect))| {
                let lexer = Lexer::new(input);
                let mut parser = Parser::new(lexer);
                let program = parser.parse_program();
                check_parser_errors(&parser, i);

                assert!(program.first().is_some(), "first is none");

                match program.get(0).unwrap() {
                    Statement::Let(statement) => {
                        assert_boolean_expression(&statement.value, expect, i)
                    }
                    Statement::Expression(statement) => {
                        assert_boolean_expression(&statement.expression, expect, i)
                    }
                    _ => panic!(
                        "[{}] statement is not LetStatement or ExpressionStatement. got={}",
                        i,
                        program.get(0).unwrap()
                    ),
                }
            });
    }

    fn assert_boolean_expression(expression: &Expression, expect: bool, i: usize) {
        match expression {
            Expression::Boolean(boolean) => {
                assert_eq!(
                    expect, boolean.value,
                    "[{}] boolean value not {}. got={}",
                    i, expect, boolean.value
                );
                assert_eq!(
                    expect.to_string(),
                    boolean.token_literal(),
                    "[{}] literal token_literal not {}. got={}",
                    i,
                    expect.to_string(),
                    boolean.token_literal()
                );
            }
            _ => panic!("[{}] expression is not Boolean. got={}", i, expression),
        }
    }

    fn assert_integer_literal(expression: &Expression, value: i64, i: usize) {
        match expression {
            Expression::Integer(literal) => {
                assert_eq!(
                    value, literal.value,
                    "[{}] literal value not {}. got={}",
                    i, value, literal.value
                );
                assert_eq!(
                    value.to_string(),
                    literal.token_literal(),
                    "[{}] literal token_literal not {}. got={}",
                    i,
                    value.to_string(),
                    literal.token_literal()
                );
            }
            _ => panic!("[{}] expression not IntegerLiteral. got={}", i, expression),
        }
    }

    fn assert_identifier(expression: &Expression, value: String, i: usize) {
        match expression {
            Expression::Identifier(identifier) => {
                assert_eq!(
                    identifier.value, value,
                    "[{}] identifier.value not {}. got={}",
                    i, value, identifier.value
                );
                assert_eq!(
                    identifier.token_literal(),
                    value,
                    "[{}] identifier.token_literal() not {}. got={}",
                    i,
                    value,
                    identifier.token_literal()
                );
            }
            _ => panic!("[{}] expression not Identifier. got={}", i, expression),
        }
    }

    fn assert_boolean_literal(expression: &Expression, value: bool, i: usize) {
        match expression {
            Expression::Boolean(expression) => {
                assert_eq!(
                    value, expression.value,
                    "[{}] expression.value not {}. got={}",
                    i, value, expression.value
                );
                assert_eq!(
                    value.to_string(),
                    expression.token_literal(),
                    "[{}] expression.token_literal() not {}. got={}",
                    i,
                    value.to_string(),
                    expression.token_literal()
                );
            }
            _ => panic!("[{}] expression not boolean. got={}", i, expression),
        }
    }

    fn check_parser_errors(parser: &Parser, i: usize) {
        if parser.errors.is_empty() {
            return;
        }

        eprintln!("[{}] parser has {} errors", i, parser.errors.len());
        parser
            .errors
            .iter()
            .enumerate()
            .for_each(|(i, err)| eprintln!("[{}] parser error: {}", i, err));
    }

    trait Wrapped {
        fn assert_literal_expression(self, expression: &Expression, i: usize);
    }

    enum Concrete {
        Integer(i64),
        String(String),
        Boolean(bool),
    }

    impl Wrapped for Concrete {
        fn assert_literal_expression(self, expression: &Expression, i: usize) {
            match self {
                Concrete::Integer(v) => assert_integer_literal(expression, v, i),
                Concrete::String(v) => assert_identifier(expression, v, i),
                Concrete::Boolean(v) => assert_boolean_literal(expression, v, i),
            }
        }
    }
}
