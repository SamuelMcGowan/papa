use std::cell::OnceCell;
use std::rc::Rc;

use crate::context::slice::Slice;
use crate::context::Context;
use crate::parser::{ParseResult, Parser};

// Technique stolen from https://crates.io/crates/chumsky.

pub struct Recursive<'a, In: Slice, Out, Error> {
    parser: OnceCell<Box<dyn Parser<In, Out, Error> + 'a>>,
}

impl<'a, In: Slice, Out, Error> Recursive<'a, In, Out, Error> {
    fn declare() -> Rc<Self> {
        Rc::new(Self {
            parser: OnceCell::new(),
        })
    }

    fn define(&self, parser: impl Parser<In, Out, Error> + 'a) {
        if self.parser.set(Box::new(parser)).is_err() {
            panic!("Tried to define the parser multiple times.");
        }
    }
}

impl<'a, In: Slice, Out, Error> Parser<In, Out, Error> for Rc<Recursive<'a, In, Out, Error>> {
    fn parse(&self, context: &mut Context<In, Error>) -> ParseResult<Out, Error> {
        let parser = self
            .parser
            .get()
            .expect("parser called before (end of) definition");
        parser.parse(context)
    }
}

/// Construct a parser that can call itself.
///
/// A reference to the parser itself is provided to the builder, and can be used
/// to construct a recursive parser.
///
/// Panics if the parser is called inside the builder.
pub fn recursive<'a, In: Slice, Out, Error, P: Parser<In, Out, Error> + 'a>(
    build_parser: impl Fn(Rc<Recursive<'a, In, Out, Error>>) -> P,
) -> Rc<Recursive<'a, In, Out, Error>> {
    let rec = Recursive::declare();
    rec.define(build_parser(rec.clone()));
    rec
}
