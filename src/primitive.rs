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

#[derive_where::derive_where(Debug, Clone, Copy)]
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

#[derive_where::derive_where(Debug, Clone, Copy)]
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

#[derive_where::derive_where(Debug, Clone, Copy; F)]
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
            Some(token) if (self.pred)(token) => Ok(token),
            _ => {
                context.set_location(start);
                Err(None)
            }
        }
    }
}

/// Match a single token.
pub fn just<In: Slice, Error>(token: In::Token) -> Just<In, Error>
where
    In::Token: Eq,
{
    Just {
        token,
        _phantom: PhantomData,
    }
}

#[derive_where::derive_where(Debug, Clone, Copy; In::Token)]
pub struct Just<In: Slice, Error>
where
    In::Token: Eq,
{
    token: In::Token,
    _phantom: PhantomData<*const Error>,
}

impl<In: Slice, Error> Parser<In, In::Token, Error> for Just<In, Error>
where
    In::Token: Eq,
{
    fn parse(&self, context: &mut Context<In, Error>) -> ParseResult<In::Token, Error> {
        let start = context.location();

        match context.next() {
            Some(token) if token == self.token => Ok(token),
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
        func: f,
        _phantom: PhantomData,
    }
}

#[derive_where::derive_where(Debug, Clone; F)]
pub struct FuncParser<In, Out, Error, F>
where
    In: Slice,
    F: Fn(&mut Context<In, Error>) -> ParseResult<Out, Error>,
{
    func: F,
    _phantom: PhantomData<*const (In, Out, Error)>,
}

impl<In, Out, Error, F> Parser<In, Out, Error> for FuncParser<In, Out, Error, F>
where
    In: Slice,
    F: Fn(&mut Context<In, Error>) -> ParseResult<Out, Error>,
{
    fn parse(&self, context: &mut Context<In, Error>) -> ParseResult<Out, Error> {
        (self.func)(context)
    }
}
