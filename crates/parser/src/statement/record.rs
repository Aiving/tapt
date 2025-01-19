use crate::prelude::*;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordStatement {
    pub name: Positioned<Ident>,
    pub fields: Positioned<Vec<Positioned<Type>>>,
}

impl fmt::Display for RecordStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "record {}({});",
            self.name,
            self.fields
                .value
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

impl Parse for RecordStatement {
    fn parse(parser: &mut Parser) -> ParseResult<Positioned<Self>> {
        parser.verify(&Token::Record)?;
        parser.verify2_if(Token::is_ident)?;

        let start = parser.consume(&Token::Record)?;
        let name = Ident::parse(parser)?;

        let fields =
            Type::parse_separated_in(parser, &Token::Comma, &Token::ParenOpen, &Token::ParenClose)?;

        let end = parser.consume(&Token::Semi)?;

        Ok(start.between(&end).wrap(Self { name, fields }))
    }
}
