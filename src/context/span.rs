use std::fmt;
use std::ops::Range;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Span {
    start: usize,
    end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self {
            start,
            end: end.max(start),
        }
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn end(&self) -> usize {
        self.end
    }

    pub fn len(&self) -> usize {
        self.start.abs_diff(self.end)
    }

    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }

    pub fn to_range(self) -> Range<usize> {
        self.start..self.end
    }
}

impl fmt::Debug for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_range().fmt(f)
    }
}
