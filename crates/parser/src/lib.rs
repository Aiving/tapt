mod statement;

pub mod prelude {
    pub use crate::{Ident, Parse, ParseError, ParseResult, ParseResultExt, Parser, statement::*};
    pub use tapt_lexer::{Lexer, StringPart, Token};
    pub use tapt_shared::{Positioned, Span};
    pub use tapt_typing::*;
}

use peekmore::{PeekMore, PeekMoreIterator};
use std::{
    error::Error,
    fmt,
    ops::{Deref, Range},
    vec::IntoIter,
};
use tapt_lexer::{Lexer, Token};
use tapt_shared::{Positioned, Span};
use tapt_typing::Type;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParseError(String, Option<Span>);

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Error for ParseError {}

impl ParseError {
    pub fn new<T: Into<String>>(value: T, span: Option<Span>) -> Self {
        Self(value.into(), span)
    }

    #[must_use]
    pub fn message(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub const fn location(&self) -> Option<Span> {
        self.1
    }

    fn unexpected_token(token: Option<&Positioned<Token>>) -> Self {
        token.map_or_else(
            || Self("Unexpected nothing".to_string(), None),
            |token| Self(format!("Unexpected {}", token.value), Some(token.span)),
        )
    }

    fn expected_token(expected: &Token, found: Option<&Positioned<Token>>) -> Self {
        found.map_or_else(
            || Self(format!("Expected {expected}, found nothing"), None),
            |found| {
                Self(
                    format!("Expected {expected}, found {}", found.value),
                    Some(found.span),
                )
            },
        )
    }

    fn expected_tokens(expected: &[Token], found: Option<&Positioned<Token>>) -> Self {
        let expected = match expected.len() {
            0 => "nothing".into(),
            1 => expected[0].to_string(),
            value => {
                format!(
                    "{}{} or {}",
                    expected[0],
                    expected[1..value - 1]
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<_>>()
                        .join(", "),
                    expected[value - 1]
                )
            }
        };

        found.map_or_else(
            || Self(format!("Expected {expected}, found nothing"), None),
            |found| {
                Self(
                    format!("Expected {expected}, found {}", found.value),
                    Some(found.span),
                )
            },
        )
    }

    #[must_use]
    pub fn too_much_tokens(tokens: &[Positioned<Token>]) -> Self {
        Self(
            format!(
                "found more than zero ({}) tokens after parsing: {tokens:#?}",
                tokens.len()
            ),
            None,
        )
    }
}

pub type ParseResult<T> = std::result::Result<T, ParseError>;

pub trait ParseResultExt {
    #[must_use]
    fn message<T: Into<String>>(self, message: T) -> Self;
}

impl<V> ParseResultExt for ParseResult<V> {
    fn message<T: Into<String>>(mut self, message: T) -> Self {
        if let Err(error) = &mut self {
            error.0 = message.into();
        }

        self
    }
}

#[derive(Debug)]
pub struct Parser {
    tokens: PeekMoreIterator<IntoIter<Positioned<Token>>>,
}

impl Parser {
    #[must_use]
    pub fn new(tokens: Vec<Positioned<Token>>) -> Self {
        Self {
            tokens: tokens.into_iter().peekmore(),
        }
    }

    /// Consumes the current token only if it exists and is equal to `value`.
    pub fn try_consume(&mut self, value: &Token) -> bool {
        self.tokens.next_if(|token| &token.value == value).is_some()
    }

    fn verify_nth(&mut self, index: usize, token: &Token) -> ParseResult<()> {
        if self
            .tokens
            .peek_nth(index)
            .is_some_and(|value| &value.value == token)
        {
            Ok(())
        } else {
            Err(ParseError::unexpected_token(self.tokens.peek_nth(index)))
        }
    }

    fn verify_nth_if<F: Fn(&Token) -> bool>(&mut self, index: usize, func: F) -> ParseResult<()> {
        if self
            .tokens
            .peek_nth(index)
            .is_some_and(|value| func(&value.value))
        {
            Ok(())
        } else {
            Err(ParseError::unexpected_token(self.tokens.peek_nth(index)))
        }
    }

    /// # Errors
    ///
    /// Will return an error if next token is not equal to "token".
    pub fn verify(&mut self, token: &Token) -> ParseResult<()> {
        self.verify_nth(0, token)
    }

    /// # Errors
    ///
    /// Will return an error if next token at second position is not equal to "token".
    pub fn verify2(&mut self, token: &Token) -> ParseResult<()> {
        self.verify_nth(1, token)
    }

    /// # Errors
    ///
    /// Will return an error if next token at third position is not equal to "token".
    pub fn verify3(&mut self, token: &Token) -> ParseResult<()> {
        self.verify_nth(2, token)
    }

    /// # Errors
    ///
    /// Will return an error if "func" return false.
    pub fn verify_if<F: Fn(&Token) -> bool>(&mut self, func: F) -> ParseResult<()> {
        self.verify_nth_if(0, func)
    }

    /// # Errors
    ///
    /// Will return an error if "func" return false.
    pub fn verify2_if<F: Fn(&Token) -> bool>(&mut self, func: F) -> ParseResult<()> {
        self.verify_nth_if(1, func)
    }

    /// # Errors
    ///
    /// Will return an error if "func" return false.
    pub fn verify3_if<F: Fn(&Token) -> bool>(&mut self, func: F) -> ParseResult<()> {
        self.verify_nth_if(2, func)
    }

    /// Checks if the next token exists and it is equal to `value`.
    pub fn check(&mut self, value: &Token) -> bool {
        self.check_if(|v| v == value)
    }

    /// Checks if the next token exists and it is equal to `value`.
    pub fn check2(&mut self, value: &Token) -> bool {
        self.check2_if(|v| v == value)
    }

    /// Checks if the next token exists and it is equal to `value`.
    pub fn check3(&mut self, value: &Token) -> bool {
        self.check3_if(|v| v == value)
    }

    /// Returns the `bool` result of `func` if the next token exists.
    pub fn check_if<F: Fn(&Token) -> bool>(&mut self, func: F) -> bool {
        self.tokens
            .peek_nth(0)
            .is_some_and(|value| func(&value.value))
    }

    /// Returns the `bool` result of `func` if the next token exists.
    pub fn check2_if<F: Fn(&Token) -> bool>(&mut self, func: F) -> bool {
        self.tokens
            .peek_nth(1)
            .is_some_and(|value| func(&value.value))
    }

    /// Returns the `bool` result of `func` if the next token exists.
    pub fn check3_if<F: Fn(&Token) -> bool>(&mut self, func: F) -> bool {
        self.tokens
            .peek_nth(2)
            .is_some_and(|value| func(&value.value))
    }

    /// Consumes the current token if it exists and is equal to `value`, otherwise returning `ParseError`.
    ///
    /// # Errors
    ///
    /// Returns error if current token is not equal to `value`
    pub fn consume(&mut self, value: &Token) -> ParseResult<Positioned<Token>> {
        self.next_if(|current| current == value)
            .map_or_else(|| Err(ParseError::expected_token(value, self.peek())), Ok)
    }

    /// Consumes the current token if it exists and is equal to one of the values inside `values`, otherwise returning `ParseError`.
    ///
    /// # Errors
    ///
    /// Returns error if current token is not equal to one of the tokens inside `values`
    pub fn consume_one_of(&mut self, values: &[Token]) -> ParseResult<Positioned<Token>> {
        self.next_if(|value| values.contains(value))
            .map_or_else(|| Err(ParseError::expected_tokens(values, self.peek())), Ok)
    }

    /// Consumes the current token if it exists and the result of `func` is `true`, otherwise returning `ParseError`.
    ///
    /// # Errors
    ///
    /// Returns error if result of the `func` is false
    pub fn consume_if<F: Fn(&Token) -> bool>(&mut self, func: F) -> ParseResult<Positioned<Token>> {
        self.next_if(func)
            .map_or_else(|| Err(ParseError::unexpected_token(self.peek())), Ok)
    }

    /// Consumes the current token if it exists and the result of the `func` is `Some(T)`, otherwise returning `ParseError`.
    ///
    /// # Errors
    ///
    /// Returns error if there is no token or result of the `func` is None
    pub fn consume_map<T, F: Fn(&Token) -> Option<T>>(
        &mut self,
        func: F,
    ) -> ParseResult<Positioned<T>> {
        if let Some(value) = self
            .peek()
            .and_then(|value| func(&value.value).map(|result| value.span.wrap(result)))
        {
            self.next();

            Ok(value)
        } else {
            Err(ParseError::unexpected_token(self.peek()))
        }
    }

    /// Consumes the current token and returns it wrapped in `Some` if it exists, otherwise returning `None`.
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<Positioned<Token>> {
        self.tokens.next()
    }

    /// Peeks the current token and returns a reference to it wrapped in `Some` if it exists, otherwise returning `None`.
    pub fn peek(&mut self) -> Option<&Positioned<Token>> {
        self.tokens.peek()
    }

    /// Consumes the current token and returns it wrapped in `Some` if the result of the `func` function is `true`, otherwise returning `None`.
    pub fn next_if<F: Fn(&Token) -> bool>(&mut self, func: F) -> Option<Positioned<Token>> {
        self.tokens.next_if(|value| func(&value.value))
    }

    #[must_use]
    pub fn collect(self) -> Vec<Positioned<Token>> {
        self.tokens.collect()
    }

    pub fn expected_token<T: fmt::Display>(&mut self, expected: T) -> ParseError {
        self.peek().map_or_else(
            || ParseError(format!("Expected {expected}, found nothing"), None),
            |found| {
                ParseError(
                    format!("Expected {expected}, found {}", found.value),
                    Some(found.span),
                )
            },
        )
    }
}

pub trait Parse: Sized {
    /// # Errors
    ///
    /// Returns error if parsing failed
    fn parse_value<T: AsRef<str>>(value: T) -> ParseResult<Positioned<Self>> {
        let mut parser = Parser::new(Lexer::parse(value));

        let value = Self::parse(&mut parser)?;

        parser.try_consume(&Token::EOF);

        Ok(value)
    }

    /// # Errors
    ///
    /// Returns error if parsing failed
    fn parse(parser: &mut Parser) -> ParseResult<Positioned<Self>>;

    /// # Errors
    ///
    /// Returns error if parsing failed
    fn parse_separated(
        parser: &mut Parser,
        separator: &Token,
    ) -> ParseResult<Vec<Positioned<Self>>> {
        let mut values = vec![Self::parse(parser)?];

        while parser.try_consume(separator) {
            values.push(Self::parse(parser)?);
        }

        Ok(values)
    }

    /// # Errors
    ///
    /// Returns error if parsing failed
    fn parse_separated_in(
        parser: &mut Parser,
        separator: &Token,
        from: &Token,
        to: &Token,
    ) -> ParseResult<Positioned<Vec<Positioned<Self>>>> {
        let from = parser.consume(from)?;

        let mut values = vec![];

        while !parser.check(to) {
            if !values.is_empty() {
                parser.consume(separator)?;
            }

            if parser.check(to) {
                break;
            }

            values.push(Self::parse(parser)?);
        }

        let to = parser.consume(to)?;

        Ok(from.between(&to).wrap(values))
    }

    /// # Errors
    ///
    /// Returns error if parsing failed
    fn parse_in(
        parser: &mut Parser,
        from: &Token,
        to: &Token,
    ) -> ParseResult<Positioned<Vec<Positioned<Self>>>> {
        let from = parser.consume(from)?;

        let mut values = Vec::new();

        while !parser.check(to) {
            values.push(Self::parse(parser)?);
        }

        let to = parser.consume(to)?;

        Ok(from.between(&to).wrap(values))
    }

    /// # Errors
    ///
    /// Returns error if parsing failed
    fn parse_until(parser: &mut Parser, value: &Token) -> ParseResult<Vec<Positioned<Self>>> {
        let mut values = Vec::new();

        while !parser.check(value) {
            values.push(Self::parse(parser)?);
        }

        parser.consume(value)?;

        Ok(values)
    }

    /// # Errors
    ///
    /// Returns error if parsing failed
    fn parse_until_range(
        parser: &mut Parser,
        value: &Token,
    ) -> ParseResult<Positioned<Vec<Positioned<Self>>>> {
        let start = parser.peek().unwrap().span;
        let mut values = Vec::new();

        while !parser.check(value) {
            values.push(Self::parse(parser)?);
        }

        let end = parser.consume(value)?;

        Ok(start.between(end.span).wrap(values))
    }
}

impl Parse for f32 {
    fn parse(parser: &mut Parser) -> ParseResult<Positioned<Self>> {
        parser
            .consume_if(Token::is_float)
            .map(|Positioned { value, span }| span.wrap(value.unwrap_float()))
    }
}

impl Parse for usize {
    fn parse(parser: &mut Parser) -> ParseResult<Positioned<Self>> {
        parser
            .consume_if(Token::is_usize)
            .map(|Positioned { value, span }| span.wrap(value.into_usize()))
    }
}

impl Parse for i64 {
    fn parse(parser: &mut Parser) -> ParseResult<Positioned<Self>> {
        parser
            .consume_if(Token::is_integer)
            .map(|Positioned { value, span }| span.wrap(value.unwrap_integer()))
    }
}

impl Parse for String {
    fn parse(parser: &mut Parser) -> ParseResult<Positioned<Self>> {
        parser
            .consume_if(Token::is_string)
            .map(|Positioned { value, span }| span.wrap(value.unwrap_string()))
    }
}

impl Parse for bool {
    fn parse(parser: &mut Parser) -> ParseResult<Positioned<Self>> {
        parser
            .consume_if(Token::is_boolean)
            .map(|Positioned { value, span }| span.wrap(value.unwrap_boolean()))
    }
}

impl Parse for Range<usize> {
    fn parse(parser: &mut Parser) -> ParseResult<Positioned<Self>> {
        parser.verify_if(Token::is_usize)?;
        parser.verify2(&Token::Dot)?;
        parser.verify3(&Token::Dot)?;

        let start = usize::parse(parser)?;

        parser.consume(&Token::Dot)?;
        parser.consume(&Token::Dot)?;

        usize::parse(parser).map(|end| start.between(&end).wrap(start.value..end.value))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Ident(pub String);

impl fmt::Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Parse for Ident {
    fn parse(parser: &mut Parser) -> ParseResult<Positioned<Self>> {
        parser
            .consume_if(Token::is_ident)
            .map(|Positioned { value, span }| span.wrap(Self(value.unwrap_ident())))
    }
}

impl Deref for Ident {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Parse> Parse for Vec<Positioned<T>> {
    fn parse(parser: &mut Parser) -> ParseResult<Positioned<Self>> {
        T::parse_until_range(parser, &Token::EOF)
    }
}

impl Parse for Type {
    fn parse(parser: &mut Parser) -> ParseResult<Positioned<Self>> {
        parser
            .consume_if(|value| value.is_ident_and(|value| value == "int"))
            .map(|value| value.wrap(Self::Integer))
            .or_else(|_| {
                parser
                    .consume_if(|value| value.is_ident_and(|value| value == "float"))
                    .map(|value| value.wrap(Self::Float))
            })
            .or_else(|_| {
                parser
                    .consume_if(|value| value.is_ident_and(|value| value == "bool"))
                    .map(|value| value.wrap(Self::Boolean))
            })
            .or_else(|_| {
                parser
                    .consume_if(|value| value.is_ident_and(|value| value == "string"))
                    .map(|value| value.wrap(Self::String))
            })
    }
}
