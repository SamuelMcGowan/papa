use crate::context::Context;

pub trait Parser {
    type Context: Context;
    type Output;

    fn parse(&self, context: &mut Self::Context) -> ParseResult<Self::Context, Self::Output>;
}

pub struct ParseResult<C: Context, Output> {
    inner: Result<Option<Output>, Option<C::Error>>,
}

impl<C: Context, Output> ParseResult<C, Output> {
    pub fn err(err: Option<C::Error>) -> Self {
        Self { inner: Err(None) }
    }

    pub fn ok(output: Option<Output>) -> Self {
        Self { inner: Ok(output) }
    }

    pub fn to_result(self) -> Result<Option<Output>, Option<C::Error>> {
        self.inner
    }

    pub fn is_ok(&self) -> bool {
        self.inner.is_ok()
    }

    pub fn is_err(&self) -> bool {
        self.inner.is_err()
    }
}

impl<C: Context, T> ParseResult<C, T> {
    pub fn map<U>(self, f: impl Fn(T) -> U) -> ParseResult<C, U> {
        ParseResult {
            inner: self.inner.map(|output| output.map(f)),
        }
    }
}

impl<C: Context, Output> From<()> for ParseResult<C, Output> {
    fn from(_value: ()) -> Self {
        Self { inner: Ok(None) }
    }
}

impl<C: Context, Output> From<Option<Output>> for ParseResult<C, Output> {
    fn from(output: Option<Output>) -> Self {
        Self { inner: Ok(output) }
    }
}

impl<C: Context, Output> From<Result<Output, C::Error>> for ParseResult<C, Output> {
    fn from(output: Result<Output, C::Error>) -> Self {
        Self {
            inner: output.map(Some).map_err(Some),
        }
    }
}
