use std::marker::PhantomData;

use crate::context::Context;
use crate::parser::Parser;

/// Map the result of this parser to another value.
pub fn map<C: Context, P: Parser<C, OA>, OA, OB, F: Fn(OA) -> OB>(
    parser: P,
    map: F,
) -> Map<C, P, OA, OB, F> {
    Map {
        parser,
        map,
        _phantom: PhantomData,
    }
}

pub struct Map<C: Context, P: Parser<C, OA>, OA, OB, F: Fn(OA) -> OB> {
    parser: P,
    map: F,
    _phantom: PhantomData<*const (C, OA, OB)>,
}

impl<C: Context, P: Parser<C, OA>, OA, OB, F: Fn(OA) -> OB> Parser<C, OB> for Map<C, P, OA, OB, F> {
    fn parse(&mut self, context: &mut C) -> OB {
        let output = self.parser.parse(context);
        (self.map)(output)
    }
}

pub struct OkOr<C: Context, P: Parser<C, Option<Output>>, Output>
where
    C::Error: Clone,
{
    pub(crate) parser: P,
    pub(crate) error: C::Error,
    pub(crate) _phantom: PhantomData<*const Output>,
}

impl<C: Context, P: Parser<C, Option<Output>>, Output> Parser<C, Result<Output, C::Error>>
    for OkOr<C, P, Output>
where
    C::Error: Clone,
{
    fn parse(&mut self, context: &mut C) -> Result<Output, C::Error> {
        self.parser.parse(context).ok_or(self.error.clone())
    }
}
