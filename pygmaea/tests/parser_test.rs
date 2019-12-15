#[cfg(test)]
mod tests {
    use pygmaea::ast::*;
    use pygmaea::lexer::Lexer;
    use pygmaea::parser::Parser;

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
        let let_statement = match statement {
            Statement::Let(ref statement) => statement,
            other_statement => panic!(
                "[{}] statement not ReturnStatement. got={}",
                i, other_statement
            ),
        };
        assert_eq!(
            "let".to_string(),
            let_statement.token_literal(),
            "[{}] statement.token_literal not 'let'. got={}",
            i,
            let_statement.token_literal()
        );
        assert_eq!(
            expect_name, let_statement.identifier.value,
            "[{}] let_statement.name.value not '{}'. got={}",
            i, expect_name, let_statement.identifier.value
        );
        assert_eq!(
            expect_name,
            let_statement.identifier.token_literal(),
            "[{}] let_statement.name.token_literal not '{}'. got={}",
            i,
            expect_name,
            let_statement.identifier.token_literal()
        );
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

            program.iter().for_each(|statement| {
                let return_statement = match statement {
                    Statement::Return(statement) => statement,
                    other_statement => panic!(
                        "[{}] statement not ReturnStatement. got={}",
                        i, other_statement
                    ),
                };
                assert_eq!(
                    "return",
                    return_statement.token_literal(),
                    "[{}] statement.token_literal not 'return'. got={}",
                    i,
                    return_statement.token_literal()
                );
            });
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

            let expression_statement = match program.get(0) {
                Some(Statement::Expression(statement)) => statement,
                _ => panic!(
                    "[{}] program statement is not ExpressionStatement. got={}",
                    i,
                    program.get(0).unwrap()
                ),
            };

            let identifier = match *expression_statement.expression {
                Expression::Identifier(ref identifier) => identifier,
                _ => panic!(
                    "[{}] expression not Identifier. got={}",
                    i, expression_statement
                ),
            };

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

            let expression_statement = match program.get(0) {
                Some(Statement::Expression(statement)) => statement,
                _ => panic!(
                    "[{}] program statements is not ExpressionStatement. got={}",
                    i,
                    program.get(0).unwrap()
                ),
            };

            let integer_literal = match *expression_statement.expression {
                Expression::Integer(ref literal) => literal,
                _ => panic!(
                    "[{}] expression not IntegerLiteral. got={}",
                    i, expression_statement.expression
                ),
            };
            assert_eq!(
                5, integer_literal.value,
                "[{}] literal value not 5. got={}",
                i, integer_literal.value
            );
            assert_eq!(
                "5".to_string(),
                integer_literal.token_literal(),
                "[{}] literal token_literal not '5'. got={}",
                i,
                integer_literal.token_literal()
            );
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

                let expression_statement = match program.get(0) {
                    Some(Statement::Expression(statement)) => statement,
                    _ => panic!(
                        "[{}] program statements is not ExpressionStatement. got={}",
                        i,
                        program.get(0).unwrap()
                    ),
                };

                let prefix_expression = match *expression_statement.expression {
                    Expression::Prefix(ref expression) => expression,
                    _ => panic!(
                        "[{}] expression is not PrefixExpression. got={}",
                        i, expression_statement.expression
                    ),
                };
                assert_eq!(
                    expect.0, prefix_expression.operator,
                    "[{}] expression operator is not {}. got={}",
                    i, expect.0, prefix_expression.operator
                );
                expect
                    .1
                    .assert_literal_expression(&prefix_expression.right, i);
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

                let expression_statement = match program.get(0) {
                    Some(Statement::Expression(statement)) => statement,
                    _ => panic!("[{}] program statement is not ExpressionStatement.", i),
                };

                assert_infix_expression(
                    &expression_statement.expression,
                    expect.0,
                    expect.1,
                    expect.2,
                    i,
                );
            });
    }

    fn assert_infix_expression(
        expression: &Expression,
        left: impl Wrapped,
        operator: String,
        right: impl Wrapped,
        i: usize,
    ) {
        let infix_expression = match expression {
            Expression::Infix(expression) => expression,
            _ => panic!(
                "[{}] expression is not InfixExpression. got={}",
                i, expression
            ),
        };

        left.assert_literal_expression(&infix_expression.left, i);
        assert_eq!(
            infix_expression.operator, operator,
            "[{}] expression.operator is not {}. got={}",
            i, operator, infix_expression.operator
        );
        right.assert_literal_expression(&infix_expression.right, i);
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
                        assert_boolean_expression(&statement.expression, expect, i)
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
        let boolean_expression = match expression {
            Expression::Boolean(expression) => expression,
            _ => panic!("[{}] expression is not Boolean. got={}", i, expression),
        };
        assert_eq!(
            expect, boolean_expression.value,
            "[{}] boolean value not {}. got={}",
            i, expect, boolean_expression.value
        );
        assert_eq!(
            expect.to_string(),
            boolean_expression.token_literal(),
            "[{}] literal token_literal not {}. got={}",
            i,
            expect.to_string(),
            boolean_expression.token_literal()
        );
    }

    fn assert_integer_literal(expression: &Expression, value: i64, i: usize) {
        let integer_literal = match expression {
            Expression::Integer(literal) => literal,
            _ => panic!("[{}] expression not IntegerLiteral. got={}", i, expression),
        };

        assert_eq!(
            value, integer_literal.value,
            "[{}] literal value not {}. got={}",
            i, value, integer_literal.value
        );
        assert_eq!(
            value.to_string(),
            integer_literal.token_literal(),
            "[{}] literal token_literal not {}. got={}",
            i,
            value.to_string(),
            integer_literal.token_literal()
        );
    }

    fn assert_identifier(expression: &Expression, value: String, i: usize) {
        let identifier = match expression {
            Expression::Identifier(identifier) => identifier,
            _ => panic!("[{}] expression not Identifier. got={}", i, expression),
        };
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

    fn assert_boolean_literal(expression: &Expression, value: bool, i: usize) {
        let boolean = match expression {
            Expression::Boolean(boolean) => boolean,
            _ => panic!("[{}] expression not boolean. got={}", i, expression),
        };
        assert_eq!(
            value, boolean.value,
            "[{}] expression.value not {}. got={}",
            i, value, boolean.value
        );
        assert_eq!(
            value.to_string(),
            boolean.token_literal(),
            "[{}] expression.token_literal() not {}. got={}",
            i,
            value.to_string(),
            boolean.token_literal()
        );
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
