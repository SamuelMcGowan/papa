use std::marker::PhantomData;

use crate::prelude::*;

pub struct Map<'a, C, P, OA, OB, F>
where
    C: Context<'a>,
    P: Parser<'a, C, OA>,
    F: Fn(OA) -> OB + Copy,
{
    pub(crate) parser: P,
    pub(crate) map: F,
    pub(crate) _phantom: PhantomData<&'a (C, OA, OB)>,
}

impl<'a, C, P, OA, OB, F> Parser<'a, C, OB> for Map<'a, C, P, OA, OB, F>
where
    C: Context<'a>,
    P: Parser<'a, C, OA>,
    F: Fn(OA) -> OB + Copy,
{
    fn parse(&self, context: &mut C) -> ParseResult<'a, C, OB> {
        self.parser.parse(context).map(self.map)
    }
}
