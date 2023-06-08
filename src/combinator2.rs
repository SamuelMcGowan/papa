use std::marker::PhantomData;

use crate::context::Context;
use crate::parser2::{ParseResult, Parser};

pub struct Map<C, P, OA, OB, F>
where
    C: Context,
    P: Parser<Context = C, Output = OA>,
    F: Fn(OA) -> OB + Copy,
{
    pub(crate) parser: P,
    pub(crate) map: F,
    pub(crate) _phantom: PhantomData<*const (C, OA, OB)>,
}

impl<C, P, OA, OB, F> Parser for Map<C, P, OA, OB, F>
where
    C: Context,
    P: Parser<Context = C, Output = OA>,
    F: Fn(OA) -> OB + Copy,
{
    type Context = C;
    type Output = OB;

    fn parse(&self, context: &mut Self::Context) -> ParseResult<C, OB> {
        self.parser.parse(context).map(self.map)
    }
}
