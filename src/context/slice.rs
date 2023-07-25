pub trait Slice: Sized + Copy {
    type Token: Copy;

    fn next(&self) -> Option<(Self::Token, Self)>;

    fn slice(&self, start: usize, end: usize) -> Option<Self>;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<'a, T: Copy> Slice for &'a [T] {
    type Token = T;

    fn next(&self) -> Option<(Self::Token, Self)> {
        match self {
            [first, rest @ ..] => Some((*first, rest)),
            _ => None,
        }
    }

    fn slice(&self, start: usize, end: usize) -> Option<Self> {
        self.get(start..end)
    }

    fn len(&self) -> usize {
        <[T]>::len(self)
    }
}

impl<'a> Slice for &'a str {
    type Token = char;

    fn next(&self) -> Option<(Self::Token, Self)> {
        let mut chars = self.chars();
        chars.next().map(|c| (c, chars.as_str()))
    }

    fn slice(&self, start: usize, end: usize) -> Option<Self> {
        self.get(start..end)
    }

    fn len(&self) -> usize {
        str::len(self)
    }
}
