use crate::ast::{Identifier, LetStatement, Program, StatementKind};
use crate::lexer::Lexer;
use crate::token::Token;
use crate::token_type::TokenType;

struct Parser {
    lexer: Lexer,
    current_token: Option<Box<Token>>,
    peek_token: Option<Box<Token>>,
}

impl Parser {
    fn new(lexer: Lexer) -> Self {
        let mut parser = Self {
            lexer,
            current_token: None,
            peek_token: None,
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
            let statement = self.parse_statement();
            if let Some(statement) = statement {
                program.push(statement);
            }
            self.next_token();
        }
        program
    }

    fn parse_statement(&mut self) -> Option<StatementKind> {
        match self.current_token {
            Some(ref token) if token.token_type == TokenType::Let => self.parse_let_statement(),
            _ => None,
        }
    }

    fn parse_let_statement(&mut self) -> Option<StatementKind> {
        if !self.peek_token_is(TokenType::Ident) {
            return None;
        }

        let let_token = self.current_token.as_ref().unwrap().as_ref().clone();
        let identifier = Identifier::new(self.peek_token.as_ref().unwrap().clone());
        self.next_token();

        match self.peek_token {
            Some(ref token) if token.token_type == TokenType::Assign => {
                while !self.current_token_is(TokenType::Semicolon) {
                    self.next_token();
                }
                Some(StatementKind::LetStatement(LetStatement::new(
                    let_token, identifier, None,
                )))
            }
            _ => None,
        }
    }

    fn current_token_is(&self, token_type: TokenType) -> bool {
        match self.current_token {
            Some(ref token) if token.token_type == token_type => true,
            _ => false,
        }
    }

    fn peek_token_is(&self, token_type: TokenType) -> bool {
        match self.peek_token {
            Some(ref token) if token.token_type == token_type => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Parser;
    use crate::ast::*;
    use crate::lexer::Lexer;

    fn setup_input() -> String {
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
        let input = setup_input();
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        assert_eq!(
            program.len(),
            3,
            "program.statements does not contains 3 statements. got={}",
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

    fn assert_let_statement(statement: &StatementKind, expect_name: String) {
        match statement {
            StatementKind::LetStatement(ref statement) => {
                assert_eq!(
                    statement.token_literal(),
                    "let".to_string(),
                    "statement.token_literal not 'let'. got={}",
                    statement.token_literal()
                );
                assert_eq!(
                    statement.name.value, expect_name,
                    "let_statement.name.value not '{}'. got={}",
                    expect_name, statement.name.value
                );
                assert_eq!(
                    statement.name.token_literal(),
                    expect_name,
                    "let_statement.name.token_literal not '{}'. got={}",
                    expect_name,
                    statement.name.token_literal()
                );
            }
        };
    }
}
