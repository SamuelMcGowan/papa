use std::marker::PhantomData;

use crate::context::slice::Slice;
use crate::context::Context;
use crate::prelude::*;

pub struct Drop<In, Out, Error, P>
where
    In: Slice,
    P: Parser<In, Out, Error>,
{
    pub(crate) parser: P,
    pub(crate) _phantom: PhantomData<*const (In, Out, Error)>,
}

impl<In, Out, Error, P> Parser<In, (), Error> for Drop<In, Out, Error, P>
where
    In: Slice,
    P: Parser<In, Out, Error>,
{
    fn parse(&self, context: &mut Context<In, Error>) -> ParseResult<(), Error> {
        let _ = self.parser.parse(context);
        Ok(())
    }
}
