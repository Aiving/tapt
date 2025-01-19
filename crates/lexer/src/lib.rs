mod token;

pub use self::token::{StringPart, Token};
use peekmore::{PeekMore, PeekMoreIterator};
use std::{iter, mem, ops::Neg, str::Chars};
use tapt_shared::{Positioned, Span};

pub struct Lexer;

impl Lexer {
    fn parse_reserved(ident: String) -> Token {
        match ident.as_str() {
            "true" => Token::Boolean(true),
            "false" => Token::Boolean(false),
            "record" => Token::Record,
            "new" => Token::New,
            "struct" => Token::Struct,
            "match" => Token::Match,
            "func" => Token::Func,
            "const" => Token::Const,
            "let" => Token::Let,
            "while" => Token::While,
            "for" => Token::For,
            "in" => Token::In,
            "if" => Token::If,
            "else" => Token::Else,
            _ => Token::Ident(ident),
        }
    }

    fn parse_other(chars: &mut PeekMoreIterator<Chars>, span: &mut Span, character: char) -> Token {
        match character {
            '[' => Token::BracketOpen,
            ']' => Token::BracketClose,
            '{' => Token::BraceOpen,
            '}' => Token::BraceClose,
            '(' => Token::ParenOpen,
            ')' => Token::ParenClose,
            '!' => {
                if chars.next_if_eq(&'=').is_some() {
                    span.end += 1;
                    span.column += 1;

                    Token::NotEq
                } else {
                    Token::Not
                }
            }
            '=' => {
                if chars.next_if_eq(&'=').is_some() {
                    span.end += 1;
                    span.column += 1;

                    Token::EqEq
                } else if chars.next_if_eq(&'>').is_some() {
                    span.end += 1;
                    span.column += 1;

                    Token::FatArrow
                } else {
                    Token::Eq
                }
            }
            ':' => Token::Colon,
            ';' => Token::Semi,
            '+' => Token::Plus,
            '*' => Token::Star,
            '/' => Token::Slash,
            '%' => Token::Percent,
            '.' => Token::Dot,
            ',' => Token::Comma,
            '#' => Token::Pound,
            '>' => Token::Greater,
            '<' => Token::Less,
            '&' if chars.next_if_eq(&'&').is_some() => {
                span.end += 1;
                span.column += 1;

                Token::And
            }
            '|' if chars.next_if_eq(&'|').is_some() => {
                span.end += 1;
                span.column += 1;

                Token::Or
            }
            character => Token::Unknown(character),
        }
    }

    fn parse_string(chars: &mut PeekMoreIterator<Chars>) -> (Token, usize, usize) {
        let mut size = 0;
        let mut utf8size = 0;
        let mut data = String::new();
        let mut datas = Vec::new();
        let mut formatted = Vec::new();

        while let Some(character) = chars.next_if(|character| character != &'"') {
            size += 1;
            utf8size += character.len_utf8();

            if character == '\\' {
                if let Some(character) = chars.next() {
                    size += 1;
                    utf8size += character.len_utf8();

                    data.push(character);
                }
            }

            if character == '{' {
                datas.push(mem::take(&mut data));

                formatted.push(
                    iter::from_fn(|| {
                        chars.next_if(|&s| s != '{' && s != '}').inspect(|value| {
                            size += 1;
                            utf8size += value.len_utf8();
                        })
                    })
                    .collect::<String>(),
                );

                size += 1;
                utf8size += 1;

                chars.next_if_eq(&'}');
            } else {
                data.push(character);
            }
        }

        chars.next_if(|&s| s == '"');

        if datas.is_empty() && formatted.is_empty() {
            (Token::String(data), size, utf8size)
        } else {
            let mut parts = datas
                .into_iter()
                .map(StringPart::String)
                .zip(
                    formatted
                        .into_iter()
                        .map(|value| StringPart::Formatted(Self::parse(value))),
                )
                .fold(Vec::new(), |mut parts, tuple| {
                    parts.extend(<[StringPart; 2]>::from(tuple));

                    parts
                });

            parts.push(StringPart::String(data));

            (Token::FormattedString(parts), size, utf8size)
        }
    }

    fn parse_number(
        chars: &mut PeekMoreIterator<Chars>,
        tokens: &mut Vec<Positioned<Token>>,
        span: &mut Span,
        character: char,
        neg: bool,
    ) {
        let mut value = iter::once(character)
            .chain(iter::from_fn(|| {
                chars.by_ref().next_if(char::is_ascii_digit)
            }))
            .collect::<String>();

        let [dot, after] = chars.peek_amount(2) else {
            unreachable!()
        };

        if dot == &Some('.') && after.as_ref().is_some_and(char::is_ascii_digit) {
            chars.next();

            value.push('.');
            value.push_str(
                &iter::from_fn(|| chars.by_ref().next_if(char::is_ascii_digit)).collect::<String>(),
            );
        }

        span.start = span.end;
        span.end += value.len();

        if value.contains('.') {
            let mut value: f32 = value.parse().unwrap();

            if neg {
                value = value.neg();
            }

            tokens.push(span.wrap(Token::Float(value)));
        } else {
            let mut value: i64 = value.parse().unwrap();

            if neg {
                value = value.neg();
            }

            tokens.push(span.wrap(Token::Integer(value)));
        }

        span.column += value.len();
    }

    /// # Panics
    ///
    /// Can panic if number failed to parse
    pub fn parse<T: AsRef<str>>(data: T) -> Vec<Positioned<Token>> {
        let mut tokens = vec![];
        let mut chars = data.as_ref().chars().peekmore();
        let mut span = Span::new(0, 0, 0, 0);

        while let Some(character) = chars.next() {
            match character {
                'A'..='z' => {
                    let ident = iter::once(character)
                        .chain(iter::from_fn(|| {
                            chars
                                .by_ref()
                                .next_if(|s| s.is_ascii_alphanumeric() || s == &'-' || s == &'_')
                        }))
                        .collect::<String>();

                    span.start = span.end;
                    span.end += ident.len();
                    span.column += ident.len();

                    tokens.push(span.wrap(Self::parse_reserved(ident)));
                }
                '0'..='9' => {
                    Self::parse_number(&mut chars, &mut tokens, &mut span, character, false);
                }
                '-' => {
                    if let Some(character) = chars.next_if(char::is_ascii_digit) {
                        Self::parse_number(&mut chars, &mut tokens, &mut span, character, true);
                    } else {
                        span.start = span.end;
                        span.end += 1;

                        tokens.push(span.wrap(Token::Minus));

                        span.column += 1;
                    }
                }
                '"' => {
                    span.start = span.end;
                    span.end += 1;
                    span.column += 1;

                    let (value, size, utf8size) = Self::parse_string(&mut chars);

                    span.start = span.end;
                    span.end += utf8size + 1;
                    span.column += size + 1;

                    tokens.push(span.wrap(value));
                }
                character => {
                    span.start = span.end;
                    span.end += 1;

                    if character.is_ascii_whitespace() {
                        if character == '\n' {
                            span.line += 1;
                            span.column = 0;
                        } else {
                            span.column += 1;
                        }

                        continue;
                    }

                    tokens.push(span.wrap(Self::parse_other(&mut chars, &mut span, character)));

                    span.column += 1;
                }
            }
        }

        tokens.push(span.wrap(Token::EOF));

        tokens
    }
}
