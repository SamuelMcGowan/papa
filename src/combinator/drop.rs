use std::marker::PhantomData;

use crate::prelude::*;

pub struct Drop<C, P, Output>
where
    C: Context,
    P: Parser<C, Output>,
{
    pub(crate) parser: P,
    pub(crate) _phantom: PhantomData<*const (C, Output)>,
}

impl<C, P, Output> Parser<C, ()> for Drop<C, P, Output>
where
    C: Context,
    P: Parser<C, Output>,
{
    fn parse(&self, context: &mut C) -> ParseResult<C, ()> {
        self.parser.parse(context);
        Ok(())
    }
}
