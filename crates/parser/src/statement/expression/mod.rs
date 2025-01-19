mod array;
mod binary;
mod block;
mod call;
mod if_else;
mod index;
mod literal;
mod matching;
mod new;
mod object;

pub use self::{
    array::ArrayExpr,
    binary::{BinaryExpression, Operator},
    block::Block,
    call::FunctionCall,
    if_else::IfElseExpression,
    index::{IndexExpression, IndexKind},
    literal::{Literal, Number},
    matching::{MatchCase, MatchExpression, MatchVariant},
    new::{InstanceArgs, NewInstanceExpression, StructFieldValue},
    object::{ObjectExpr, ObjectProperty},
};
use crate::prelude::*;
use std::fmt;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Hash)]
pub enum Precedence {
    PLowest,
    PEquals,
    PLessGreater,
    PSum,
    PProduct,
    PCall,
    PIndex,
}

impl Precedence {
    const fn from_ref(token: &Positioned<Token>) -> (Self, Option<Positioned<Operator>>) {
        match &token.value {
            Token::Eq => (Self::PEquals, Some(token.span.wrap(Operator::Assign))),
            Token::EqEq => (Self::PEquals, Some(token.span.wrap(Operator::Equal))),
            Token::NotEq => (Self::PEquals, Some(token.span.wrap(Operator::NotEqual))),
            Token::Less => (
                Self::PLessGreater,
                Some(token.span.wrap(Operator::LessThan)),
            ),
            Token::Greater => (
                Self::PLessGreater,
                Some(token.span.wrap(Operator::GreaterThan)),
            ),
            Token::Plus => (Self::PSum, Some(token.span.wrap(Operator::Add))),
            Token::Minus => (Self::PSum, Some(token.span.wrap(Operator::Sub))),
            Token::Star => (Self::PProduct, Some(token.span.wrap(Operator::Mul))),
            Token::Slash => (Self::PProduct, Some(token.span.wrap(Operator::Div))),
            Token::ParenOpen => (Self::PCall, None),
            Token::BracketOpen | Token::Dot => (Self::PIndex, None),
            _ => (Self::PLowest, None),
        }
    }
}

fn go_parse_pratt_expr(
    parser: &mut Parser,
    precedence: Precedence,
    left: Positioned<Expression>,
) -> ParseResult<Positioned<Expression>> {
    if let Some(value) = parser.peek() {
        let (p, operator) = Precedence::from_ref(value);

        if operator.is_some() {
            parser.next();
        }

        match p {
            Precedence::PCall if precedence < Precedence::PCall => {
                let left = FunctionCall::parse(parser, left)?.map(Expression::FunctionCall);

                go_parse_pratt_expr(parser, precedence, left)
            }
            Precedence::PIndex if precedence < Precedence::PIndex => {
                let left = IndexExpression::parse(parser, left)?
                    .map(Box::new)
                    .map(Expression::Index);

                go_parse_pratt_expr(parser, precedence, left)
            }
            ref peek_precedence if precedence < *peek_precedence => {
                let left = if let Some(operator) = operator {
                    BinaryExpression::parse(parser, left, operator)?
                        .map(Box::new)
                        .map(Expression::Binary)
                } else {
                    left
                };

                go_parse_pratt_expr(parser, precedence, left)
            }
            _ => Ok(left),
        }
    } else {
        Ok(left)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Literal(Literal),
    FunctionCall(FunctionCall),
    Ident(Ident),
    Object(ObjectExpr),
    Array(ArrayExpr),
    NewInstance(NewInstanceExpression),
    IfElse(IfElseExpression),
    Block(Block),
    Match(MatchExpression),
    Binary(Box<BinaryExpression>),
    Index(Box<IndexExpression>),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Literal(value) => value.fmt(f),
            Self::FunctionCall(value) => value.fmt(f),
            Self::Ident(value) => value.fmt(f),
            Self::Array(value) => value.fmt(f),
            Self::Object(value) => value.fmt(f),
            Self::NewInstance(value) => value.fmt(f),
            Self::IfElse(value) => value.fmt(f),
            Self::Block(value) => value.fmt(f),
            Self::Match(value) => value.fmt(f),
            Self::Binary(value) => value.fmt(f),
            Self::Index(value) => value.fmt(f),
        }
    }
}

impl Parse for Expression {
    fn parse(parser: &mut Parser) -> ParseResult<Positioned<Self>> {
        let value = Literal::parse(parser)
            .map(|value| value.map(Self::Literal))
            .or_else(|_| ObjectExpr::parse(parser).map(|value| value.map(Self::Object)))
            .or_else(|_| ArrayExpr::parse(parser).map(|value| value.map(Self::Array)))
            .or_else(|_| {
                NewInstanceExpression::parse(parser).map(|value| value.map(Self::NewInstance))
            })
            .or_else(|_| MatchExpression::parse(parser).map(|value| value.map(Self::Match)))
            .or_else(|_| IfElseExpression::parse(parser).map(|value| value.map(Self::IfElse)))
            .or_else(|_| Block::parse(parser).map(|value| value.map(Self::Block)))
            .or_else(|_| Ident::parse(parser).map(|value| value.map(Self::Ident)))?;

        go_parse_pratt_expr(parser, Precedence::PLowest, value)
    }
}
