use std::marker::PhantomData;

use crate::prelude::*;

pub struct Spanned<'a, C: Context<'a>, P: Parser<'a, C, Output>, Output> {
    pub(crate) parser: P,
    pub(crate) _phantom: PhantomData<&'a (C, Output)>,
}

impl<'a, C: Context<'a>, P: Parser<'a, C, Output>, Output>
    Parser<'a, C, (Span<C::Location>, Output)> for Spanned<'a, C, P, Output>
{
    fn parse(&self, context: &mut C) -> ParseResult<'a, C, (Span<C::Location>, Output)> {
        let start = context.location();
        let output = self.parser.parse(context);

        output.map(|output| {
            let span = Span::new(start, context.location());
            (span, output)
        })
    }
}
