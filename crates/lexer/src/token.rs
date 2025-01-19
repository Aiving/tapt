use derive_more::derive::{Display, IsVariant, Unwrap};
use tapt_shared::Positioned;

#[derive(Debug, Display, PartialEq, Clone)]
pub enum StringPart {
    String(String),
    #[display("{}", _0.iter().map(ToString::to_string).collect::<String>())]
    Formatted(Vec<Positioned<Token>>),
}

#[derive(Debug, Display, IsVariant, Unwrap, PartialEq, Clone)]
pub enum Token {
    #[is_variant]
    #[unwrap]
    Ident(String),
    #[display("\"{}\"", _0.iter().map(ToString::to_string).collect::<String>())]
    #[is_variant]
    FormattedString(Vec<StringPart>),
    #[display("\"{_0}\"")]
    #[is_variant]
    #[unwrap]
    String(String),
    #[display("{_0}")]
    #[is_variant]
    #[unwrap]
    Float(f32),
    #[display("{_0}")]
    #[is_variant]
    #[unwrap]
    Integer(i64),
    #[display("{_0}")]
    #[is_variant]
    #[unwrap]
    Boolean(bool),
    #[display("record")]
    Record,
    #[display("struct")]
    Struct,
    #[display("const")]
    Const,
    #[display("let")]
    Let,
    #[display("while")]
    While,
    #[display("for")]
    For,
    #[display("in")]
    In,
    #[display("if")]
    If,
    #[display("else")]
    Else,
    #[display("func")]
    Func,
    #[display("match")]
    Match,
    #[display("[")]
    BracketOpen,
    #[display("]")]
    BracketClose,
    #[display("{{")]
    BraceOpen,
    #[display("}}")]
    BraceClose,
    #[display("(")]
    ParenOpen,
    #[display(")")]
    ParenClose,
    #[display(":")]
    Colon,
    #[display(";")]
    Semi,
    #[display("-")]
    Minus,
    #[display("+")]
    Plus,
    #[display("/")]
    Slash,
    #[display("*")]
    Star,
    #[display("=>")]
    FatArrow,
    #[display("=")]
    Eq,
    #[display("==")]
    EqEq,
    #[display("!=")]
    NotEq,
    #[display("!")]
    Not,
    #[display("||")]
    Or,
    #[display("&&")]
    And,
    #[display("#")]
    Pound,
    #[display("%")]
    Percent,
    #[display(",")]
    Comma,
    #[display("<")]
    Less,
    #[display(">")]
    Greater,
    #[display(".")]
    Dot,
    #[display("new")]
    New,
    #[display("<EOF>")]
    EOF,
    #[display("unknown: {_0}")]
    Unknown(char),
}

impl Token {
    #[must_use]
    pub fn is_ident_and(&self, func: impl FnOnce(&String) -> bool) -> bool {
        if let Self::Ident(value) = self {
            func(value)
        } else {
            false
        }
    }

    #[must_use]
    pub const fn is_number(&self) -> bool {
        self.is_float() || self.is_integer()
    }

    #[must_use]
    pub const fn is_usize(&self) -> bool {
        if let Self::Integer(value) = self {
            *value >= 0
        } else {
            false
        }
    }

    #[must_use]
    pub fn into_usize(self) -> usize {
        if let Self::Integer(value) = self {
            let Ok(value) = value.try_into() else {
                unreachable!()
            };

            value
        } else {
            unreachable!()
        }
    }
}
