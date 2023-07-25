use std::marker::PhantomData;

use crate::context::slice::Slice;
use crate::context::Context;
use crate::prelude::*;

pub struct Map<In, OA, OB, Error, P, F>
where
    In: Slice,
    P: Parser<In, OA, Error>,
    F: Fn(OA) -> OB + Copy,
{
    pub(crate) parser: P,
    pub(crate) map: F,
    pub(crate) _phantom: PhantomData<*const (In, OA, OB, Error)>,
}

impl<In, OA, OB, Error, P, F> Parser<In, OB, Error> for Map<In, OA, OB, Error, P, F>
where
    In: Slice,
    P: Parser<In, OA, Error>,
    F: Fn(OA) -> OB + Copy,
{
    fn parse(&self, context: &mut Context<In, Error>) -> ParseResult<OB, Error> {
        self.parser.parse(context).map(self.map)
    }
}
