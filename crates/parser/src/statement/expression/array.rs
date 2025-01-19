use crate::prelude::*;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct ArrayExpr {
    values: Vec<Positioned<Expression>>,
}

impl fmt::Display for ArrayExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.values.is_empty() {
            f.write_str("[]")
        } else {
            f.write_str("[\n")?;

            for value in &self.values {
                writeln!(f, "{value},")?;
            }

            f.write_str("]")
        }
    }
}

impl Parse for ArrayExpr {
    fn parse(parser: &mut Parser) -> ParseResult<Positioned<Self>> {
        Expression::parse_separated_in(
            parser,
            &Token::Comma,
            &Token::BracketOpen,
            &Token::BracketClose,
        )
        .map(
            |Positioned {
                 value: values,
                 span,
             }| span.wrap(Self { values }),
        )
    }
}
