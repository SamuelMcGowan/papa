use crate::span::Location;

pub trait Context: Sized {
    type Token: Copy;
    type Slice;

    type Location: Location;

    type Error;

    fn next(&mut self) -> Option<Self::Token>;

    fn location(&self) -> Self::Location;
    fn set_location(&mut self, location: Self::Location);

    fn slice(&self, start: Self::Location, end: Self::Location) -> Self::Slice;

    fn report(&mut self, error: Self::Error);
}

pub struct VecContext<'a, Token: Clone, Error> {
    tokens: &'a [Token],
    errors: Vec<Error>,
    loc: usize,
}

impl<'a, Token: Clone, Error> VecContext<'a, Token, Error> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self {
            tokens,
            errors: vec![],
            loc: 0,
        }
    }

    pub fn tokens(&self) -> &[Token] {
        self.tokens
    }

    pub fn errors(&self) -> &[Error] {
        &self.errors
    }
}

impl<'a, Token: Copy, Error: 'a> Context for VecContext<'a, Token, Error> {
    type Token = Token;
    type Slice = &'a [Token];

    type Location = usize;

    type Error = Error;

    fn next(&mut self) -> Option<Self::Token> {
        let token = *self.tokens.get(self.loc)?;
        self.loc += 1;
        Some(token)
    }

    fn location(&self) -> usize {
        self.loc
    }

    fn set_location(&mut self, location: usize) {
        self.loc = location;
    }

    fn slice(&self, start: Self::Location, end: Self::Location) -> Self::Slice {
        &self.tokens[start..end]
    }

    fn report(&mut self, error: Self::Error) {
        self.errors.push(error);
    }
}
