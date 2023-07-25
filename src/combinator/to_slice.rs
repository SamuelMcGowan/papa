use std::marker::PhantomData;

use crate::prelude::*;

pub struct ToSlice<'a, C: Context<'a>, P: Parser<'a, C, Output>, Output> {
    pub(crate) parser: P,
    pub(crate) _phantom: PhantomData<&'a (C, Output)>,
}

impl<'a, C: Context<'a>, P: Parser<'a, C, Output>, Output> Parser<'a, C, C::Slice>
    for ToSlice<'a, C, P, Output>
{
    fn parse(&self, context: &mut C) -> ParseResult<'a, C, C::Slice> {
        let start = context.location();
        let output = self.parser.parse(context);
        output.map(|_| context.slice(start, context.location()))
    }
}
