use crate::prelude::*;
use std::{fmt, ops::Range};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Number {
    Float(f32),
    Int(i64),
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Float(value) => value.fmt(f),
            Self::Int(value) => value.fmt(f),
        }
    }
}

impl Parse for Number {
    fn parse(parser: &mut Parser) -> ParseResult<Positioned<Self>> {
        f32::parse(parser)
            .map(|Positioned { value, span }| span.wrap(Self::Float(value)))
            .or_else(|_| {
                i64::parse(parser).map(|Positioned { value, span }| span.wrap(Self::Int(value)))
            })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Number(Number),
    // Length(Length),
    String(String),
    Boolean(bool),
    Range(Range<usize>),
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(value) => value.fmt(f),
            Self::String(value) => write!(f, "{value:?}"),
            Self::Boolean(value) => value.fmt(f),
            Self::Range(value) => write!(f, "{}..{}", value.start, value.end),
        }
    }
}

impl Parse for Literal {
    fn parse(parser: &mut Parser) -> ParseResult<Positioned<Self>> {
        bool::parse(parser)
            .map(|Positioned { value, span }| span.wrap(Self::Boolean(value)))
            .or_else(|_| {
                Range::parse(parser).map(|Positioned { value, span }| span.wrap(Self::Range(value)))
            })
            .or_else(|_| {
                Number::parse(parser)
                    .map(|Positioned { value, span }| span.wrap(Self::Number(value)))
            })
            // .or_else(|_| Length::parse(parser).map(Self::Length))
            .or_else(|_| {
                String::parse(parser)
                    .map(|Positioned { value, span }| span.wrap(Self::String(value)))
            })
    }
}
