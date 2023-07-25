use std::marker::PhantomData;

use crate::combinator::drop::Drop;
use crate::combinator::filter::Filter;
use crate::combinator::map::Map;
use crate::combinator::repeat::RepetitionBuilder;
use crate::combinator::spanned::Spanned;
use crate::context::Context;

pub trait Parser<C: Context, Output> {
    fn parse(&self, context: &mut C) -> ParseResult<C, Output>;

    fn map<F, OutputB>(self, f: F) -> Map<C, Self, Output, OutputB, F>
    where
        Self: Sized,
        F: Fn(Output) -> OutputB + Copy,
    {
        Map {
            parser: self,
            map: f,
            _phantom: PhantomData,
        }
    }

    fn filter<F>(self, f: F) -> Filter<C, Self, Output, F>
    where
        Self: Sized,
        F: Fn(&Output) -> bool,
    {
        Filter {
            parser: self,
            filter: f,
            _phantom: PhantomData,
        }
    }

    fn drop(self) -> Drop<C, Self, Output>
    where
        Self: Sized,
    {
        Drop {
            parser: self,
            _phantom: PhantomData,
        }
    }

    fn repeat(self) -> RepetitionBuilder<C, Self, Output>
    where
        Self: Sized,
    {
        RepetitionBuilder {
            parser: self,
            min: 0,
            max: None,
            _phantom: PhantomData,
        }
    }

    fn spanned(self) -> Spanned<C, Self, Output>
    where
        Self: Sized,
    {
        Spanned {
            parser: self,
            _phantom: PhantomData,
        }
    }
}

pub struct ParseResult<C: Context, Output> {
    inner: Result<Output, Option<C::Error>>,
}

impl<C: Context, Output> ParseResult<C, Output> {
    pub fn err(err: Option<C::Error>) -> Self {
        Self { inner: Err(err) }
    }

    pub fn ok(output: Output) -> Self {
        Self { inner: Ok(output) }
    }

    pub fn to_result(self) -> Result<Output, Option<C::Error>> {
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
            inner: self.inner.map(f),
        }
    }
}

impl<C: Context, Output: Default> From<()> for ParseResult<C, Output> {
    fn from(_value: ()) -> Self {
        Self {
            inner: Ok(Output::default()),
        }
    }
}

impl<C: Context, Output> From<Option<Output>> for ParseResult<C, Output> {
    fn from(output: Option<Output>) -> Self {
        match output {
            Some(output) => Self { inner: Ok(output) },
            None => Self { inner: Err(None) },
        }
    }
}

impl<C: Context, Output> From<Result<Output, C::Error>> for ParseResult<C, Output> {
    fn from(output: Result<Output, C::Error>) -> Self {
        Self {
            inner: output.map_err(Some),
        }
    }
}
