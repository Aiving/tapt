use std::fmt;

#[derive(Debug, Hash, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub line: usize,
    pub column: usize,
}

impl Span {
    #[must_use]
    pub const fn new(start: usize, end: usize, line: usize, column: usize) -> Self {
        Self {
            start,
            end,
            line,
            column,
        }
    }

    #[must_use]
    pub const fn between(&self, to: Self) -> Self {
        Self {
            start: self.start,
            end: to.end,
            line: self.line,
            column: self.column,
        }
    }

    pub const fn wrap<A>(self, value: A) -> Positioned<A> {
        Positioned { value, span: self }
    }
}

#[derive(Debug, Hash, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Positioned<T> {
    pub value: T,
    pub span: Span,
}

impl<T: fmt::Display> fmt::Display for Positioned<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.value.fmt(f)
    }
}

impl<T> Positioned<T> {
    pub const fn new(value: T, span: Span) -> Self {
        Self { value, span }
    }

    pub const fn between<U>(&self, value: &Positioned<U>) -> Span {
        self.span.between(value.span)
    }

    pub const fn wrap<U>(&self, value: U) -> Positioned<U> {
        self.span.wrap(value)
    }

    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Positioned<U> {
        self.span.wrap(f(self.value))
    }

    pub fn unpack(self) -> (Span, T) {
        (self.span, self.value)
    }
}
