use crate::token::{Token, TokenType};
use std::rc::Rc;

#[derive(Default)]
struct Lexer {
    input: Rc<Vec<char>>,
    position: usize,
    read_position: usize,
    examining_char: Option<char>,
}

impl Lexer {
    fn new(input: String) -> Self {
        let mut lexer = Self {
            input: Rc::new(input.chars().collect()),
            ..Default::default()
        };
        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.examining_char = None;
        } else {
            self.examining_char = self.input.get(self.read_position).cloned();
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn next_token(&mut self) -> Token {
        dbg!(&self.examining_char);
        let token = match self.examining_char {
            Some('=') => Token::new(
                TokenType::Assign,
                self.input[self.position..self.read_position]
                    .iter()
                    .collect::<String>(),
            ),
            Some(';') => Token::new(
                TokenType::Semicolon,
                self.input[self.position..self.read_position]
                    .iter()
                    .collect::<String>(),
            ),
            Some('(') => Token::new(
                TokenType::LParen,
                self.input[self.position..self.read_position]
                    .iter()
                    .collect::<String>(),
            ),
            Some(')') => Token::new(
                TokenType::RParen,
                self.input[self.position..self.read_position]
                    .iter()
                    .collect::<String>(),
            ),
            Some(',') => Token::new(
                TokenType::Comma,
                self.input[self.position..self.read_position]
                    .iter()
                    .collect::<String>(),
            ),
            Some('+') => Token::new(
                TokenType::Plus,
                self.input[self.position..self.read_position]
                    .iter()
                    .collect::<String>(),
            ),
            Some('{') => Token::new(
                TokenType::LBrace,
                self.input[self.position..self.read_position]
                    .iter()
                    .collect::<String>(),
            ),
            Some('}') => Token::new(
                TokenType::RBrace,
                self.input[self.position..self.read_position]
                    .iter()
                    .collect::<String>(),
            ),
            _ => Token::new(TokenType::EOF, "".to_string()),
        };

        self.read_char();
        dbg!(&token);
        token
    }
}

#[cfg(test)]
mod tests {
    use crate::token::*;

    struct Expect {
        token_type: TokenType,
        literal: &'static str,
    }

    fn setup_expects() -> Vec<Expect> {
        vec![
            Expect {
                token_type: TokenType::Assign,
                literal: "=",
            },
            Expect {
                token_type: TokenType::Plus,
                literal: "+",
            },
            Expect {
                token_type: TokenType::LParen,
                literal: "(",
            },
            Expect {
                token_type: TokenType::RParen,
                literal: ")",
            },
            Expect {
                token_type: TokenType::LBrace,
                literal: "{",
            },
            Expect {
                token_type: TokenType::RBrace,
                literal: "}",
            },
            Expect {
                token_type: TokenType::Comma,
                literal: ",",
            },
            Expect {
                token_type: TokenType::Semicolon,
                literal: ";",
            },
            Expect {
                token_type: TokenType::EOF,
                literal: "",
            },
        ]
    }

    #[test]
    fn test_next_token() {
        let input = "=+(){},;".to_string();
        let expects = setup_expects();

        let mut lexer = super::Lexer::new(input);

        for expect in expects.iter() {
            let token = lexer.next_token();
            assert_eq!(expect.token_type, token.token_type);
            assert_eq!(expect.literal, token.literal);
        }
    }
}
