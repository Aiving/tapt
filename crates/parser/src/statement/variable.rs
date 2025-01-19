use crate::prelude::*;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct VariableStatement {
    pub mutable: Positioned<bool>,
    pub name: Positioned<Ident>,
    pub ty: Option<Positioned<Type>>,
    pub value: Positioned<Expression>,
}

impl fmt::Display for VariableStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(if self.mutable.value { "let " } else { "const " })?;
        f.write_str(&self.name.value)?;

        if let Some(ty) = &self.ty {
            write!(f, ": {ty}")?;
        }

        write!(f, " = {};", self.value)
    }
}

impl Parse for VariableStatement {
    fn parse(parser: &mut Parser) -> ParseResult<Positioned<Self>> {
        parser
            .verify_if(|value| matches!(value, Token::Let | Token::Const))
            .map_err(|_| parser.expected_token("const or let"))?;

        parser.verify2_if(Token::is_ident)?;

        let def = parser.consume_one_of(&[Token::Let, Token::Const])?;
        let name = Ident::parse(parser)?;
        let ty = if parser.try_consume(&Token::Colon) {
            Some(Type::parse(parser)?)
        } else {
            None
        };

        parser.consume(&Token::Eq)?;

        let value = Expression::parse(parser)?;

        let end = parser.consume(&Token::Semi)?;

        Ok(def.between(&end).wrap(Self {
            mutable: def.wrap(def.value == Token::Let),
            name,
            ty,
            value,
        }))
    }
}
