use crate::prelude::*;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryExpression {
    pub lhs: Positioned<Expression>,
    pub rhs: Positioned<Expression>,
    pub operator: Positioned<Operator>,
}

impl fmt::Display for BinaryExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.lhs, self.operator, self.rhs)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operator {
    Assign,      // =
    Add,         // +
    Sub,         // -
    Mul,         // *
    Div,         // /
    Equal,       // ==
    NotEqual,    // !=
    LessThan,    // <
    GreaterThan, // >
    And,         // &&
    Or,          // ||
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Assign => "=",
            Self::Add => "+",
            Self::Sub => "-",
            Self::Mul => "*",
            Self::Div => "/",
            Self::Equal => "==",
            Self::NotEqual => "!=",
            Self::LessThan => "<",
            Self::GreaterThan => ">",
            Self::And => "&&",
            Self::Or => "||",
        })
    }
}

impl BinaryExpression {
    /// # Errors
    /// 
    /// Returns `ParseError` if parsing failed
    pub fn parse(
        parser: &mut Parser,
        lhs: Positioned<Expression>,
        operator: Positioned<Operator>,
    ) -> ParseResult<Positioned<Self>> {
        let rhs = Expression::parse(parser)?;

        Ok(lhs.between(&rhs).wrap(Self { lhs, rhs, operator }))
    }
}
