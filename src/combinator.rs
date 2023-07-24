use std::marker::PhantomData;

use crate::context::Context;
use crate::parser::{ParseResult, Parser};

pub struct Map<C, P, OA, OB, F>
where
    C: Context,
    P: Parser<C, OA>,
    F: Fn(OA) -> OB + Copy,
{
    pub(crate) parser: P,
    pub(crate) map: F,
    pub(crate) _phantom: PhantomData<*const (C, OA, OB)>,
}

impl<C, P, OA, OB, F> Parser<C, OB> for Map<C, P, OA, OB, F>
where
    C: Context,
    P: Parser<C, OA>,
    F: Fn(OA) -> OB + Copy,
{
    fn parse(&self, context: &mut C) -> ParseResult<C, OB> {
        self.parser.parse(context).map(self.map)
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
    fn parse(&self, context: &mut C) -> ParseResult<C, ()> {
        self.parser.parse(context);
        ().into()
    }
}

/// A parser that tries to parse one of a tuple of parsers.
pub fn choice<C: Context, P: ChoiceParsers<C, Output>, Output>(parsers: P) -> Choice<C, Output, P> {
    Choice {
        parsers,
        _phantom: PhantomData,
    }
}

pub struct Choice<C: Context, Output, Parsers: ChoiceParsers<C, Output>> {
    parsers: Parsers,
    _phantom: PhantomData<*const (C, Output)>,
}

impl<C: Context, Output, Parsers: ChoiceParsers<C, Output>> Parser<C, Output>
    for Choice<C, Output, Parsers>
{
    fn parse(&self, context: &mut C) -> ParseResult<C, Output> {
        self.parsers.parse_choice(context)
    }
}

/// A tuple of [`ParserOptional`]s, to be passed to [`choice`].
///
/// Currently implemented for tuples of up to 8 elements.
pub trait ChoiceParsers<Ctx: Context, Output> {
    #[doc(hidden)]
    fn parse_choice(&self, context: &mut Ctx) -> ParseResult<Ctx, Output>;
}

macro_rules! impl_choice {
    ($($n:tt $parser:ident),*) => {
        impl<Ctx, Output, $($parser,)*>
        ChoiceParsers<Ctx, Output> for ($($parser,)*)
        where
            Ctx: Context,
            $($parser: Parser<Ctx, Output>,)*
        {
            fn parse_choice(&self, context: &mut Ctx) -> ParseResult<Ctx, Output> {
                $(
                    let start = context.location();
                    let result = self.$n.parse(context);
                    if result.is_ok() {
                        return result;
                    }
                    context.set_location(start);
                )*

                ParseResult::err(None)
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
