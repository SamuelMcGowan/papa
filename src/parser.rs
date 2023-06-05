use std::marker::PhantomData;

use crate::combinator::OkOr;
use crate::context::Context;

/// A parser.
pub trait Parser<C: Context, Output> {
    fn parse(&mut self, context: &mut C) -> Output;
}

impl<F: FnMut(&mut C) -> Output, C: Context, Output> Parser<C, Output> for F {
    fn parse(&mut self, context: &mut C) -> Output {
        self(context)
    }
}

/// An extension trait for parsers that output a [`Result`].
pub trait ParserFallible<C: Context, Success>: Parser<C, Result<Success, C::Error>> {
    fn parse_or_else(&mut self, context: &mut C, mut recover: impl Parser<C, Success>) -> Success
    where
        Self: Sized,
    {
        match self.parse(context) {
            Ok(ok) => ok,
            Err(err) => {
                context.report(err);
                recover.parse(context)
            }
        }
    }
}

impl<P: Parser<C, Result<Success, C::Error>>, C: Context, Success> ParserFallible<C, Success>
    for P
{
}

/// An extension trait for parsers that output an [`Option`].
pub trait ParserOptional<C: Context, Success>: Parser<C, Option<Success>> {
    /// Convert the output of this parser from `Some(ok)` | `None` to `Ok(ok)` |
    /// `Err(error)`.
    fn ok_or(self, error: C::Error) -> OkOr<C, Self, Success>
    where
        Self: Sized,
        C::Error: Clone,
    {
        OkOr {
            parser: self,
            error,
            _phantom: PhantomData,
        }
    }
}

impl<P: Parser<C, Option<Success>>, C: Context, Success> ParserOptional<C, Success> for P {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::*;
    use crate::primitive::any;
    use crate::span::Span;

    type Ctx = VecContext<u8, String>;

    fn parse_ident(ctx: &mut Ctx) -> Result<Vec<u8>, String> {
        let s: Vec<_> = ctx
            .eat_while(|c| c.is_ascii_alphabetic() || *c == b'_')
            .collect();

        if s.is_empty() {
            Err("identifier is empty".to_owned())
        } else {
            Ok(s)
        }
    }

    fn parse_ident_always(ctx: &mut Ctx) -> Vec<u8> {
        parse_ident.parse_or_else(ctx, |_ctx: &mut _| b"dummy_ident".to_vec())
    }

    #[test]
    fn ident() {
        let mut ctx = Ctx::new("hello");
        assert_eq!(&parse_ident_always(&mut ctx), b"hello");
    }

    #[test]
    fn empty_ident() {
        let mut ctx = Ctx::new("");
        assert_eq!(&parse_ident_always(&mut ctx), b"dummy_ident");
    }

    #[test]
    fn spanned_token() {
        let mut ctx = Ctx::new("hello");
        let token = ctx.spanned(any().ok_or("oops".to_owned()));
        assert_eq!(token, (Ok(b'h'), Span::new(0, 1)));
    }

    #[test]
    fn no_token() {
        let mut ctx = Ctx::new("");
        let token = any().ok_or("oops".to_owned()).parse(&mut ctx);
        assert_eq!(token, Err("oops".to_owned()));
    }

    #[test]
    fn spanned_ident() {
        let mut ctx = Ctx::new("hello");
        let (output, span) = ctx.spanned(parse_ident_always);
        assert_eq!(&output, b"hello");
        assert_eq!(span, Span::new(0, 5));
    }
}
