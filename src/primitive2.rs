use std::marker::PhantomData;

use crate::context::Context;
use crate::parser2::{ParseResult, Parser};

/// Parse any token.
pub fn any<C: Context>() -> Any<C> {
    Any {
        _phantom: PhantomData,
    }
}

pub struct Any<C: Context> {
    _phantom: PhantomData<*const C>,
}

impl<C: Context> Parser for Any<C> {
    type Context = C;

    type Result = Option<C::Token>;
    type Output = C::Token;

    #[inline]
    fn parse(&self, context: &mut Self::Context) -> Option<Self::Output> {
        context.next()
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

impl<C: Context> Parser for Nothing<C> {
    type Context = C;

    type Result = ();
    type Output = ();

    fn parse(&self, _context: &mut Self::Context) -> Self::Result {}
}

/// Parse a token if it matches a predicate.
pub fn pred<C, F>(pred: F) -> Pred<C, F>
where
    C: Context,
    F: Fn(&C::Token) -> bool + Copy,
{
    Pred {
        pred,
        _phantom: PhantomData,
    }
}

pub struct Pred<C, F>
where
    C: Context,
    F: Fn(&C::Token) -> bool + Copy,
{
    pred: F,
    _phantom: PhantomData<*const C>,
}

impl<C, F> Parser for Pred<C, F>
where
    C: Context,
    F: Fn(&C::Token) -> bool + Copy,
{
    type Context = C;

    type Result = Option<C::Token>;
    type Output = C::Token;

    fn parse(&self, context: &mut Self::Context) -> Self::Result {
        context.eat_if(self.pred)
    }
}

/// Construct a parser from a function.
pub fn func<C, F, Result, Output>(f: F) -> FuncParser<C, F, Result, Output>
where
    C: Context,
    F: Fn(&mut C) -> Result,
    Result: ParseResult<C, Output>,
{
    FuncParser {
        f,
        _phantom: PhantomData,
    }
}

pub struct FuncParser<C, F, Result, Output>
where
    C: Context,
    F: Fn(&mut C) -> Result,
    Result: ParseResult<C, Output>,
{
    f: F,
    _phantom: PhantomData<*const (C, Result, Output)>,
}

impl<C, F, Result, Output> Parser for FuncParser<C, F, Result, Output>
where
    C: Context,
    F: Fn(&mut C) -> Result,
    Result: ParseResult<C, Output>,
{
    type Context = C;

    type Result = Result;
    type Output = Output;

    fn parse(&self, context: &mut Self::Context) -> Self::Result {
        (self.f)(context)
    }
}
