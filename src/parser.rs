use std::marker::PhantomData;

use crate::combinator::drop::Drop;
use crate::combinator::filter::Filter;
use crate::combinator::map::Map;
use crate::combinator::repeat::{NoRepeatOutput, Repeat};
use crate::combinator::spanned::Spanned;
use crate::combinator::to_slice::ToSlice;
use crate::context::slice::Slice;
use crate::context::Context;

pub trait Parser<In: Slice, Out, Error> {
    /// Run this parser.
    fn parse(&self, context: &mut Context<In, Error>) -> ParseResult<Out, Error>;

    /// Map the output of this parser to some other value.
    fn map<F, OutputB>(self, f: F) -> Map<In, Out, OutputB, Error, Self, F>
    where
        Self: Sized,
        F: Fn(Out) -> OutputB + Copy,
    {
        Map {
            parser: self,
            map: f,
            _phantom: PhantomData,
        }
    }

    /// Decide whether to accept an output.
    fn filter<F>(self, f: F) -> Filter<In, Out, Error, Self, F>
    where
        Self: Sized,
        F: Fn(&Out) -> bool,
    {
        Filter {
            parser: self,
            filter: f,
            _phantom: PhantomData,
        }
    }

    /// Convert the output of this parser to `()`.
    fn drop(self) -> Drop<In, Out, Error, Self>
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
    fn repeat(self) -> Repeat<In, Out, Error, Self, NoRepeatOutput>
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
    fn spanned(self) -> Spanned<In, Out, Error, Self>
    where
        Self: Sized,
    {
        Spanned {
            parser: self,
            _phantom: PhantomData,
        }
    }

    /// Convert the output to a slice of the matched input.
    fn to_slice(self) -> ToSlice<In, Out, Error, Self>
    where
        Self: Sized,
    {
        ToSlice {
            parser: self,
            _phantom: PhantomData,
        }
    }
}

pub type ParseResult<Out, Error> = Result<Out, Option<Error>>;
