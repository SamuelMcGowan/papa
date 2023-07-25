use std::marker::PhantomData;

use crate::prelude::*;

pub struct Filter<'a, C, P, Output, F>
where
    C: Context<'a>,
    P: Parser<'a, C, Output>,
    F: Fn(&Output) -> bool,
{
    pub(crate) parser: P,
    pub(crate) filter: F,
    pub(crate) _phantom: PhantomData<&'a (C, Output)>,
}

impl<'a, C, P, Output, F> Parser<'a, C, Output> for Filter<'a, C, P, Output, F>
where
    C: Context<'a>,
    P: Parser<'a, C, Output>,
    F: Fn(&Output) -> bool,
{
    fn parse(&self, context: &mut C) -> ParseResult<'a, C, Output> {
        self.parser.parse(context).and_then(|output| {
            if (self.filter)(&output) {
                Ok(output)
            } else {
                Err(None)
            }
        })
    }
}
