use crate::context::Context;

pub trait Parser {
    type Context: Context;
    type Output;

    fn parse(&self, context: &mut Self::Context) -> ParseResult<Self::Context, Self::Output>;
}

pub struct ParseResult<C: Context, T> {
    pub output: Option<T>,
    pub error: Option<C::Error>,
}

impl<C: Context, T> ParseResult<C, T> {
    pub fn empty() -> Self {
        Self {
            output: None,
            error: None,
        }
    }

    pub fn from_output(ok: Option<T>) -> Self {
        Self {
            output: ok,
            error: None,
        }
    }

    pub fn from_err(err: Option<C::Error>) -> Self {
        Self {
            output: None,
            error: err,
        }
    }

    pub fn map<U>(self, f: impl Fn(T) -> U) -> ParseResult<C, U> {
        ParseResult {
            output: self.output.map(f),
            error: self.error,
        }
    }
}
