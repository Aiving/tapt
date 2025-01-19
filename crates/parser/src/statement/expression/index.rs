use crate::prelude::*;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum IndexKind {
    Expr(Expression),
    Number(usize),
    Ident(Ident),
}

impl fmt::Display for IndexKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Expr(value) => value.fmt(f),
            Self::Number(value) => value.fmt(f),
            Self::Ident(value) => value.fmt(f),
        }
    }
}

impl Parse for IndexKind {
    fn parse(parser: &mut Parser) -> ParseResult<Positioned<Self>> {
        if parser.try_consume(&Token::BracketOpen) {
            let value = Expression::parse(parser).map(|value| value.map(Self::Expr));

            parser.consume(&Token::BracketClose)?;

            value
        } else {
            parser.consume(&Token::Dot)?;

            Ident::parse(parser)
                .map(|value| value.map(Self::Ident))
                .or_else(|_| usize::parse(parser).map(|value| value.map(Self::Number)))
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct IndexExpression {
    pub target: Positioned<Expression>,
    pub index: Positioned<IndexKind>,
}

impl fmt::Display for IndexExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.target, self.index)
    }
}

impl IndexExpression {
    /// # Errors
    ///
    /// Returns `ParseError` if parsing failed
    pub fn parse(
        parser: &mut Parser,
        target: Positioned<Expression>,
    ) -> ParseResult<Positioned<Self>> {
        let index = IndexKind::parse(parser)?;

        Ok(target.between(&index).wrap(Self { target, index }))
    }
}
