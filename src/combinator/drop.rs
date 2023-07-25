use std::marker::PhantomData;

use crate::prelude::*;

pub struct Drop<'a, C, P, Output>
where
    C: Context,
    P: Parser<'a, C, Output>,
{
    pub(crate) parser: P,
    pub(crate) _phantom: PhantomData<&'a (C, Output)>,
}

impl<'a, C, P, Output> Parser<'a, C, ()> for Drop<'a, C, P, Output>
where
    C: Context,
    P: Parser<'a, C, Output>,
{
    fn parse(&self, context: &mut C) -> ParseResult<C, ()> {
        self.parser.parse(context);
        Ok(())
    }
}
