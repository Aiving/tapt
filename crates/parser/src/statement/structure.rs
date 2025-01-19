use crate::prelude::*;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructField {
    pub name: Positioned<Ident>,
    pub ty: Positioned<Type>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructStatement {
    pub name: Positioned<Ident>,
    pub fields: Positioned<Vec<Positioned<StructField>>>,
}

impl fmt::Display for StructStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "struct {} {{\n{}\n}};",
            self.name,
            self.fields
                .value
                .iter()
                .map(|field| format!("  {}: {}", field.value.name, field.value.ty))
                .collect::<Vec<_>>()
                .join(",\n")
        )
    }
}

impl Parse for StructField {
    fn parse(parser: &mut Parser) -> ParseResult<Positioned<Self>> {
        let name = Ident::parse(parser)?;

        parser.consume(&Token::Colon)?;

        let ty = Type::parse(parser)?;

        Ok(name.between(&ty).wrap(Self { name, ty }))
    }
}

impl Parse for StructStatement {
    fn parse(parser: &mut Parser) -> ParseResult<Positioned<Self>> {
        parser.verify(&Token::Struct)?;
        parser.verify2_if(Token::is_ident)?;

        let start = parser.consume(&Token::Struct)?;
        let name = Ident::parse(parser)?;

        let fields = StructField::parse_separated_in(
            parser,
            &Token::Comma,
            &Token::BraceOpen,
            &Token::BraceClose,
        )?;

        Ok(start.between(&fields).wrap(Self { name, fields }))
    }
}
