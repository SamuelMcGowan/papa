use std::marker::PhantomData;

use crate::prelude::*;

pub struct Filter<C, P, Output, F>
where
    C: Context,
    P: Parser<C, Output>,
    F: Fn(&Output) -> bool,
{
    pub(crate) parser: P,
    pub(crate) filter: F,
    pub(crate) _phantom: PhantomData<*const (C, Output)>,
}

impl<C, P, Output, F> Parser<C, Output> for Filter<C, P, Output, F>
where
    C: Context,
    P: Parser<C, Output>,
    F: Fn(&Output) -> bool,
{
    fn parse(&self, context: &mut C) -> ParseResult<C, Output> {
        self.parser.parse(context).and_then(|output| {
            if (self.filter)(&output) {
                Ok(output)
            } else {
                Err(None)
            }
        })
    }
}
