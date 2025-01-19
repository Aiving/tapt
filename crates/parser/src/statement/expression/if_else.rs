use crate::prelude::*;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct IfElseExpression {
    pub condition: Box<Positioned<Expression>>,
    pub block: Positioned<Block>,
    pub else_block: Option<Box<Positioned<Expression>>>,
}

impl fmt::Display for IfElseExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let result = write!(f, "if {} {}", self.condition, self.block);

        if let Some(else_block) = &self.else_block {
            write!(f, " {else_block}")?;
        }

        result
    }
}

impl Parse for IfElseExpression {
    fn parse(parser: &mut Parser) -> ParseResult<Positioned<Self>> {
        let start = parser.consume(&Token::If)?;

        let condition = Expression::parse(parser)?;
        let block = Block::parse(parser)?;

        let else_block = if parser.try_consume(&Token::Else) {
            if parser.check(&Token::If) {
                Some(Self::parse(parser)?.map(Expression::IfElse))
            } else {
                Some(Block::parse(parser)?.map(Expression::Block))
            }
        } else {
            None
        }
        .map(Box::new);

        Ok(else_block
            .as_ref()
            .map_or_else(
                || start.between(&block),
                |else_block| start.between(else_block),
            )
            .wrap(Self {
                condition: Box::new(condition),
                block,
                else_block,
            }))
    }
}
