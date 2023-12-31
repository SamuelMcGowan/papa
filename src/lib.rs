pub mod combinator;
pub mod context;
pub mod parser;
pub mod primitive;
pub mod recursive;
pub mod utils;

#[cfg(test)]
mod tests;

pub mod prelude {
    pub use crate::combinator::chain::chain;
    pub use crate::combinator::choice::choice;
    pub use crate::context::span::Span;
    pub use crate::parser::{ParseResult, Parser};
    pub use crate::primitive::{any, func, just, nothing, pred};
    pub use crate::recursive::recursive;
}
