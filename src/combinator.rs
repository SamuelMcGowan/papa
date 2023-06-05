use std::marker::PhantomData;

use crate::context::Context;
use crate::parser::{Parser, ParserFallible, ParserOptional};

pub struct Map<C, P, OA, OB, F>
where
    C: Context,
    P: Parser<C, OA>,
    F: Fn(OA) -> OB,
{
    pub(crate) parser: P,
    pub(crate) map: F,
    pub(crate) _phantom: PhantomData<*const (C, OA, OB)>,
}

impl<C, P, OA, OB, F> Parser<C, OB> for Map<C, P, OA, OB, F>
where
    C: Context,
    P: Parser<C, OA>,
    F: Fn(OA) -> OB,
{
    fn parse(&mut self, context: &mut C) -> OB {
        let output = self.parser.parse(context);
        (self.map)(output)
    }
}

pub struct OkOr<C, P, Success>
where
    C::Error: Clone,
    C: Context,
    P: ParserOptional<C, Success>,
{
    pub(crate) parser: P,
    pub(crate) error: C::Error,
    pub(crate) _phantom: PhantomData<*const Success>,
}

impl<C, P, Success> Parser<C, Result<Success, C::Error>> for OkOr<C, P, Success>
where
    C::Error: Clone,
    C: Context,
    P: ParserOptional<C, Success>,
{
    fn parse(&mut self, context: &mut C) -> Result<Success, C::Error> {
        self.parser.parse(context).ok_or(self.error.clone())
    }
}

pub struct Recover<C, P, R, Success, D>
where
    C: Context,
    P: ParserFallible<C, Success>,
    R: Parser<C, ()>,
    D: Fn() -> Success,
{
    pub(crate) parser: P,
    pub(crate) recover: R,
    pub(crate) default: D,
    pub(crate) _phantom: PhantomData<*const (C, Success)>,
}

impl<C, P, R, Success, D> Parser<C, Success> for Recover<C, P, R, Success, D>
where
    C: Context,
    P: ParserFallible<C, Success>,
    R: Parser<C, ()>,
    D: Fn() -> Success,
{
    fn parse(&mut self, context: &mut C) -> Success {
        self.parser.parse(context).unwrap_or_else(|error| {
            context.report(error);
            self.recover.parse(context);
            (self.default)()
        })
    }
}

pub struct Drop<C, P, Output>
where
    C: Context,
    P: Parser<C, Output>,
{
    pub(crate) parser: P,
    pub(crate) _phantom: PhantomData<*const (C, Output)>,
}

impl<C, P, Output> Parser<C, ()> for Drop<C, P, Output>
where
    C: Context,
    P: Parser<C, Output>,
{
    fn parse(&mut self, context: &mut C) {
        self.parser.parse(context);
    }
}

/// A parser that tries to parse one of a tuple of parsers.
pub fn choice<C: Context, P: ParserTuple<C, Output>, Output>(parsers: P) -> Choice<C, Output, P> {
    Choice {
        parsers,
        _phantom: PhantomData,
    }
}

pub struct Choice<C: Context, Output, Parsers: ParserTuple<C, Output>> {
    parsers: Parsers,
    _phantom: PhantomData<*const (C, Output)>,
}

impl<C: Context, Output, Parsers: ParserTuple<C, Output>> Parser<C, Option<Output>>
    for Choice<C, Output, Parsers>
{
    fn parse(&mut self, context: &mut C) -> Option<Output> {
        self.parsers.parse_choice(context)
    }
}

/// A tuple of [`ParserOptional`]s, to be passed to [`choice`].
///
/// Currently implemented for tuples of up to 8 elements.
pub trait ParserTuple<Ctx, Output> {
    #[doc(hidden)]
    fn parse_choice(&mut self, context: &mut Ctx) -> Option<Output>;
}

macro_rules! impl_choice {
    ($($n:tt $parser:ident),*) => {
        impl<Ctx, Output, $($parser,)*>
        ParserTuple<Ctx, Output> for ($($parser,)*)
        where
            Ctx: Context,
            $($parser: ParserOptional<Ctx, Output>,)*
        {
            fn parse_choice(&mut self, context: &mut Ctx) -> Option<Output> {
                $(
                    let start = context.location();
                    if let Some(output) = self.$n.parse(context) {
                        return Some(output);
                    }
                    context.set_location(start);
                )*

                None
            }
        }
    };
}

impl_choice! { 0 A }
impl_choice! { 0 A, 1 B }
impl_choice! { 0 A, 1 B, 2 C }
impl_choice! { 0 A, 1 B, 2 C, 3 D }
impl_choice! { 0 A, 1 B, 2 C, 3 D, 4 E }
impl_choice! { 0 A, 1 B, 2 C, 3 D, 4 E, 5 F }
impl_choice! { 0 A, 1 B, 2 C, 3 D, 4 E, 5 F, 6 G }
impl_choice! { 0 A, 1 B, 2 C, 3 D, 4 E, 5 F, 6 G, 7 H }
