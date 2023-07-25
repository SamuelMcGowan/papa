use std::cell::OnceCell;
use std::rc::Rc;

use crate::context::Context;
use crate::parser::{ParseResult, Parser};

// Technique stolen from https://crates.io/crates/chumsky.

pub struct Recursive<'a, C: Context, Output> {
    parser: OnceCell<Box<dyn Parser<'a, C, Output> + 'a>>,
}

impl<'a, C: Context, Output> Recursive<'a, C, Output> {
    fn declare() -> Rc<Self> {
        Rc::new(Self {
            parser: OnceCell::new(),
        })
    }

    fn define(&self, parser: impl Parser<'a, C, Output> + 'a) {
        if self.parser.set(Box::new(parser)).is_err() {
            panic!("Tried to define the parser multiple times.");
        }
    }
}

impl<'a, C: Context, Output> Parser<'a, C, Output> for Rc<Recursive<'a, C, Output>> {
    fn parse(&self, context: &mut C) -> ParseResult<C, Output> {
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
pub fn recursive<'a, C: Context, P: Parser<'a, C, Output> + 'a, Output>(
    build_parser: impl Fn(Rc<Recursive<'a, C, Output>>) -> P,
) -> Rc<Recursive<'a, C, Output>> {
    let rec = Recursive::declare();
    rec.define(build_parser(rec.clone()));
    rec
}
