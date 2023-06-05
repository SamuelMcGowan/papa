use crate::context::Context;

pub trait Parser<C: Context, Output> {
    fn parse(&mut self, context: &mut C) -> Output;
}

pub trait ParserFallible<C: Context, Success> {
    fn parse_fallible(&mut self, context: &mut C) -> Result<Success, C::Error>;

    fn parse_or_else(&mut self, context: &mut C, mut recover: impl Parser<C, Success>) -> Success {
        match self.parse_fallible(context) {
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
    fn parse_fallible(&mut self, context: &mut C) -> Result<Success, C::Error> {
        self.parse(context)
    }
}

impl<F: FnMut(&mut C) -> Output, C: Context, Output> Parser<C, Output> for F {
    fn parse(&mut self, context: &mut C) -> Output {
        self(context)
    }
}

pub fn any<C: Context>() -> impl Parser<C, Result<C::Token, ()>> {
    |ctx: &mut C| ctx.next().ok_or(())
}

pub fn pred<C: Context, F: FnMut(&C::Token) -> bool + Copy>(
    pred: F,
) -> impl Parser<C, Result<C::Token, ()>> {
    move |ctx: &mut C| ctx.eat_if(pred).ok_or(())
}

pub fn just<C: Context>(token: C::Token) -> impl Parser<C, Result<C::Token, ()>>
where
    C::Token: Eq,
{
    move |ctx: &mut C| ctx.eat_if(|t| t == &token).ok_or(())
}

#[cfg(test)]
mod tests {
    use super::{any, ParserFallible};
    use crate::context::*;
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
        let token = ctx.spanned(any());
        assert_eq!(token, (Ok(b'h'), Span::new(0, 1)));
    }

    #[test]
    fn spanned_ident() {
        let mut ctx = Ctx::new("hello");
        let (output, span) = ctx.spanned(parse_ident_always);
        assert_eq!(&output, b"hello");
        assert_eq!(span, Span::new(0, 5));
    }
}
