use std::marker::PhantomData;

use crate::combinator::drop::Drop;
use crate::combinator::filter::Filter;
use crate::combinator::map::Map;
use crate::combinator::repeat::{NoRepeatOutput, Repeat};
use crate::combinator::spanned::Spanned;
use crate::combinator::to_slice::ToSlice;
use crate::context::Context;

pub trait Parser<'a, C: Context<'a>, Output> {
    /// Run this parser.
    fn parse(&self, context: &mut C) -> ParseResult<'a, C, Output>;

    /// Map the output of this parser to some other value.
    fn map<F, OutputB>(self, f: F) -> Map<'a, C, Self, Output, OutputB, F>
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

    /// Decide whether to accept an output.
    fn filter<F>(self, f: F) -> Filter<'a, C, Self, Output, F>
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

    /// Convert the output of this parser to `()`.
    fn drop(self) -> Drop<'a, C, Self, Output>
    where
        Self: Sized,
    {
        Drop {
            parser: self,
            _phantom: PhantomData,
        }
    }

    /// Repeat this parser.
    ///
    /// The number of repetitions to match can be configured by calling `min`
    /// and 'max`.
    ///
    /// Has no output by default. To output as a collection, call `collect` on
    /// it.
    fn repeat(self) -> Repeat<'a, C, Self, Output, NoRepeatOutput>
    where
        Self: Sized,
    {
        Repeat {
            parser: self,
            min: 0,
            max: None,
            _phantom: PhantomData,
        }
    }

    /// Get the span of the matched input.
    ///
    /// Has an output of form `(span, output)`.
    fn spanned(self) -> Spanned<'a, C, Self, Output>
    where
        Self: Sized,
    {
        Spanned {
            parser: self,
            _phantom: PhantomData,
        }
    }

    /// Convert the output to a slice of the matched input.
    fn to_slice(self) -> ToSlice<'a, C, Self, Output>
    where
        Self: Sized,
    {
        ToSlice {
            parser: self,
            _phantom: PhantomData,
        }
    }
}

pub struct ParseResult<'a, C: Context<'a>, Output> {
    inner: Result<Output, Option<C::Error>>,
}

impl<'a, C: Context<'a>, Output> ParseResult<'a, C, Output> {
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

impl<'a, C: Context<'a>, T> ParseResult<'a, C, T> {
    pub fn map<U>(self, f: impl Fn(T) -> U) -> ParseResult<'a, C, U> {
        ParseResult {
            inner: self.inner.map(f),
        }
    }
}

impl<'a, C: Context<'a>, Output: Default> From<()> for ParseResult<'a, C, Output> {
    fn from(_value: ()) -> Self {
        Self {
            inner: Ok(Output::default()),
        }
    }
}

impl<'a, C: Context<'a>, Output> From<Option<Output>> for ParseResult<'a, C, Output> {
    fn from(output: Option<Output>) -> Self {
        match output {
            Some(output) => Self { inner: Ok(output) },
            None => Self { inner: Err(None) },
        }
    }
}

impl<'a, C: Context<'a>, Output> From<Result<Output, C::Error>> for ParseResult<'a, C, Output> {
    fn from(output: Result<Output, C::Error>) -> Self {
        Self {
            inner: output.map_err(Some),
        }
    }
}
