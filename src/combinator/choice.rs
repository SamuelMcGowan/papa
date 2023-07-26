use std::marker::PhantomData;

use crate::context::slice::Slice;
use crate::context::Context;
use crate::prelude::*;

/// A parser that tries to parse one of a tuple of parsers.
pub fn choice<In: Slice, Out, Error, P: ChoiceParsers<In, Out, Error>>(
    parsers: P,
) -> Choice<In, Out, Error, P> {
    Choice {
        parsers,
        _phantom: PhantomData,
    }
}

#[derive_where::derive_where(Debug, Clone; Parsers)]
pub struct Choice<In: Slice, Out, Error, Parsers: ChoiceParsers<In, Out, Error>> {
    parsers: Parsers,
    _phantom: PhantomData<*const (In, Out, Error)>,
}

impl<In: Slice, Out, Error, Parsers: ChoiceParsers<In, Out, Error>> Parser<In, Out, Error>
    for Choice<In, Out, Error, Parsers>
{
    fn parse(&self, context: &mut Context<In, Error>) -> ParseResult<Out, Error> {
        self.parsers.parse_choice(context)
    }
}

/// A tuple of [`Parser`]s, to be passed to [`choice`].
///
/// Currently implemented for tuples of up to 8 elements.
pub trait ChoiceParsers<In: Slice, Out, Error> {
    #[doc(hidden)]
    fn parse_choice(&self, context: &mut Context<In, Error>) -> ParseResult<Out, Error>;
}

macro_rules! impl_choice {
    ($($n:tt $parser:ident),*) => {
        impl<In, Out, Error, $($parser,)*>
        ChoiceParsers<In, Out, Error> for ($($parser,)*)
        where
            In: Slice,
            $($parser: Parser<In, Out, Error>,)*
        {
            fn parse_choice(&self, context: &mut Context<In, Error>) -> ParseResult<Out, Error> {
                $(
                    let start = context.location();
                    let result = self.$n.parse(context);
                    if result.is_ok() {
                        return result;
                    }
                    context.set_location(start);
                )*

                Err(None)
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
