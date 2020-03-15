use crate::token::Token;
use std::fmt;

#[derive(Debug)]
pub enum Expression {
    Binary {
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>,
    },
    Unary {
        operator: Token,
        right: Box<Expression>,
    },
    Grouping(Box<Expression>),
    NumberLiteral(f64),
    StringLiteral(String),
    BoolLiteral(bool),
    NilLiteral,
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::NumberLiteral(n) => write!(f, "{}", n),
            Expression::StringLiteral(s) => write!(f, "'{}'", s),
            Expression::BoolLiteral(b) => write!(f, "{}", b),
            Expression::NilLiteral => write!(f, "nil"),
            Expression::Grouping(e) => write!(f, "({})", e),
            Expression::Unary { operator, right } => write!(f, "({} {})", operator, right),
            Expression::Binary {
                left,
                operator,
                right,
            } => write!(f, "({} {} {})", operator, left, right),
        }
    }
}
