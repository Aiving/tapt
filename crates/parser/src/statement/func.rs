use crate::prelude::*;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FuncArg {
    pub name: Positioned<Ident>,
    pub ty: Positioned<Type>,
}

impl fmt::Display for FuncArg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.ty)
    }
}

impl Parse for FuncArg {
    fn parse(parser: &mut Parser) -> ParseResult<Positioned<Self>> {
        let name = Ident::parse(parser)?;

        parser.consume(&Token::Colon)?;

        let ty = Type::parse(parser)?;

        Ok(name.between(&ty).wrap(Self { name, ty }))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FuncStatement {
    pub name: Positioned<Ident>,
    pub args: Positioned<Vec<Positioned<FuncArg>>>,
    pub output_type: Option<Positioned<Type>>,
    pub body: Positioned<Block>,
}

impl fmt::Display for FuncStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "func {}({})",
            self.name,
            self.args
                .value
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(", ")
        )?;

        if let Some(output_type) = &self.output_type {
            write!(f, ": {output_type} ")?;
        } else {
            f.write_str(" ")?;
        }

        write!(f, "{}", self.body)
    }
}

impl Parse for FuncStatement {
    fn parse(parser: &mut Parser) -> ParseResult<Positioned<Self>> {
        parser.verify(&Token::Func)?;
        parser.verify2_if(Token::is_ident)?;

        let start = parser.consume(&Token::Func)?;

        let name = Ident::parse(parser)?;

        let args = FuncArg::parse_separated_in(
            parser,
            &Token::Comma,
            &Token::ParenOpen,
            &Token::ParenClose,
        )?;

        let output_type = if parser.try_consume(&Token::Colon) {
            Some(Type::parse(parser)?)
        } else {
            None
        };

        let body = Block::parse(parser)?;

        Ok(start.between(&body).wrap(Self {
            name,
            args,
            output_type,
            body,
        }))
    }
}
