use crate::token::{Token, TokenType, KEYWORDS};

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

    fn skip_whitespace(&mut self) {
        while self
            .examining_char
            .as_ref()
            .map_or(false, char::is_ascii_whitespace)
        {
            self.read_char();
        }
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while self.examining_char.as_ref().map_or(false, is_letter) {
            self.read_char();
        }
        self.input[position..self.position].iter().collect()
    }

    fn read_number(&mut self) -> String {
        let position = self.position;
        while self
            .examining_char
            .as_ref()
            .map_or(false, char::is_ascii_digit)
        {
            self.read_char();
        }
        self.input[position..self.position].iter().collect()
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();

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
            Some(ref ch) if is_letter(ch) => {
                let ident = self.read_identifier();
                Token::new(look_up_ident(&ident), ident)
            }
            Some(ref ch) if ch.is_ascii_digit() => Token::new(TokenType::Int, self.read_number()),
            Some(ch) => Token::new(TokenType::Illegal, ch.to_string()),
            None => Token::new(TokenType::EOF, "".to_string()),
        };

        match token.token_type {
            TokenType::Ident | TokenType::Function | TokenType::Int => {}
            _ => self.read_char(),
        }
        token
    }
}

fn is_letter(ch: &char) -> bool {
    ch.is_ascii_alphabetic() || ch == &'_'
}

fn look_up_ident(ident: &str) -> TokenType {
    KEYWORDS.get(ident).cloned().unwrap_or(TokenType::Ident)
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

    fn setup_expects() -> Vec<(TokenType, &'static str)> {
        vec![
            (TokenType::Let, "let"),
            (TokenType::Ident, "five"),
            (TokenType::Assign, "="),
            (TokenType::Int, "5"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Ident, "ten"),
            (TokenType::Assign, "="),
            (TokenType::Int, "10"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Ident, "add"),
            (TokenType::Assign, "="),
            (TokenType::Function, "fn"),
            (TokenType::LParen, "("),
            (TokenType::Ident, "x"),
            (TokenType::Comma, ","),
            (TokenType::Ident, "y"),
            (TokenType::RParen, ")"),
            (TokenType::LBrace, "{"),
            (TokenType::Ident, "x"),
            (TokenType::Plus, "+"),
            (TokenType::Ident, "y"),
            (TokenType::Semicolon, ";"),
            (TokenType::RBrace, "}"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Ident, "result"),
            (TokenType::Assign, "="),
            (TokenType::Ident, "add"),
            (TokenType::LParen, "("),
            (TokenType::Ident, "five"),
            (TokenType::Comma, ","),
            (TokenType::Ident, "ten"),
            (TokenType::RParen, ")"),
            (TokenType::Semicolon, ";"),
            (TokenType::EOF, ""),
        ]
    }

    fn exact_expect(input: &str, expect: &[(TokenType, &'static str)]) -> (String, String) {
        (
            input
                .chars()
                .filter(|ch| !ch.is_ascii_whitespace())
                .collect::<String>(),
            expect.iter().map(|expect| expect.1).collect::<String>(),
        )
    }

    #[test]
    fn test_next_token() {
        let input = setup_input();
        let expects = setup_expects();
        let (exact_input, exact_expect) = exact_expect(&input, &expects);
        assert_eq!(exact_input, exact_expect);

        let mut lexer = super::Lexer::new(input);

        for expect in expects.iter() {
            let token = lexer.next_token();
            assert_eq!(expect.0, token.token_type);
            assert_eq!(expect.1, token.literal);
        }
    }
}
