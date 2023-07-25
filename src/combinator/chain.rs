use std::marker::PhantomData;

use paste::paste;

use crate::context::slice::Slice;
use crate::context::Context;
use crate::prelude::*;

pub fn chain<In: Slice, Out, Error, P: ChainParsers<In, Out, Error>>(
    parsers: P,
) -> Chain<In, Out, Error, P> {
    Chain {
        parsers,
        _phantom: PhantomData,
    }
}

pub struct Chain<In: Slice, Out, Error, Parsers: ChainParsers<In, Out, Error>> {
    parsers: Parsers,
    _phantom: PhantomData<*const (In, Out, Error)>,
}

impl<In: Slice, Out, Error, Parsers: ChainParsers<In, Out, Error>> Parser<In, Out, Error>
    for Chain<In, Out, Error, Parsers>
{
    fn parse(&self, context: &mut Context<In, Error>) -> ParseResult<Out, Error> {
        self.parsers.parse_chain(context)
    }
}

/// A tuple of [`Parser`]s, to be passed to [`chain`].
///
/// Currently implemented for tuples of up to 8 elements.
pub trait ChainParsers<In: Slice, Out, Error> {
    #[doc(hidden)]
    fn parse_chain(&self, context: &mut Context<In, Error>) -> ParseResult<Out, Error>;
}

macro_rules! impl_chain {
    ($($n:tt $parser:ident),*) => { paste!{
        impl<In, $($parser, [<$parser Out>],)* Error>
        ChainParsers<In, ($([<$parser Out>],)*), Error> for ($($parser,)*)
        where
            In: Slice,
            $($parser: Parser<In, [<$parser Out>], Error>,)*
        {
            fn parse_chain(&self, context: &mut Context<In, Error>) -> ParseResult<($([<$parser Out>],)*), Error> {
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
