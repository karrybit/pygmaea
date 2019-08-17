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

        let token = match self.examining_char {
            Some(ch) if ch == '+' => Token::new(TokenType::Plus, ch.to_string()),
            Some(ch) if ch == '-' => Token::new(TokenType::Minus, ch.to_string()),
            Some(ch) if ch == '*' => Token::new(TokenType::Asterisk, ch.to_string()),
            Some(ch) if ch == '/' => Token::new(TokenType::Slash, ch.to_string()),
            Some(ch) if ch == '=' => Token::new(TokenType::Assign, ch.to_string()),
            Some(ch) if ch == '!' => Token::new(TokenType::Bang, ch.to_string()),
            Some(ch) if ch == '<' => Token::new(TokenType::LT, ch.to_string()),
            Some(ch) if ch == '>' => Token::new(TokenType::GT, ch.to_string()),
            Some(ch) if ch == '(' => Token::new(TokenType::LParen, ch.to_string()),
            Some(ch) if ch == ')' => Token::new(TokenType::RParen, ch.to_string()),
            Some(ch) if ch == '{' => Token::new(TokenType::LBrace, ch.to_string()),
            Some(ch) if ch == '}' => Token::new(TokenType::RBrace, ch.to_string()),
            Some(ch) if ch == ',' => Token::new(TokenType::Comma, ch.to_string()),
            Some(ch) if ch == ';' => Token::new(TokenType::Semicolon, ch.to_string()),
            Some(ref ch) if ch.is_ascii_digit() => Token::new(TokenType::Int, self.read_number()),
            Some(ref ch) if is_letter(ch) => {
                let ident = self.read_identifier();
                Token::new(look_up_ident(&ident), ident)
            }
            Some(ch) => Token::new(TokenType::Illegal, ch.to_string()),
            None => Token::new(TokenType::EOF, "".to_string()),
        };

        if !(token.token_type.is_keyword() || token.token_type.is_int()) {
            self.read_char();
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
        !-/*5;
        5 < 10 > 5;

        if (5 < 10) {
            return true;
        } else {
            return false;
        }
        "
        .to_string()
    }

    fn setup_expects() -> Vec<(TokenType, &'static str)> {
        use TokenType::*;
        vec![
            (Let, "let"),
            (Ident, "five"),
            (Assign, "="),
            (Int, "5"),
            (Semicolon, ";"),
            (Let, "let"),
            (Ident, "ten"),
            (Assign, "="),
            (Int, "10"),
            (Semicolon, ";"),
            (Let, "let"),
            (Ident, "add"),
            (Assign, "="),
            (Function, "fn"),
            (LParen, "("),
            (Ident, "x"),
            (Comma, ","),
            (Ident, "y"),
            (RParen, ")"),
            (LBrace, "{"),
            (Ident, "x"),
            (Plus, "+"),
            (Ident, "y"),
            (Semicolon, ";"),
            (RBrace, "}"),
            (Semicolon, ";"),
            (Let, "let"),
            (Ident, "result"),
            (Assign, "="),
            (Ident, "add"),
            (LParen, "("),
            (Ident, "five"),
            (Comma, ","),
            (Ident, "ten"),
            (RParen, ")"),
            (Semicolon, ";"),
            (Bang, "!"),
            (Minus, "-"),
            (Slash, "/"),
            (Asterisk, "*"),
            (Int, "5"),
            (Semicolon, ";"),
            (Int, "5"),
            (LT, "<"),
            (Int, "10"),
            (GT, ">"),
            (Int, "5"),
            (Semicolon, ";"),
            (If, "if"),
            (LParen, "("),
            (Int, "5"),
            (LT, "<"),
            (Int, "10"),
            (RParen, ")"),
            (LBrace, "{"),
            (Return, "return"),
            (True, "true"),
            (Semicolon, ";"),
            (RBrace, "}"),
            (Else, "else"),
            (LBrace, "{"),
            (Return, "return"),
            (False, "false"),
            (Semicolon, ";"),
            (RBrace, "}"),
            (EOF, ""),
        ]
    }

    fn exact_expect(input: &str, expect: &[(TokenType, &'static str)]) -> (String, String) {
        (
            input
                .chars()
                .partition::<String, _>(char::is_ascii_whitespace)
                .1,
            expect.iter().map(|expect| expect.1).collect(),
        )
    }

    #[test]
    fn test_next_token() {
        let input = setup_input();
        let expects = setup_expects();
        let (exact_input, exact_expect) = exact_expect(&input, &expects);
        assert_eq!(exact_input, exact_expect);

        let mut lexer = super::Lexer::new(input);

        expects.iter().enumerate().for_each(|(i, expect)| {
            let token = lexer.next_token();
            assert_eq!(
                expect.0, token.token_type,
                "tests[{}] - tokentype wrong. expected={}, got={}",
                i, expect.0, token.token_type
            );
            assert_eq!(
                expect.1, token.literal,
                "tests[{}] - literal wrong. expected={}, got={}",
                i, expect.1, token.literal
            );
        });
    }
}
