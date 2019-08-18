use crate::ast::Program;
use crate::lexer::Lexer;
use crate::token::Token;

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

    // TODO:
    fn parser_program(&self) -> Program {
        unimplemented!()
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
        let parser = Parser::new(lexer);
        let program = parser.parser_program();

        assert_eq!(
            program.statements.len(),
            3,
            "program.statements does not contains 3 statements. got={}",
            program.statements.len()
        );

        setup_expects()
            .into_iter()
            .enumerate()
            .for_each(|(i, expect)| {
                let statement = program.statements.get(i);
                assert!(statement.is_some());
                assert_let_statement(statement.unwrap(), expect);
            });
    }

    fn assert_let_statement(statement: &StatementType, expect_name: String) {
        match statement {
            StatementType::Let(ref let_statement) => {
                assert_eq!(
                    let_statement.token_literal(),
                    "let".to_string(),
                    "statement.token_literal not 'let'. got={}",
                    let_statement.token_literal()
                );
                assert_eq!(
                    let_statement.name.value, expect_name,
                    "let_statement.name.value not '{}'. got={}",
                    expect_name, let_statement.name.value
                );
                assert_eq!(
                    let_statement.name.token_literal(),
                    expect_name,
                    "let_statement.name.token_literal not '{}'. got={}",
                    expect_name,
                    let_statement.name.token_literal()
                );
            }
        };
    }
}
