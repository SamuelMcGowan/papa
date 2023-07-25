use std::marker::PhantomData;

use crate::prelude::*;

pub struct ToSlice<C: Context, P: Parser<C, Output>, Output> {
    pub(crate) parser: P,
    pub(crate) _phantom: PhantomData<*const (C, Output)>,
}

impl<C: Context, P: Parser<C, Output>, Output> Parser<C, C::Slice> for ToSlice<C, P, Output> {
    fn parse(&self, context: &mut C) -> ParseResult<C, C::Slice> {
        let start = context.location();
        let output = self.parser.parse(context);
        output.map(|_| context.slice(start, context.location()))
    }
}
