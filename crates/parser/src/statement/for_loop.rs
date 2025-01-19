use crate::prelude::*;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct ForStatement {
    pub name: Positioned<Ident>,
    pub target: Positioned<Expression>,
    pub body: Positioned<Block>,
}

impl fmt::Display for ForStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("for ")?;
        f.write_str(&self.name.value)?;
        f.write_str(" in ")?;

        write!(f, "{} ", self.target)?;

        if self.body.value.statements.is_empty() {
            f.write_str("{ }")
        } else {
            write!(
                f,
                "{{\n{}\n}}",
                self.body
                    .value
                    .statements
                    .iter()
                    .map(|value| value.value.to_string())
                    .collect::<Vec<_>>()
                    .join("\n")
            )
        }
    }
}

impl Parse for ForStatement {
    fn parse(parser: &mut Parser) -> ParseResult<Positioned<Self>> {
        parser.verify(&Token::For)?;
        parser.verify2_if(Token::is_ident)?;
        parser.verify3(&Token::In)?;

        let start = parser.consume(&Token::For)?;

        let name = Ident::parse(parser)?;

        parser.consume(&Token::In)?;

        let target = Expression::parse(parser)?;

        let body = Block::parse(parser)?;

        Ok(start.between(&body).wrap(Self { name, target, body }))
    }
}
