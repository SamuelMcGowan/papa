use std::marker::PhantomData;

use crate::prelude::*;

pub struct RepetitionBuilder<'a, C: Context<'a>, P: Parser<'a, C, Output>, Output> {
    pub(crate) parser: P,
    pub(crate) min: usize,
    pub(crate) max: Option<usize>,
    pub(crate) _phantom: PhantomData<&'a (C, Output)>,
}

impl<'a, C: Context<'a>, P: Parser<'a, C, Output>, Output> RepetitionBuilder<'a, C, P, Output> {
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
    ///
    /// Must be called to turn this [`RepetitionBuilder`] into a parser.
    pub fn collect<Collection>(self) -> Repetition<'a, C, P, Output, Collection>
    where
        Collection: FromIterator<Output>,
    {
        Repetition {
            builder: self,
            _phantom: PhantomData,
        }
    }
}

pub struct Repetition<'a, C: Context<'a>, P: Parser<'a, C, Output>, Output, Collection>
where
    Collection: FromIterator<Output>,
{
    builder: RepetitionBuilder<'a, C, P, Output>,
    _phantom: PhantomData<&'a Collection>,
}

impl<'a, C: Context<'a>, P: Parser<'a, C, Output>, Output, Collection> Parser<'a, C, Collection>
    for Repetition<'a, C, P, Output, Collection>
where
    Collection: FromIterator<Output>,
{
    fn parse(&self, context: &mut C) -> ParseResult<'a, C, Collection> {
        let start = context.location();
        let mut num_matches = 0;

        let parse_iter = std::iter::from_fn(|| {
            let output = self.builder.parser.parse(context).to_result().ok()?;
            num_matches += 1;
            Some(output)
        });

        let collection = if let Some(max) = self.builder.max {
            Collection::from_iter(parse_iter.take(max))
        } else {
            Collection::from_iter(parse_iter)
        };

        if num_matches < self.builder.min {
            context.set_location(start);
            ParseResult::err(None)
        } else {
            ParseResult::ok(collection)
        }
    }
}
