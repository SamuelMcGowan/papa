use std::marker::PhantomData;

use crate::context::Context;
use crate::parser::{ParseResult, Parser};

/// Parse any token.
pub fn any<C: Context>() -> Any<C> {
    Any {
        _phantom: PhantomData,
    }
}

pub struct Any<C: Context> {
    _phantom: PhantomData<*const C>,
}

impl<C: Context> Parser<C, C::Token> for Any<C> {
    #[inline]
    fn parse(&self, context: &mut C) -> ParseResult<C, C::Token> {
        context.next().ok_or(None)
    }
}

/// Don't do anything, just output `()`.
pub fn nothing<C: Context>() -> Nothing<C> {
    Nothing {
        _phantom: PhantomData,
    }
}

pub struct Nothing<C: Context> {
    _phantom: PhantomData<*const C>,
}

impl<C: Context> Parser<C, ()> for Nothing<C> {
    fn parse(&self, _context: &mut C) -> ParseResult<C, ()> {
        Ok(())
    }
}

/// Parse a token if it matches a predicate.
pub fn pred<C, F>(pred: F) -> Pred<C, F>
where
    C: Context,
    F: Fn(C::Token) -> bool + Copy,
{
    Pred {
        pred,
        _phantom: PhantomData,
    }
}

pub struct Pred<C, F>
where
    C: Context,
    F: Fn(C::Token) -> bool + Copy,
{
    pred: F,
    _phantom: PhantomData<*const C>,
}

impl<C, F> Parser<C, C::Token> for Pred<C, F>
where
    C: Context,
    F: Fn(C::Token) -> bool + Copy,
{
    fn parse(&self, context: &mut C) -> ParseResult<C, C::Token> {
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
pub fn func<C, F, Output>(f: F) -> FuncParser<C, F, Output>
where
    C: Context,
    F: Fn(&mut C) -> ParseResult<C, Output>,
{
    FuncParser {
        f,
        _phantom: PhantomData,
    }
}

pub struct FuncParser<C, F, Output>
where
    C: Context,
    F: Fn(&mut C) -> ParseResult<C, Output>,
{
    f: F,
    _phantom: PhantomData<*const (C, Output)>,
}

impl<C, F, Output> Parser<C, Output> for FuncParser<C, F, Output>
where
    C: Context,
    F: Fn(&mut C) -> ParseResult<C, Output>,
{
    fn parse(&self, context: &mut C) -> ParseResult<C, Output> {
        (self.f)(context)
    }
}
