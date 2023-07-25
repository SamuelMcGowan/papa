use std::marker::PhantomData;

use crate::context::slice::Slice;
use crate::context::Context;
use crate::prelude::*;

pub struct Spanned<In: Slice, Out, Error, P: Parser<In, Out, Error>> {
    pub(crate) parser: P,
    pub(crate) _phantom: PhantomData<*const (In, Out, Error)>,
}

impl<In: Slice, Out, Error, P: Parser<In, Out, Error>> Parser<In, (Span<In::Location>, Out), Error>
    for Spanned<In, Out, Error, P>
{
    fn parse(
        &self,
        context: &mut Context<In, Error>,
    ) -> ParseResult<(Span<In::Location>, Out), Error> {
        let start = context.location();
        let output = self.parser.parse(context);

        output.map(|output| {
            let span = Span::new(start, context.location());
            (span, output)
        })
    }
}
