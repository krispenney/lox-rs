use crate::expression::Expression;
use crate::lox_err::LoxErr;
use crate::token::{Token, TokenKind};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens: tokens,
            current: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Expression, LoxErr> {
        self.parse_equality()
    }

    // equality → comparison ( ( "!=" | "==" ) comparison )*
    fn parse_equality(&mut self) -> Result<Expression, LoxErr> {
        let mut expr = self.parse_comparison()?;
        while self.match_tokens(&vec![TokenKind::BangEqual, TokenKind::EqualEqual]) {
            let operator = self.previous();
            let right = self.parse_comparison()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    // comparison → addition ( ( ">" | ">=" | "<" | "<=" ) addition )*
    fn parse_comparison(&mut self) -> Result<Expression, LoxErr> {
        let mut expr = self.parse_addition()?;
        let operators = vec![
            TokenKind::Greater,
            TokenKind::GreaterEqual,
            TokenKind::Less,
            TokenKind::LessEqual,
        ];

        while self.match_tokens(&operators) {
            let operator = self.previous();
            let right = self.parse_addition()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn parse_addition(&mut self) -> Result<Expression, LoxErr> {
        let mut expr = self.parse_multiplication()?;
        while self.match_tokens(&vec![TokenKind::Minus, TokenKind::Plus]) {
            let operator = self.previous();
            let right = self.parse_multiplication()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            }
        }
        Ok(expr)
    }

    fn parse_multiplication(&mut self) -> Result<Expression, LoxErr> {
        let mut expr = self.parse_unary()?;
        while self.match_tokens(&vec![TokenKind::Slash, TokenKind::Star]) {
            let operator = self.previous();
            let right = self.parse_unary()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            }
        }
        Ok(expr)
    }

    fn parse_unary(&mut self) -> Result<Expression, LoxErr> {
        if self.match_tokens(&vec![TokenKind::Bang, TokenKind::Minus]) {
            let operator = self.previous();
            let right = self.parse_unary()?;
            Ok(Expression::Unary {
                operator: operator,
                right: Box::new(right),
            })
        } else {
            self.parse_primary()
        }
    }

    fn parse_primary(&mut self) -> Result<Expression, LoxErr> {
        if self.match_tokens(&vec![TokenKind::True]) {
            Ok(Expression::BoolLiteral(true))
        } else if self.match_tokens(&vec![TokenKind::False]) {
            Ok(Expression::BoolLiteral(false))
        } else if self.match_tokens(&vec![TokenKind::Nil]) {
            Ok(Expression::NilLiteral)
        } else if self.match_tokens(&vec![TokenKind::Number]) {
            let number_token = self.previous();
            match number_token.lexeme.parse() {
                Ok(v) => Ok(Expression::NumberLiteral(v)),
                Err(_) => Err(LoxErr::new(
                    number_token.line,
                    format!("Could not parse number: {}", number_token.lexeme),
                )),
            }
        } else if self.match_tokens(&vec![TokenKind::Str]) {
            Ok(Expression::StringLiteral(self.previous().lexeme))
        } else if self.match_tokens(&vec![TokenKind::LeftParen]) {
            let expr = self.parse_comparison()?;
            self.consume(TokenKind::RightParen)?;

            Ok(Expression::Grouping(Box::new(expr)))
        } else {
            let token = self.peek();
            Err(LoxErr::new(
                token.line,
                format!("Unknown primary: {:?}", token.lexeme),
            ))
        }
    }

    fn match_tokens(&mut self, token_kinds: &Vec<TokenKind>) -> bool {
        for kind in token_kinds {
            if self.check(kind) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, kind: &TokenKind) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().kind == *kind
        }
    }

    fn is_at_end(&self) -> bool {
        self.peek().kind == TokenKind::Eof
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn consume(&mut self, kind: TokenKind) -> Result<(), LoxErr> {
        let expected = vec![kind];
        if !self.match_tokens(&expected) {
            let token = self.peek();
            Err(LoxErr::new(
                token.line,
                format!(
                    "Unexpected token. expected: {:?}, got: {:?}",
                    expected.first(),
                    token.kind
                )
            ))
        } else {
            Ok(())
        }
    }
}
