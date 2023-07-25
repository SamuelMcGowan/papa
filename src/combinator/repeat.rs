use std::marker::PhantomData;

use crate::prelude::*;

pub struct Repeat<
    'a,
    C: Context,
    P: Parser<'a, C, Output>,
    Output,
    Collection: FromIterator<Output>,
> {
    pub(crate) parser: P,
    pub(crate) min: usize,
    pub(crate) max: Option<usize>,
    pub(crate) _phantom: PhantomData<&'a (C, Output, Collection)>,
}

impl<'a, C: Context, P: Parser<'a, C, Output>, Output, Collection: FromIterator<Output>>
    Repeat<'a, C, P, Output, Collection>
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
    pub fn collect<Dest>(self) -> Repeat<'a, C, P, Output, Dest>
    where
        Dest: FromIterator<Output>,
    {
        Repeat {
            parser: self.parser,
            min: self.min,
            max: self.max,
            _phantom: PhantomData,
        }
    }
}

impl<'a, C: Context, P: Parser<'a, C, Output>, Output, Collection> Parser<'a, C, Collection>
    for Repeat<'a, C, P, Output, Collection>
where
    Collection: FromIterator<Output>,
{
    fn parse(&self, context: &mut C) -> ParseResult<C, Collection> {
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

pub struct NoRepeatOutput;

impl<Item> FromIterator<Item> for NoRepeatOutput {
    fn from_iter<T: IntoIterator<Item = Item>>(iter: T) -> Self {
        iter.into_iter().for_each(|_| {});
        Self
    }
}
