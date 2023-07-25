use crate::prelude::*;

pub fn ident<C: Context<Token = char, Slice = Slice>, Slice>() -> impl Parser<C, Slice> {
    chain((pred(is_ident_start), pred(is_ident).repeat())).to_slice()
}

fn is_ident_start(c: &char) -> bool {
    c.is_ascii_alphabetic() || *c == '_'
}

fn is_ident(c: &char) -> bool {
    c.is_ascii_alphanumeric() || *c == '_'
}
