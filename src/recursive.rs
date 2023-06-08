use std::cell::OnceCell;
use std::rc::Rc;

use crate::context::Context;
use crate::parser::{ParseResult, Parser};

// Technique stolen from https://crates.io/crates/chumsky.

pub struct Recursive<C: Context, Output> {
    parser: OnceCell<Box<dyn Parser<C, Output>>>,
}

impl<C: Context, Output> Recursive<C, Output> {
    fn declare() -> Rc<Self> {
        Rc::new(Self {
            parser: OnceCell::new(),
        })
    }

    fn define(&self, parser: impl Parser<C, Output> + 'static) {
        if self.parser.set(Box::new(parser)).is_err() {
            panic!("Tried to define the parser multiple times.");
        }
    }
}

impl<C: Context, Output> Parser<C, Output> for Rc<Recursive<C, Output>> {
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
pub fn recursive<C: Context, P: Parser<C, Output> + 'static, Output>(
    build_parser: impl Fn(Rc<Recursive<C, Output>>) -> P,
) -> Rc<Recursive<C, Output>> {
    let rec = Recursive::declare();
    rec.define(build_parser(rec.clone()));
    rec
}
