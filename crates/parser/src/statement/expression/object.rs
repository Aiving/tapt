use crate::prelude::*;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct ObjectProperty {
    name: Positioned<Ident>,
    value: Positioned<Expression>,
}

impl fmt::Display for ObjectProperty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.name.value)?;
        f.write_str(": ")?;

        write!(f, "{}", self.value)
    }
}

impl Parse for ObjectProperty {
    fn parse(parser: &mut Parser) -> ParseResult<Positioned<Self>> {
        parser.verify_if(Token::is_ident)?;
        parser.verify2(&Token::Colon)?;

        let name = Ident::parse(parser)?;

        parser.consume(&Token::Colon)?;

        let value = Expression::parse(parser)?;

        Ok(name.between(&value).wrap(Self { name, value }))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ObjectExpr {
    properties: Vec<Positioned<ObjectProperty>>,
}

impl fmt::Display for ObjectExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.properties.is_empty() {
            f.write_str("#{ }")
        } else {
            f.write_str("#{\n")?;

            for property in &self.properties {
                writeln!(f, "{property},")?;
            }

            f.write_str("}")
        }
    }
}

impl Parse for ObjectExpr {
    fn parse(parser: &mut Parser) -> ParseResult<Positioned<Self>> {
        parser.verify(&Token::Pound)?;
        parser.verify2(&Token::BraceOpen)?;

        ObjectProperty::parse_separated_in(
            parser,
            &Token::Comma,
            &Token::BraceOpen,
            &Token::BraceClose,
        )
        .map(
            |Positioned {
                 value: properties,
                 span,
             }| span.wrap(Self { properties }),
        )
    }
}
