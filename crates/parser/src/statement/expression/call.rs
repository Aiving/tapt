use crate::prelude::*;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionCall {
    pub target: Box<Positioned<Expression>>,
    pub args: Positioned<Vec<Positioned<Expression>>>,
}

impl fmt::Display for FunctionCall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}({})",
            self.target,
            self.args
                .value
                .iter()
                .map(|value| value.value.to_string())
                .collect::<Vec<_>>()
                .join(", "),
        )
    }
}

impl FunctionCall {
    /// # Errors
    /// 
    /// Returns `ParseError` if parsing failed
    pub fn parse(
        parser: &mut Parser,
        target: Positioned<Expression>,
    ) -> ParseResult<Positioned<Self>> {
        let args = Expression::parse_separated_in(
            parser,
            &Token::Comma,
            &Token::ParenOpen,
            &Token::ParenClose,
        )?;

        Ok(target.between(&args).wrap(Self {
            target: Box::new(target),
            args,
        }))
    }
}
