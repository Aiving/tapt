use crate::prelude::*;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub statements: Vec<Positioned<Statement>>,
    pub return_statement: Option<Box<Positioned<Statement>>>,
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{{")?;

        for statement in &self.statements {
            writeln!(f, "{statement}")?;
        }

        if let Some(statement) = &self.return_statement {
            writeln!(f, "{statement}")?;
        }

        write!(f, "}}")
    }
}

pub type Statements = (Vec<Positioned<Statement>>, Option<Positioned<Statement>>);

impl Block {
    /// Note: this method will not consume `until` token.
    ///
    /// # Errors
    ///
    /// Returns parsing error if parsing fails.
    pub fn parse_statements_until(parser: &mut Parser, until: &Token) -> ParseResult<Statements> {
        let mut statements = Vec::new();
        let mut return_statement: Option<Positioned<Statement>> = None;

        while !parser.check(until) {
            let statement = Statement::parse(parser)?;

            if parser.try_consume(&Token::Semi)
                || matches!(
                    statement.value,
                    Statement::Variable(_)
                        | Statement::ForIn(_)
                        | Statement::WhileLoop(_)
                        | Statement::Record(_)
                        | Statement::Struct(_)
                )
            {
                if let Some(statement) = &return_statement {
                    return Err(ParseError::new("missing ;", Some(statement.span)));
                }

                statements.push(statement);
            } else if let Some(statement) = &return_statement {
                return Err(ParseError::new("missing ;", Some(statement.span)));
            } else {
                return_statement = Some(statement);
            }
        }

        Ok((statements, return_statement))
    }
}

impl Parse for Block {
    fn parse(parser: &mut Parser) -> ParseResult<Positioned<Self>> {
        let from = parser.consume(&Token::BraceOpen)?;

        let (statements, return_statement) =
            Self::parse_statements_until(parser, &Token::BraceClose)?;
        let return_statement = return_statement.map(Box::new);

        let to = parser.consume(&Token::BraceClose)?;

        Ok(from.between(&to).wrap(Self {
            statements,
            return_statement,
        }))
    }
}
