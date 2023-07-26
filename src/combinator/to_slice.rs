use std::marker::PhantomData;

use crate::context::slice::Slice;
use crate::context::Context;
use crate::prelude::*;

#[derive_where::derive_where(Debug, Clone; P)]
pub struct ToSlice<In: Slice, Out, Error, P: Parser<In, Out, Error>> {
    pub(crate) parser: P,
    pub(crate) _phantom: PhantomData<*const (In, Out, Error)>,
}

impl<In: Slice, Out, Error, P: Parser<In, Out, Error>> Parser<In, In, Error>
    for ToSlice<In, Out, Error, P>
{
    fn parse(&self, context: &mut Context<In, Error>) -> ParseResult<In, Error> {
        let start = context.location();
        let output = self.parser.parse(context);
        output.map(|_| {
            let end = context.location();
            context
                .slice_all()
                .slice(start, end)
                .expect("couldn't get slice")
        })
    }
}
