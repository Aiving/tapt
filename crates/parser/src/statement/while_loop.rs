use crate::prelude::*;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct WhileStatement {
    pub condition: Positioned<Expression>,
    pub body: Positioned<Block>,
}

impl fmt::Display for WhileStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "while {} ", self.condition)?;

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

impl Parse for WhileStatement {
    fn parse(parser: &mut Parser) -> ParseResult<Positioned<Self>> {
        let start = parser.consume(&Token::While)?;

        println!("{parser:#?}");

        let condition = Expression::parse(parser)?;

        let body = Block::parse(parser)?;

        Ok(start.between(&body).wrap(Self { condition, body }))
    }
}
