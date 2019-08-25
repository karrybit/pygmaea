use crate::ast::*;
use crate::error::ParseError;
use crate::lexer::Lexer;
use crate::token::Token;
use crate::token_type::TokenType;

enum PriorityType {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
}

struct Parser {
    lexer: Lexer,
    current_token: Box<Token>,
    peek_token: Box<Token>,
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
        self.current_token = std::mem::replace(&mut self.peek_token, Default::default());
        self.peek_token = Box::new(self.lexer.next_token());
    }

    fn parse_program(&mut self) -> Program {
        let mut program = Program::new();
        while !self.current_token.token_type.is_eof() {
            if let Some(statement) = self.parse_statement() {
                program.push(statement);
            }
            self.next_token();
        }
        program
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.current_token.token_type {
            TokenType::Let => self.parse_let_statement(),
            TokenType::Return => Some(self.parse_return_statement()),
            _ => Some(self.parse_expression_statement()),
        }
    }

    fn parse_let_statement(&mut self) -> Option<Statement> {
        if !self.peek_token_is(TokenType::Ident) {
            self.peek_error(TokenType::Ident);
            return None;
        }

        let let_token = self.current_token.clone();
        let identifier = Identifier::new(self.peek_token.clone());
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

    fn parse_return_statement(&mut self) -> Statement {
        let statement = ReturnStatement::new(self.current_token.clone());
        self.next_token();
        while !self.current_token_is(TokenType::Semicolon) {
            self.next_token();
        }
        Statement::Return(statement)
    }

    fn parse_expression_statement(&mut self) -> Statement {
        let mut statement = ExpressionStatement::new(self.current_token.clone());
        statement.expression = self.parse_expression(PriorityType::Lowest);
        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }
        Statement::Expression(statement)
    }

    fn parse_expression(&self, priority_type: PriorityType) -> Option<Box<Expression>> {
        match self.current_token.token_type {
            TokenType::Ident => Some(Box::new(Expression::Identifier(Identifier::new(
                self.current_token.clone(),
            )))),
            _ => None,
        }
    }

    fn current_token_is(&self, token_type: TokenType) -> bool {
        self.current_token.token_type == token_type
    }

    fn peek_token_is(&mut self, token_type: TokenType) -> bool {
        self.peek_token.token_type == token_type
    }

    fn peek_error(&mut self, token_type: TokenType) {
        self.errors.push(ParseError::PeekTokenError {
            msg: format!(
                "expected next token to be {}, got {} instead",
                token_type, self.peek_token.token_type
            ),
        })
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

    fn setup_expects() -> Vec<String> {
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

        setup_expects()
            .into_iter()
            .enumerate()
            .for_each(|(i, expect)| {
                let statement = program.get(i);
                assert!(statement.is_some());
                assert_let_statement(statement.unwrap(), expect);
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
            let expression: &Expression = statement.expression.as_ref().unwrap();
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

    fn check_parser_errors(parser: &Parser) {
        if parser.errors.is_empty() {
            return;
        }

        eprintln!("parser has {} errors", parser.errors.len());
        parser
            .errors
            .iter()
            .for_each(|err| eprintln!("parser error: {}", err));

        panic!();
    }
}
