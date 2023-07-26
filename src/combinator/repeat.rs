use std::marker::PhantomData;

use crate::context::slice::Slice;
use crate::context::Context;
use crate::prelude::*;

#[derive_where::derive_where(Debug, Clone; P)]
pub struct Repeat<In: Slice, Out, Error, P: Parser<In, Out, Error>, Collection: FromIterator<Out>> {
    pub(crate) parser: P,
    pub(crate) min: usize,
    pub(crate) max: Option<usize>,
    pub(crate) _phantom: PhantomData<*const (In, Out, Error, Collection)>,
}

impl<In, Out, Error, P, Collection> Repeat<In, Out, Error, P, Collection>
where
    In: Slice,
    P: Parser<In, Out, Error>,
    Collection: FromIterator<Out>,
{
    /// Set the minimum number of times to match.
    pub fn min(mut self, min: usize) -> Self {
        self.min = min;
        self
    }

    /// Set the maximum number of times to match.
    pub fn max(mut self, max: usize) -> Self {
        self.max = Some(max);
        self
    }

    /// Collect the output of this parser.
    pub fn collect<Dest>(self) -> Repeat<In, Out, Error, P, Dest>
    where
        Dest: FromIterator<Out>,
    {
        Repeat {
            parser: self.parser,
            min: self.min,
            max: self.max,
            _phantom: PhantomData,
        }
    }
}

impl<In, Out, Error, P, Collection> Parser<In, Collection, Error>
    for Repeat<In, Out, Error, P, Collection>
where
    In: Slice,
    P: Parser<In, Out, Error>,
    Collection: FromIterator<Out>,
{
    fn parse(&self, context: &mut Context<In, Error>) -> ParseResult<Collection, Error> {
        let start = context.location();
        let mut num_matches = 0;

        let parse_iter = std::iter::from_fn(|| {
            let output = self.parser.parse(context).ok()?;
            num_matches += 1;
            Some(output)
        });

        let collection = if let Some(max) = self.max {
            Collection::from_iter(parse_iter.take(max))
        } else {
            Collection::from_iter(parse_iter)
        };

        if num_matches < self.min {
            context.set_location(start);
            Err(None)
        } else {
            Ok(collection)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NoRepeatOutput;

impl<Item> FromIterator<Item> for NoRepeatOutput {
    fn from_iter<T: IntoIterator<Item = Item>>(iter: T) -> Self {
        iter.into_iter().for_each(|_| {});
        Self
    }
}
