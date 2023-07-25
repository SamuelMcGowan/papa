use std::marker::PhantomData;

use paste::paste;

use crate::prelude::*;

pub fn chain<'a, C: Context, P: ChainParsers<'a, C, Output>, Output>(
    parsers: P,
) -> Chain<'a, C, Output, P> {
    Chain {
        parsers,
        _phantom: PhantomData,
    }
}

pub struct Chain<'a, C: Context, Output, Parsers: ChainParsers<'a, C, Output>> {
    parsers: Parsers,
    _phantom: PhantomData<&'a (C, Output)>,
}

impl<'a, C: Context, Output, Parsers: ChainParsers<'a, C, Output>> Parser<'a, C, Output>
    for Chain<'a, C, Output, Parsers>
{
    fn parse(&self, context: &mut C) -> ParseResult<C, Output> {
        self.parsers.parse_chain(context)
    }
}

/// A tuple of [`Parser`]s, to be passed to [`chain`].
///
/// Currently implemented for tuples of up to 8 elements.
pub trait ChainParsers<'a, Ctx: Context, Output> {
    #[doc(hidden)]
    fn parse_chain(&self, context: &mut Ctx) -> ParseResult<Ctx, Output>;
}

macro_rules! impl_chain {
    ($($n:tt $parser:ident),*) => { paste!{
        impl<'a, Ctx, $($parser, [<$parser Out>],)*>
        ChainParsers<'a, Ctx, ($([<$parser Out>],)*)> for ($($parser,)*)
        where
            Ctx: Context,
            $($parser: Parser<'a, Ctx, [<$parser Out>]>,)*
        {
            fn parse_chain(&self, context: &mut Ctx) -> ParseResult<Ctx,($([<$parser Out>],)*)> {
                let start = context.location();

                $(
                    let [<$parser:lower _out>] = match self.$n.parse(context) {
                         Ok(output) => output,
                         Err(err) => {
                             context.set_location(start);
                             return Err(err);
                         }
                    };
                )*

                Ok(($([<$parser:lower _out>],)*))
            }
        }
    } };
}

impl_chain! { 0 A }
impl_chain! { 0 A, 1 B }
impl_chain! { 0 A, 1 B, 2 C }
impl_chain! { 0 A, 1 B, 2 C, 3 D }
impl_chain! { 0 A, 1 B, 2 C, 3 D, 4 E }
impl_chain! { 0 A, 1 B, 2 C, 3 D, 4 E, 5 F }
impl_chain! { 0 A, 1 B, 2 C, 3 D, 4 E, 5 F, 6 G }
impl_chain! { 0 A, 1 B, 2 C, 3 D, 4 E, 5 F, 6 G, 7 H }
