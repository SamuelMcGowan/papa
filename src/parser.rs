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

#[cfg(test)]
mod tests {
    use super::ParserFallible;
    use crate::context::*;

    fn parse_ident(ctx: &mut VecContext<u8, String>) -> Result<Vec<u8>, String> {
        let s: Vec<_> = ctx
            .eat_while(|c| c.is_ascii_alphabetic() || *c == b'_')
            .collect();

        if s.is_empty() {
            Err("identifier is empty".to_owned())
        } else {
            Ok(s)
        }
    }

    fn parse_ident_always(ctx: &mut VecContext<u8, String>) -> Vec<u8> {
        parse_ident.parse_or_else(ctx, |_ctx: &mut _| b"dummy_ident".to_vec())
    }

    #[test]
    fn foo() {
        let mut ctx: VecContext<u8, String> = VecContext::new("hello");

        let (output, span) = ctx.spanned(parse_ident_always);
        println!("{:?}, {span:?}", String::from_utf8_lossy(&output));
    }

    #[test]
    fn test_ident() {
        let mut ctx: VecContext<u8, String> = VecContext::new("hello");
        assert_eq!(&parse_ident_always(&mut ctx), b"hello");
    }

    #[test]
    fn test_ident_empty() {
        let mut ctx: VecContext<u8, String> = VecContext::new("");
        assert_eq!(&parse_ident_always(&mut ctx), b"dummy_ident");
    }
}
