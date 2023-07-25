use std::marker::PhantomData;

use crate::context::slice::Slice;
use crate::context::Context;
use crate::prelude::*;

pub struct Filter<In, Out, Error, P, F>
where
    In: Slice,
    P: Parser<In, Out, Error>,
    F: Fn(&Out) -> bool,
{
    pub(crate) parser: P,
    pub(crate) filter: F,
    pub(crate) _phantom: PhantomData<*const (In, Out, Error)>,
}

impl<In, Out, Error, P, F> Parser<In, Out, Error> for Filter<In, Out, Error, P, F>
where
    In: Slice,
    P: Parser<In, Out, Error>,
    F: Fn(&Out) -> bool,
{
    fn parse(&self, context: &mut Context<In, Error>) -> ParseResult<Out, Error> {
        self.parser.parse(context).and_then(|output| {
            if (self.filter)(&output) {
                Ok(output)
            } else {
                Err(None)
            }
        })
    }
}
