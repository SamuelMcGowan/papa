use std::marker::PhantomData;

use crate::context::slice::Slice;
use crate::context::Context;
use crate::parser::{ParseResult, Parser};

/// Parse any token.
pub fn any<In: Slice, Error>() -> Any<In, Error> {
    Any {
        _phantom: PhantomData,
    }
}

pub struct Any<In: Slice, Error> {
    _phantom: PhantomData<*const (In, Error)>,
}

impl<In: Slice, Error> Parser<In, In::Token, Error> for Any<In, Error> {
    #[inline]
    fn parse(&self, context: &mut Context<In, Error>) -> ParseResult<In::Token, Error> {
        context.next().ok_or(None)
    }
}

/// Don't do anything, just output `()`.
pub fn nothing<In: Slice, Error>() -> Nothing<In, Error> {
    Nothing {
        _phantom: PhantomData,
    }
}

pub struct Nothing<In: Slice, Error> {
    _phantom: PhantomData<*const (In, Error)>,
}

impl<In: Slice, Error> Parser<In, (), Error> for Nothing<In, Error> {
    fn parse(&self, _context: &mut Context<In, Error>) -> ParseResult<(), Error> {
        Ok(())
    }
}

/// Parse a token if it matches a predicate.
pub fn pred<In: Slice, Error, F>(pred: F) -> Pred<In, Error, F>
where
    F: Fn(In::Token) -> bool + Copy,
{
    Pred {
        pred,
        _phantom: PhantomData,
    }
}

pub struct Pred<In, Error, F>
where
    In: Slice,
    F: Fn(In::Token) -> bool + Copy,
{
    pred: F,
    _phantom: PhantomData<*const (In, Error)>,
}

impl<In, Error, F> Parser<In, In::Token, Error> for Pred<In, Error, F>
where
    In: Slice,
    F: Fn(In::Token) -> bool + Copy,
{
    fn parse(&self, context: &mut Context<In, Error>) -> ParseResult<In::Token, Error> {
        let start = context.location();

        match context.next() {
            Some(token) if (self.pred)(token) => context.next().ok_or(None),
            _ => {
                context.set_location(start);
                Err(None)
            }
        }
    }
}

/// Construct a parser from a function.
pub fn func<In, Out, Error, F>(f: F) -> FuncParser<In, Out, Error, F>
where
    In: Slice,
    F: Fn(&mut Context<In, Error>) -> ParseResult<Out, Error>,
{
    FuncParser {
        f,
        _phantom: PhantomData,
    }
}

pub struct FuncParser<In, Out, Error, F>
where
    In: Slice,
    F: Fn(&mut Context<In, Error>) -> ParseResult<Out, Error>,
{
    f: F,
    _phantom: PhantomData<*const (In, Out, Error)>,
}

impl<In, Out, Error, F> Parser<In, Out, Error> for FuncParser<In, Out, Error, F>
where
    In: Slice,
    F: Fn(&mut Context<In, Error>) -> ParseResult<Out, Error>,
{
    fn parse(&self, context: &mut Context<In, Error>) -> ParseResult<Out, Error> {
        (self.f)(context)
    }
}
