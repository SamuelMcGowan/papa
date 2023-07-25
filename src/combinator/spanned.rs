use std::marker::PhantomData;

use crate::prelude::*;

pub struct Spanned<C: Context, P: Parser<C, Output>, Output> {
    pub(crate) parser: P,
    pub(crate) _phantom: PhantomData<*const (C, Output)>,
}

impl<C: Context, P: Parser<C, Output>, Output> Parser<C, (Span<C::Location>, Output)>
    for Spanned<C, P, Output>
{
    fn parse(&self, context: &mut C) -> ParseResult<C, (Span<C::Location>, Output)> {
        let start = context.location();
        let output = self.parser.parse(context);

        output.map(|output| {
            let span = Span::new(start, context.location());
            (span, output)
        })
    }
}
