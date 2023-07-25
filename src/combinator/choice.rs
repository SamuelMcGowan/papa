use std::marker::PhantomData;

use crate::prelude::*;

/// A parser that tries to parse one of a tuple of parsers.
pub fn choice<'a, C: Context<'a>, P: ChoiceParsers<'a, C, Output>, Output>(
    parsers: P,
) -> Choice<'a, C, Output, P> {
    Choice {
        parsers,
        _phantom: PhantomData,
    }
}

pub struct Choice<'a, C: Context<'a>, Output, Parsers: ChoiceParsers<'a, C, Output>> {
    parsers: Parsers,
    _phantom: PhantomData<&'a (C, Output)>,
}

impl<'a, C: Context<'a>, Output, Parsers: ChoiceParsers<'a, C, Output>> Parser<'a, C, Output>
    for Choice<'a, C, Output, Parsers>
{
    fn parse(&self, context: &mut C) -> ParseResult<'a, C, Output> {
        self.parsers.parse_choice(context)
    }
}

/// A tuple of [`Parser`]s, to be passed to [`choice`].
///
/// Currently implemented for tuples of up to 8 elements.
pub trait ChoiceParsers<'a, Ctx: Context<'a>, Output> {
    #[doc(hidden)]
    fn parse_choice(&self, context: &mut Ctx) -> ParseResult<'a, Ctx, Output>;
}

macro_rules! impl_choice {
    ($($n:tt $parser:ident),*) => {
        impl<'a, Ctx, Output, $($parser,)*>
        ChoiceParsers<'a, Ctx, Output> for ($($parser,)*)
        where
            Ctx: Context<'a>,
            $($parser: Parser<'a, Ctx, Output>,)*
        {
            fn parse_choice(&self, context: &mut Ctx) -> ParseResult<'a, Ctx, Output> {
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
