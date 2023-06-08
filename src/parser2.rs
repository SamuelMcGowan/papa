use crate::context::Context;

pub trait Parser {
    type Context: Context;

    type Result: ParseResult<Self::Context, Self::Output>;
    type Output;

    fn parse(&self, context: &mut Self::Context) -> Self::Result;
}

pub trait ParseResult<C: Context, Output> {
    fn to_result(self) -> Result<Output, Option<C::Error>>;
}

impl<C: Context, Output> ParseResult<C, Output> for Result<Output, C::Error> {
    fn to_result(self) -> Result<Output, Option<C::Error>> {
        self.map_err(Some)
    }
}

impl<C: Context, Output> ParseResult<C, Output> for Option<Output> {
    fn to_result(self) -> Result<Output, Option<C::Error>> {
        self.ok_or(None)
    }
}

impl<C: Context> ParseResult<C, ()> for () {
    fn to_result(self) -> Result<(), Option<C::Error>> {
        Ok(())
    }
}
