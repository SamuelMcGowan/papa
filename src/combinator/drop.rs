use std::marker::PhantomData;

use crate::prelude::*;

pub struct Drop<'a, C, P, Output>
where
    C: Context<'a>,
    P: Parser<'a, C, Output>,
{
    pub(crate) parser: P,
    pub(crate) _phantom: PhantomData<&'a (C, Output)>,
}

impl<'a, C, P, Output> Parser<'a, C, ()> for Drop<'a, C, P, Output>
where
    C: Context<'a>,
    P: Parser<'a, C, Output>,
{
    fn parse(&self, context: &mut C) -> ParseResult<'a, C, ()> {
        self.parser.parse(context);
        ().into()
    }
}
