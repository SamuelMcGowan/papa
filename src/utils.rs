use crate::context::slice::Slice;
use crate::prelude::*;

pub fn ident<In: Slice<Token = char>, Error>() -> impl Parser<In, In, Error> {
    chain((pred(is_ident_start), pred(is_ident).repeat())).to_slice()
}

pub fn space<In: Slice<Token = char>, Error>() -> impl Parser<In, (), Error> {
    pred(|c: char| c.is_ascii_whitespace()).repeat().drop()
}

fn is_ident_start(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

fn is_ident(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}
