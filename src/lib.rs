pub mod combinator;
pub mod context;
pub mod parser;
pub mod primitive;
pub mod recursive;
pub mod span;
pub mod utils;

pub mod prelude {
    pub use crate::combinator::chain::chain;
    pub use crate::combinator::choice::choice;
    pub use crate::context::Context;
    pub use crate::parser::{ParseResult, Parser};
    pub use crate::primitive::{any, func, nothing, pred};
    pub use crate::recursive::recursive;
    pub use crate::span::Span;
}
