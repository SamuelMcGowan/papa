pub mod slice;
pub mod span;

use self::slice::Slice;
use crate::prelude::ParseResult;

pub struct Context<In: Slice, Error> {
    slice_all: In,
    slice_current: In,

    errors: Vec<Error>,
}

impl<In: Slice, Error> Context<In, Error> {
    pub fn new(slice: In) -> Self {
        Self {
            slice_all: slice,
            slice_current: slice,

            errors: vec![],
        }
    }

    pub fn slice_all(&self) -> In {
        self.slice_all
    }

    pub fn slice_current(&self) -> In {
        self.slice_current
    }

    pub fn location(&self) -> usize {
        self.slice_all.len() - self.slice_current.len()
    }

    pub fn set_location(&mut self, loc: usize) {
        self.slice_current = self
            .slice_all
            .slice(loc, self.slice_all.len())
            .expect("invalid location");
    }

    pub fn report(&mut self, error: Error) {
        self.errors.push(error);
    }

    pub fn errors(&self) -> &[Error] {
        &self.errors
    }

    pub(crate) fn result_to_errors<Out>(
        mut self,
        result: ParseResult<Out, Error>,
    ) -> (Option<Out>, Vec<Error>) {
        let output = match result {
            Ok(output) => Some(output),
            Err(Some(err)) => {
                self.report(err);
                None
            }
            Err(None) => None,
        };
        (output, self.errors)
    }
}

impl<In: Slice, Error> Iterator for Context<In, Error> {
    type Item = In::Token;

    fn next(&mut self) -> Option<Self::Item> {
        match self.slice_current.next() {
            Some((token, rest)) => {
                self.slice_current = rest;
                Some(token)
            }
            None => None,
        }
    }
}
