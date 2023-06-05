use std::marker::PhantomData;

use crate::context::Context;
use crate::parser::{Parser, ParserFallible, ParserOptional};

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

pub struct OkOr<C: Context, P: ParserOptional<C, Success>, Success>
where
    C::Error: Clone,
{
    pub(crate) parser: P,
    pub(crate) error: C::Error,
    pub(crate) _phantom: PhantomData<*const Success>,
}

impl<C: Context, P: ParserOptional<C, Success>, Success> Parser<C, Result<Success, C::Error>>
    for OkOr<C, P, Success>
where
    C::Error: Clone,
{
    fn parse(&mut self, context: &mut C) -> Result<Success, C::Error> {
        self.parser.parse(context).ok_or(self.error.clone())
    }
}

pub struct Recover<
    C: Context,
    P: ParserFallible<C, Success>,
    R: Parser<C, ()>,
    Success,
    D: Fn() -> Success,
> {
    pub(crate) parser: P,
    pub(crate) recover: R,
    pub(crate) default: D,
    pub(crate) _phantom: PhantomData<*const (C, Success)>,
}

impl<C: Context, P: ParserFallible<C, Success>, R: Parser<C, ()>, Success, D: Fn() -> Success>
    Parser<C, Success> for Recover<C, P, R, Success, D>
{
    fn parse(&mut self, context: &mut C) -> Success {
        self.parser.parse(context).unwrap_or_else(|error| {
            context.report(error);
            self.recover.parse(context);
            (self.default)()
        })
    }
}

pub struct Drop<C: Context, P: Parser<C, Output>, Output> {
    pub(crate) parser: P,
    pub(crate) _phantom: PhantomData<*const (C, Output)>,
}

impl<C: Context, P: Parser<C, Output>, Output> Parser<C, ()> for Drop<C, P, Output> {
    fn parse(&mut self, context: &mut C) {
        self.parser.parse(context);
    }
}
