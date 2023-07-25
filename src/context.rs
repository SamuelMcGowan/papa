use crate::span::Location;

pub trait Context: Sized {
    type Token;
    type Slice: ?Sized;

    type Location: Location;

    type Error;

    fn next(&mut self) -> Option<Self::Token>;
    fn peek(&self) -> Option<&Self::Token>;

    fn location(&self) -> Self::Location;
    fn set_location(&mut self, location: Self::Location);

    fn slice(&self, start: Self::Location, end: Self::Location) -> &Self::Slice;

    fn report(&mut self, error: Self::Error);
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
    type Slice = [Token];

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

    fn slice(&self, start: Self::Location, end: Self::Location) -> &Self::Slice {
        &self.tokens[start..end]
    }

    fn report(&mut self, error: Self::Error) {
        self.errors.push(error);
    }
}
