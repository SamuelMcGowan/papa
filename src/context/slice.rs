use crate::span::Location;

pub trait Slice: Sized + Copy {
    type Token: Copy;
    type Location: Location;

    fn get_at(&self, loc: Self::Location) -> Option<Self::Token>;
    fn slice(&self, start: Self::Location, end: Self::Location) -> Option<Self>;
}

impl<'a, T: Copy> Slice for &'a [T] {
    type Token = T;
    type Location = usize;

    fn get_at(&self, loc: Self::Location) -> Option<Self::Token> {
        self.get(loc).copied()
    }

    fn slice(&self, start: Self::Location, end: Self::Location) -> Option<Self> {
        self.get(start..end)
    }
}

impl<'a> Slice for &'a str {
    type Token = char;
    type Location = usize;

    fn get_at(&self, loc: Self::Location) -> Option<Self::Token> {
        self.get(loc..)?.chars().next()
    }

    fn slice(&self, start: Self::Location, end: Self::Location) -> Option<Self> {
        self.get(start..end)
    }
}
