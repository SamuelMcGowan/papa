use std::marker::PhantomData;

pub trait Context {
    type Token;
    type Location;

    type Error;

    fn next(&mut self) -> Option<Self::Token>;
    fn peek(&self) -> Option<&Self::Token>;

    fn location(&self) -> Self::Location;
    fn set_location(&mut self, location: Self::Location);

    fn report(&mut self, error: Self::Error);

    #[inline]
    fn matches<P>(&self, pat: P) -> bool
    where
        Self::Token: Match<P>,
    {
        self.peek().is_some_and(|item| item.is_match(pat))
    }

    #[inline]
    fn eat<P>(&mut self, pat: P) -> Option<Self::Token>
    where
        Self::Token: Match<P>,
    {
        if self.matches(pat) {
            self.next()
        } else {
            None
        }
    }

    #[inline]
    fn eat_while<P>(&mut self, pat: P) -> EatWhile<Self, P>
    where
        Self: Sized,
        Self::Token: Match<P>,
    {
        EatWhile {
            tokens: self,
            pattern: pat,
        }
    }
}

pub trait Match<Pattern> {
    fn is_match(&self, pattern: Pattern) -> bool;
}

pub struct Cond<T, F: Fn(&T) -> bool> {
    f: F,
    _phantom: PhantomData<T>,
}
impl<T, F: Fn(&T) -> bool + Clone> Clone for Cond<T, F> {
    fn clone(&self) -> Self {
        Self {
            f: self.f.clone(),
            _phantom: PhantomData,
        }
    }
}
impl<T, F: Fn(&T) -> bool + Copy> Copy for Cond<T, F> {}

pub fn cond<T, F: Fn(&T) -> bool>(f: F) -> Cond<T, F> {
    Cond {
        f,
        _phantom: PhantomData,
    }
}

impl<T: Eq> Match<&T> for T {
    fn is_match(&self, pattern: &T) -> bool {
        self == pattern
    }
}

impl<T, F: Fn(&T) -> bool> Match<Cond<T, F>> for T {
    fn is_match(&self, pattern: Cond<T, F>) -> bool {
        (pattern.f)(self)
    }
}

pub struct EatWhile<'a, T, P>
where
    T: Context,
    T::Token: Match<P>,
{
    tokens: &'a mut T,
    pattern: P,
}

impl<T, P> Iterator for EatWhile<'_, T, P>
where
    T: Context,
    P: Copy,
    T::Token: Match<P>,
{
    type Item = T::Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.tokens.eat(self.pattern)
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

    fn location(&self) -> Self::Location {
        self.loc
    }

    fn set_location(&mut self, location: Self::Location) {
        self.loc = location;
    }

    fn report(&mut self, error: Self::Error) {
        self.errors.push(error);
    }
}
