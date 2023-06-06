use std::cell::OnceCell;
use std::marker::PhantomData;
use std::rc::Rc;

use crate::context::Context;
use crate::parser::Parser;

// Method taken from https://crates.io/crates/chumsky.

struct RecursiveInner<C: Context, Output> {
    parser: OnceCell<Box<dyn Parser<C, Output>>>,
    _phantom: PhantomData<*const (C, Output)>,
}

pub struct Recursive<C: Context, Output> {
    inner: Rc<RecursiveInner<C, Output>>,
}

impl<C: Context, Output> Clone for Recursive<C, Output> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<C: Context, Output> Recursive<C, Output> {}

impl<C: Context, Output> Parser<C, Output> for Recursive<C, Output> {
    fn parse(&self, context: &mut C) -> Output {
        let parser = self
            .inner
            .parser
            .get()
            .expect("parser called before (end of) definition");
        parser.parse(context)
    }
}

pub fn recursive<C: Context, P: Parser<C, Output> + 'static, Output>(
    get_parser: impl Fn(Recursive<C, Output>) -> P,
) -> Recursive<C, Output> {
    let inner = RecursiveInner {
        parser: OnceCell::new(),
        _phantom: PhantomData,
    };
    let recursive = Recursive {
        inner: Rc::new(inner),
    };

    let parser = get_parser(recursive.clone());

    // Can't fail but can't call unwrap since `P` doesn't implement `Debug`.
    let _ = recursive.inner.parser.set(Box::new(parser));

    recursive
}
