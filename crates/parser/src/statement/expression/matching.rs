use crate::prelude::*;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum MatchCase {
    Ident(Ident),
    Value(Expression),
}

impl fmt::Display for MatchCase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ident(value) => value.fmt(f),
            Self::Value(value) => value.fmt(f),
        }
    }
}

impl Parse for MatchCase {
    fn parse(parser: &mut Parser) -> ParseResult<Positioned<Self>> {
        Ident::parse(parser)
            .map(|value| value.map(Self::Ident))
            .or_else(|_| Expression::parse(parser).map(|value| value.map(Self::Value)))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchVariant {
    pub case: Positioned<MatchCase>,
    pub then: Positioned<Expression>,
}

impl fmt::Display for MatchVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} => {}", self.case, self.then)
    }
}

impl Parse for MatchVariant {
    fn parse(parser: &mut Parser) -> ParseResult<Positioned<Self>> {
        let case = MatchCase::parse(parser)?;

        parser.consume(&Token::FatArrow)?;

        let then = Expression::parse(parser)?;

        Ok(case.between(&then).wrap(Self { case, then }))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchExpression {
    pub target: Box<Positioned<Expression>>,
    pub variants: Vec<Positioned<MatchVariant>>,
}

impl fmt::Display for MatchExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "match {} {{", self.target)?;

        for variant in &self.variants {
            writeln!(f, "{variant},")?;
        }

        write!(f, "}}")
    }
}

impl Parse for MatchExpression {
    fn parse(parser: &mut Parser) -> ParseResult<Positioned<Self>> {
        let start = parser.consume(&Token::Match)?;

        let target = Expression::parse(parser).map(Box::new)?;

        let variants = MatchVariant::parse_separated_in(
            parser,
            &Token::Comma,
            &Token::BraceOpen,
            &Token::BraceClose,
        )?;

        Ok(start.between(&variants).wrap(Self {
            target,
            variants: variants.value,
        }))
    }
}
