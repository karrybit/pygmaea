use crate::token::Token;
use crate::token_type::{TokenType, KEYWORDS};

#[derive(Default)]
pub struct Lexer {
    input: Vec<char>,
    position: usize,
    read_position: usize,
    examining_char: Option<char>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Self {
            input: input.chars().collect(),
            ..Default::default()
        };
        lexer.read_char();
        lexer
    }

    pub fn next_token(&mut self) -> Token {
        use TokenType::*;

        self.skip_whitespace();

        let token = match self.examining_char {
            Some(ch) if ch == '+' => Token::new(Plus, ch.to_string()),
            Some(ch) if ch == '-' => Token::new(Minus, ch.to_string()),
            Some(ch) if ch == '*' => Token::new(Asterisk, ch.to_string()),
            Some(ch) if ch == '/' => Token::new(Slash, ch.to_string()),
            Some(ch) if ch == '=' => {
                if let Some('=') = self.input.get(self.read_position) {
                    self.read_char();
                    Token::new(Equal, format!("{}{}", ch, self.examining_char.unwrap()))
                } else {
                    Token::new(Assign, ch.to_string())
                }
            }
            Some(ch) if ch == '!' => {
                if let Some('=') = self.input.get(self.read_position) {
                    self.read_char();
                    Token::new(NotEqual, format!("{}{}", ch, self.examining_char.unwrap()))
                } else {
                    Token::new(Bang, ch.to_string())
                }
            }
            Some(ch) if ch == '<' => Token::new(LessThan, ch.to_string()),
            Some(ch) if ch == '>' => Token::new(GreaterThan, ch.to_string()),
            Some(ch) if ch == '(' => Token::new(LParen, ch.to_string()),
            Some(ch) if ch == ')' => Token::new(RParen, ch.to_string()),
            Some(ch) if ch == '{' => Token::new(LBrace, ch.to_string()),
            Some(ch) if ch == '}' => Token::new(RBrace, ch.to_string()),
            Some(ch) if ch == ',' => Token::new(Comma, ch.to_string()),
            Some(ch) if ch == ';' => Token::new(Semicolon, ch.to_string()),
            Some(ch) if ch.is_ascii_digit() => Token::new(Int, self.read_number()),
            Some(ch) if is_letter(ch) => {
                let ident = self.read_identifier();
                Token::new(look_up_ident(&ident), ident)
            }
            Some(ch) => Token::new(Illegal, ch.to_string()),
            None => Token::new(EOF, "".to_string()),
        };

        if !(token.token_type.is_keyword() || token.token_type.is_int()) {
            self.read_char();
        }
        token
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

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while self.examining_char.map_or(false, is_letter) {
            self.read_char();
        }
        self.input[position..self.position].iter().collect()
    }
}

fn is_letter(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_'
}

fn look_up_ident(ident: &str) -> TokenType {
    KEYWORDS.get(ident).cloned().unwrap_or(TokenType::Ident)
}
