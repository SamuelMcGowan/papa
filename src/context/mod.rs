pub mod slice;

use self::slice::Slice;
use crate::span::Location;

pub struct Context<In: Slice, Error> {
    slice_all: In,
    slice_current: In,

    loc: In::Location,

    errors: Vec<Error>,
}

impl<In: Slice, Error> Context<In, Error> {
    pub fn new(slice: In) -> Self {
        Self {
            slice_all: slice,
            slice_current: slice,

            loc: Location::start(),

            errors: vec![],
        }
    }

    pub fn slice_all(&self) -> In {
        self.slice_all
    }

    pub fn slice_current(&self) -> In {
        self.slice_current
    }

    pub fn location(&self) -> In::Location {
        self.loc
    }

    pub fn set_location(&mut self, loc: In::Location) {
        self.loc = loc;
    }

    pub fn report(&mut self, error: Error) {
        self.errors.push(error);
    }

    pub fn errors(&self) -> &[Error] {
        &self.errors
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
