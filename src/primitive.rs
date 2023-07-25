use std::marker::PhantomData;

use crate::context::Context;
use crate::parser::{ParseResult, Parser};

/// Parse any token.
pub fn any<'a, C: Context<'a>>() -> Any<'a, C> {
    Any {
        _phantom: PhantomData,
    }
}

pub struct Any<'a, C: Context<'a>> {
    _phantom: PhantomData<&'a C>,
}

impl<'a, C: Context<'a>> Parser<'a, C, C::Token> for Any<'a, C> {
    #[inline]
    fn parse(&self, context: &mut C) -> ParseResult<'a, C, C::Token> {
        context.next().into()
    }
}

/// Don't do anything, just output `()`.
pub fn nothing<'a, C: Context<'a>>() -> Nothing<'a, C> {
    Nothing {
        _phantom: PhantomData,
    }
}

pub struct Nothing<'a, C: Context<'a>> {
    _phantom: PhantomData<&'a C>,
}

impl<'a, C: Context<'a>> Parser<'a, C, ()> for Nothing<'a, C> {
    fn parse(&self, _context: &mut C) -> ParseResult<'a, C, ()> {
        ().into()
    }
}

/// Parse a token if it matches a predicate.
pub fn pred<'a, C, F>(pred: F) -> Pred<'a, C, F>
where
    C: Context<'a>,
    F: Fn(&C::Token) -> bool + Copy,
{
    Pred {
        pred,
        _phantom: PhantomData,
    }
}

pub struct Pred<'a, C, F>
where
    C: Context<'a>,
    F: Fn(&C::Token) -> bool + Copy,
{
    pred: F,
    _phantom: PhantomData<&'a C>,
}

impl<'a, C, F> Parser<'a, C, C::Token> for Pred<'a, C, F>
where
    C: Context<'a>,
    F: Fn(&C::Token) -> bool + Copy,
{
    fn parse(&self, context: &mut C) -> ParseResult<'a, C, C::Token> {
        match context.peek() {
            Some(token) if (self.pred)(token) => context.next().into(),
            _ => ParseResult::err(None),
        }
    }
}

/// Construct a parser from a function.
pub fn func<'a, C, F, Output>(f: F) -> FuncParser<'a, C, F, Output>
where
    C: Context<'a>,
    F: Fn(&mut C) -> ParseResult<'a, C, Output>,
{
    FuncParser {
        f,
        _phantom: PhantomData,
    }
}

pub struct FuncParser<'a, C, F, Output>
where
    C: Context<'a>,
    F: Fn(&mut C) -> ParseResult<'a, C, Output>,
{
    f: F,
    _phantom: PhantomData<*const (C, Output)>,
}

impl<'a, C, F, Output> Parser<'a, C, Output> for FuncParser<'a, C, F, Output>
where
    C: Context<'a>,
    F: Fn(&mut C) -> ParseResult<'a, C, Output>,
{
    fn parse(&self, context: &mut C) -> ParseResult<'a, C, Output> {
        (self.f)(context)
    }
}
