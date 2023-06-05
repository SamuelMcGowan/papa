use std::fmt;
use std::ops::Range;

#[derive(Clone, Copy)]
pub struct Span<L: Location> {
    start: L,
    end: L,
}

impl<L: Location> Span<L> {
    pub fn new(start: L, end: L) -> Self {
        Self {
            start,
            end: end.max(start),
        }
    }

    pub fn start(&self) -> L {
        self.start
    }

    pub fn end(&self) -> L {
        self.end
    }

    pub fn len(&self) -> L::Diff {
        self.start.abs_diff(self.end)
    }

    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }

    pub fn to_range(self) -> Range<L> {
        self.start..self.end
    }
}

impl<L: Location> fmt::Debug for Span<L> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.to_range().fmt(f)
    }
}

pub trait Location: Copy + Eq + Ord + fmt::Debug {
    type Diff: Copy + Eq;

    fn abs_diff(self, other: Self) -> Self::Diff;
}

macro_rules! impl_loc {
    ($($t:ty)*) => {
        $(
            impl Location for $t {
                type Diff = $t;

                fn abs_diff(self, other: Self) -> Self::Diff {
                    self.abs_diff(other)
                }
            }
        )*
    };
}

impl_loc! { usize u128 u64 u32 u16 u8 }
