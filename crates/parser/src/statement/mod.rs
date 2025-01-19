mod expression;
mod for_loop;
mod func;
mod record;
mod structure;
mod variable;
mod while_loop;

pub use self::{
    expression::*,
    for_loop::ForStatement,
    func::{FuncArg, FuncStatement},
    record::RecordStatement,
    structure::{StructField, StructStatement},
    variable::VariableStatement,
    while_loop::WhileStatement,
};
use crate::prelude::*;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Variable(VariableStatement),
    Struct(StructStatement),
    Record(RecordStatement),
    Func(FuncStatement),
    ForIn(ForStatement),
    WhileLoop(WhileStatement),
    Expression(Expression),
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Variable(value) => value.fmt(f),
            Self::Struct(value) => value.fmt(f),
            Self::Record(value) => value.fmt(f),
            Self::Func(value) => value.fmt(f),
            Self::ForIn(value) => value.fmt(f),
            Self::WhileLoop(value) => value.fmt(f),
            Self::Expression(value) => value.fmt(f),
        }
    }
}

impl Parse for Statement {
    fn parse(parser: &mut Parser) -> ParseResult<Positioned<Self>> {
        let value = VariableStatement::parse(parser)
            .map(|value| value.map(Self::Variable))
            .or_else(|_| StructStatement::parse(parser).map(|value| value.map(Self::Struct)))
            .or_else(|_| RecordStatement::parse(parser).map(|value| value.map(Self::Record)))
            .or_else(|_| FuncStatement::parse(parser).map(|value| value.map(Self::Func)))
            .or_else(|_| ForStatement::parse(parser).map(|value| value.map(Self::ForIn)))
            .or_else(|_| WhileStatement::parse(parser).map(|value| value.map(Self::WhileLoop)))
            .or_else(|_| Expression::parse(parser).map(|value| value.map(Self::Expression)))?;

        Ok(value)
    }
}
