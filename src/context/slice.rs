use crate::span::Location;

pub trait Slice: Sized + Copy {
    type Token: Copy;
    type Location: Location;

    fn next(&self) -> Option<(Self::Token, Self)>;

    fn slice(&self, start: Self::Location, end: Self::Location) -> Option<Self>;
}

impl<'a, T: Copy> Slice for &'a [T] {
    type Token = T;
    type Location = usize;

    fn next(&self) -> Option<(Self::Token, Self)> {
        match self {
            [first, rest @ ..] => Some((*first, rest)),
            _ => None,
        }
    }

    fn slice(&self, start: Self::Location, end: Self::Location) -> Option<Self> {
        self.get(start..end)
    }
}

impl<'a> Slice for &'a str {
    type Token = char;
    type Location = usize;

    fn next(&self) -> Option<(Self::Token, Self)> {
        let mut chars = self.chars();
        chars.next().map(|c| (c, chars.as_str()))
    }

    fn slice(&self, start: Self::Location, end: Self::Location) -> Option<Self> {
        self.get(start..end)
    }
}
