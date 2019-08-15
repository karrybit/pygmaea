use crate::token::{Token, TokenType};

#[derive(Default)]
struct Lexer {
    input: Vec<char>,
    position: usize,
    read_position: usize,
    examining_char: Option<char>,
}

impl Lexer {
    fn new(input: String) -> Self {
        let mut lexer = Self {
            input: input.chars().collect(),
            ..Default::default()
        };
        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        self.examining_char = self.input.get(self.read_position).cloned();
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn next_token(&mut self) -> Token {
        let extract_literal = || {
            self.input[self.position..self.read_position]
                .iter()
                .collect::<String>()
        };

        let token = match self.examining_char {
            Some('=') => Token::new(TokenType::Assign, extract_literal()),
            Some(';') => Token::new(TokenType::Semicolon, extract_literal()),
            Some('(') => Token::new(TokenType::LParen, extract_literal()),
            Some(')') => Token::new(TokenType::RParen, extract_literal()),
            Some(',') => Token::new(TokenType::Comma, extract_literal()),
            Some('+') => Token::new(TokenType::Plus, extract_literal()),
            Some('{') => Token::new(TokenType::LBrace, extract_literal()),
            Some('}') => Token::new(TokenType::RBrace, extract_literal()),
            None => Token::new(TokenType::EOF, "".to_string()),
            Some(c) => panic!("'{}' is invalid charactor", c),
        };

        self.read_char();
        token
    }
}

#[cfg(test)]
mod tests {
    use crate::token::*;

    fn setup_input() -> String {
        "let five = 5;
        let ten = 10;

        let add = fn(x, y){
            x + y;
        };

        let result = add(five, ten);
        "
        .to_string()
    }

    struct Expect(TokenType, &'static str);
    impl Expect {
        fn token_type(&self) -> &TokenType {
            &self.0
        }
        fn literal(&self) -> &'static str {
            self.1
        }
    }

    fn setup_expects() -> Vec<Expect> {
        vec![
            Expect(TokenType::Let, "let"),
            Expect(TokenType::Ident, "five"),
            Expect(TokenType::Assign, "="),
            Expect(TokenType::Int, "5"),
            Expect(TokenType::Semicolon, ";"),
            Expect(TokenType::Let, "let"),
            Expect(TokenType::Ident, "ten"),
            Expect(TokenType::Assign, "="),
            Expect(TokenType::Int, "10"),
            Expect(TokenType::Semicolon, ";"),
            Expect(TokenType::Let, "let"),
            Expect(TokenType::Ident, "add"),
            Expect(TokenType::Assign, "="),
            Expect(TokenType::Function, "fn"),
            Expect(TokenType::LParen, "("),
            Expect(TokenType::Ident, "x"),
            Expect(TokenType::Comma, ","),
            Expect(TokenType::Ident, "y"),
            Expect(TokenType::RParen, ")"),
            Expect(TokenType::LBrace, "{"),
            Expect(TokenType::Ident, "x"),
            Expect(TokenType::Plus, "+"),
            Expect(TokenType::Ident, "y"),
            Expect(TokenType::Semicolon, ";"),
            Expect(TokenType::RBrace, "}"),
            Expect(TokenType::Semicolon, ";"),
            Expect(TokenType::Let, "let"),
            Expect(TokenType::Ident, "result"),
            Expect(TokenType::Assign, "="),
            Expect(TokenType::Ident, "add"),
            Expect(TokenType::LParen, "("),
            Expect(TokenType::Ident, "five"),
            Expect(TokenType::Comma, ","),
            Expect(TokenType::Ident, "ten"),
            Expect(TokenType::RParen, ")"),
            Expect(TokenType::Semicolon, ";"),
            Expect(TokenType::EOF, ""),
        ]
    }

    fn exact_expect(input: &str, expects: &[Expect]) -> bool {
        let mut expect = String::new();
        expects.iter().for_each(|e| expect.push_str(e.literal()));
        input
            .trim()
            .chars()
            .filter(|c| c != &' ' && c != &'\n')
            .collect::<String>()
            == expect
    }

    #[test]
    fn test_next_token() {
        let input = setup_input();
        let expects = setup_expects();
        assert!(exact_expect(&input, &expects));

        let mut lexer = super::Lexer::new(input);

        for expect in expects.iter() {
            let token = lexer.next_token();
            assert_eq!(expect.token_type(), &token.token_type);
            assert_eq!(expect.literal(), token.literal);
        }
    }
}
