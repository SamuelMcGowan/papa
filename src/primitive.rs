use std::marker::PhantomData;

use crate::context::Context;
use crate::parser::Parser;

/// Parse any token.
pub fn any<C: Context>() -> Any<C> {
    Any {
        _phantom: PhantomData,
    }
}

pub struct Any<C: Context> {
    _phantom: PhantomData<*const C>,
}

impl<C: Context> Parser<C, Option<C::Token>> for Any<C> {
    #[inline]
    fn parse(&mut self, context: &mut C) -> Option<C::Token> {
        context.next()
    }
}

/// Parse a token if it matches a predicate.
pub fn pred<C: Context, F: Fn(&C::Token) -> bool + Copy>(pred: F) -> Pred<C, F> {
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

impl<C: Context, F: Fn(&C::Token) -> bool + Copy> Parser<C, Option<C::Token>> for Pred<C, F> {
    fn parse(&mut self, context: &mut C) -> Option<C::Token> {
        context.eat_if(self.pred)
    }
}
