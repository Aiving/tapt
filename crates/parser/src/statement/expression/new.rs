use crate::prelude::*;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct StructFieldValue {
    pub name: Positioned<Ident>,
    pub value: Positioned<Expression>,
}

impl Parse for StructFieldValue {
    fn parse(parser: &mut Parser) -> ParseResult<Positioned<Self>> {
        let name = Ident::parse(parser)?;

        parser.consume(&Token::Colon)?;

        let value = Expression::parse(parser)?;

        Ok(name.between(&value).wrap(Self { name, value }))
    }
}

impl fmt::Display for StructFieldValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum InstanceArgs {
    Struct(Vec<Positioned<StructFieldValue>>),
    Record(Vec<Positioned<Expression>>),
}

impl Parse for InstanceArgs {
    fn parse(parser: &mut Parser) -> ParseResult<Positioned<Self>> {
        StructFieldValue::parse_separated_in(
            parser,
            &Token::Comma,
            &Token::BraceOpen,
            &Token::BraceClose,
        )
        .map(|value| value.map(Self::Struct))
        .or_else(|_| {
            Expression::parse_separated_in(
                parser,
                &Token::Comma,
                &Token::ParenOpen,
                &Token::ParenClose,
            )
            .map(|value| value.map(Self::Record))
        })
    }
}

impl fmt::Display for InstanceArgs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Struct(fields) => write!(
                f,
                "{{\n{}\n}}",
                fields
                    .iter()
                    .map(|field| format!("  {field}"))
                    .collect::<Vec<_>>()
                    .join(",\n")
            ),
            Self::Record(fields) => write!(
                f,
                "({})",
                fields
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NewInstanceExpression {
    pub target: Positioned<Ident>,
    pub args: Positioned<InstanceArgs>,
}

impl fmt::Display for NewInstanceExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "new {}{}", self.target, self.args)
    }
}

impl Parse for NewInstanceExpression {
    fn parse(parser: &mut Parser) -> ParseResult<Positioned<Self>> {
        let start = parser.consume(&Token::New)?;

        let target = Ident::parse(parser)?;

        let args = InstanceArgs::parse(parser)?;

        Ok(start.between(&args).wrap(Self { target, args }))
    }
}
