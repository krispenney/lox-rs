use crate::lox_err::LoxErr;
use crate::token::{Token, TokenKind};
use colored::*;

#[derive(Debug)]
pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source[self.current - 1]
    }

    fn push_token(&mut self, kind: TokenKind, lexeme: Option<String>) {
        let lexeme = match lexeme {
            Some(l) => l,
            None => self.token_literal(),
        };
        self.tokens.push(Token::new(kind, lexeme, self.line));
    }

    fn token_literal(&self) -> String {
        self.source
            .get(self.start..self.current)
            .unwrap()
            .into_iter()
            .collect()
    }

    fn scan_token(&mut self) -> Result<(), LoxErr> {
        let c = self.advance();
        match c {
            '(' => self.push_token(TokenKind::LeftParen, None),
            ')' => self.push_token(TokenKind::RightParen, None),
            '{' => self.push_token(TokenKind::LeftBrace, None),
            '}' => self.push_token(TokenKind::RightBrace, None),
            ',' => self.push_token(TokenKind::Comma, None),
            '.' => self.push_token(TokenKind::Dot, None),
            '-' => self.push_token(TokenKind::Minus, None),
            '+' => self.push_token(TokenKind::Plus, None),
            ';' => self.push_token(TokenKind::Semicolon, None),
            '*' => self.push_token(TokenKind::Star, None),
            '!' => match self.peek_token() {
                '=' => {
                    self.advance();
                    self.push_token(TokenKind::BangEqual, None);
                }
                _ => self.push_token(TokenKind::Bang, None),
            },
            '=' => match self.peek_token() {
                '=' => {
                    self.advance();
                    self.push_token(TokenKind::EqualEqual, None);
                }
                _ => self.push_token(TokenKind::Equal, None),
            },
            '<' => match self.peek_token() {
                '=' => {
                    self.advance();
                    self.push_token(TokenKind::LessEqual, None);
                }
                _ => self.push_token(TokenKind::Less, None),
            },
            '>' => match self.peek_token() {
                '=' => {
                    self.advance();
                    self.push_token(TokenKind::GreaterEqual, None);
                }
                _ => self.push_token(TokenKind::Greater, None),
            },
            '/' => match self.peek_token() {
                '/' => {
                    self.peek_until('\n');
                }
                _ => self.push_token(TokenKind::Slash, None),
            },
            '"' => {
                self.peek_until('"');

                if self.at_end() {
                    return Err(LoxErr::new(
                        self.line,
                        format!("Unterminated string: '{}'", self.token_literal().bold()),
                    ));
                }

                self.advance(); // catch closing "

                let lexeme = self.token_literal();
                self.push_token(
                    TokenKind::Str,
                    Some(lexeme[1..lexeme.len() - 1].to_string()),
                );
            }
            ' ' | '\r' | '\t' => {} // do nothing
            ('0'..='9') => {
                while !self.at_end() && self.is_digit(&self.peek_token()) {
                    self.advance();
                }

                if self.peek_token() == '.' && self.is_digit(&self.peek_next_token()) {
                    self.advance(); // consume .
                    while self.is_digit(&self.peek_token()) {
                        self.advance();
                    }
                }

                self.push_token(TokenKind::Number, None);
            }
            ('a'..='z') | ('A'..='Z') | '_' => {
                while self.is_alpha_numeric(&self.peek_token()) {
                    self.advance();
                }

                match TokenKind::reserve_kind(&self.token_literal()) {
                    Some(kind) => self.push_token(kind, None),
                    None => self.push_token(TokenKind::Identifier, None),
                }
            }
            '\n' => self.line += 1,
            _ => {
                return Err(LoxErr::new(
                    self.line,
                    format!("Unexpected token: '{}'", self.token_literal().bold()),
                ))
            }
        };
        Ok(())
    }

    fn peek_until(&mut self, expected: char) {
        while !self.at_end() && self.peek_token() != expected {
            self.advance();
        }
    }

    fn peek_token(&self) -> char {
        if self.at_end() {
            '\0'
        } else {
            self.source[self.current]
        }
    }

    fn is_alpha_numeric(&self, c: &char) -> bool {
        ('a'..='z').contains(c) || ('A'..='Z').contains(c) || *c == '_' || self.is_digit(c)
    }

    fn is_digit(&self, c: &char) -> bool {
        ('0'..='9').contains(c)
    }

    fn peek_next_token(&self) -> char {
        if self.source.len() <= self.current + 1 {
            '\0'
        } else {
            self.source[self.current + 1]
        }
    }

    pub fn scan(&mut self) -> Result<&Vec<Token>, Vec<LoxErr>> {
        let mut errors: Vec<LoxErr> = vec![];

        while !self.at_end() {
            self.start = self.current;
            match self.scan_token() {
                Err(e) => errors.push(e),
                _ => continue,
            }
        }
        self.push_token(TokenKind::Eof, Some(String::from("")));

        if errors.len() == 0 {
            Ok(&self.tokens)
        } else {
            Err(errors)
        }
    }

    fn at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
