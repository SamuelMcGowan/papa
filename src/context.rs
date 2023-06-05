use crate::parser::Parser;
use crate::span::{Location, Span};

pub trait Context: Sized {
    type Token;
    type Location: Location;

    type Error;

    fn next(&mut self) -> Option<Self::Token>;
    fn peek(&self) -> Option<&Self::Token>;

    fn location(&self) -> Self::Location;
    fn set_location(&mut self, location: Self::Location);

    fn report(&mut self, error: Self::Error);

    #[inline]
    fn matches<F>(&self, pred: F) -> bool
    where
        F: FnMut(&Self::Token) -> bool,
    {
        self.peek().is_some_and(pred)
    }

    #[inline]
    fn eat_if<F>(&mut self, pred: F) -> Option<Self::Token>
    where
        F: FnMut(&Self::Token) -> bool,
    {
        if self.matches(pred) {
            self.next()
        } else {
            None
        }
    }

    #[inline]
    fn eat_while<F>(&mut self, pred: F) -> EatWhile<Self, F>
    where
        Self: Sized,
        F: FnMut(&Self::Token) -> bool,
    {
        EatWhile { tokens: self, pred }
    }

    fn spanned<P: Parser<Self, Output>, Output>(
        &mut self,
        mut parser: P,
    ) -> (Output, Span<Self::Location>) {
        let start = self.location();
        let output = parser.parse(self);
        let end = self.location();
        (output, Span::new(start, end))
    }
}

pub struct EatWhile<'a, C, F>
where
    C: Context,
    F: FnMut(&C::Token) -> bool,
{
    tokens: &'a mut C,
    pred: F,
}

impl<C, F> Iterator for EatWhile<'_, C, F>
where
    C: Context,
    F: FnMut(&C::Token) -> bool + Copy,
{
    type Item = C::Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.tokens.eat_if(self.pred)
    }
}

pub struct VecContext<Token: Clone, Error> {
    tokens: Vec<Token>,
    errors: Vec<Error>,
    loc: usize,
}

impl<Token: Clone, Error> VecContext<Token, Error> {
    pub fn new(tokens: impl Into<Vec<Token>>) -> Self {
        Self {
            tokens: tokens.into(),
            errors: vec![],
            loc: 0,
        }
    }

    pub fn tokens(&self) -> &[Token] {
        &self.tokens
    }

    pub fn errors(&self) -> &[Error] {
        &self.errors
    }
}

impl<Token: Clone, Error> Context for VecContext<Token, Error> {
    type Token = Token;
    type Location = usize;

    type Error = Error;

    fn next(&mut self) -> Option<Self::Token> {
        let token = self.tokens.get(self.loc)?.clone();
        self.loc += 1;
        Some(token)
    }

    fn peek(&self) -> Option<&Self::Token> {
        self.tokens.get(self.loc)
    }

    fn location(&self) -> usize {
        self.loc
    }

    fn set_location(&mut self, location: usize) {
        self.loc = location;
    }

    fn report(&mut self, error: Self::Error) {
        self.errors.push(error);
    }
}
