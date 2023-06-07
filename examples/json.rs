use std::collections::HashMap;

use papa::combinator::choice;
use papa::context::VecContext;
use papa::parser::{Parser, ParserOptional};
use papa::recursive::recursive;

pub fn main() {
    let s = "{}";
    // TODO: avoid allocations where possible.
    let chars: Vec<char> = s.chars().collect();

    let mut ctx = Ctx::new(chars);

    let json = parse_value().parse(&mut ctx);

    for error in ctx.errors() {
        eprintln!("{error:?}");
    }

    println!("{json:?}");
}

type Ctx = VecContext<char, ()>;

#[derive(Debug)]
enum Json {
    Object(HashMap<String, Json>),
    Array(Vec<Json>),
    String(String),
    Integer(u64),
    Float(f64),
    Bool(bool),
    Null,
}

fn parse_value() -> impl ParserOptional<Ctx, Json> {
    recursive(|expr| {
        choice((
            parse_object(expr.clone()),
            parse_array(expr),
            parse_string(),
            parse_number(),
            parse_bool(),
            parse_null(),
        ))
    })
}

fn parse_object(expr: impl ParserOptional<Ctx, Json>) -> impl ParserOptional<Ctx, Json> {
    dummy_parser
}

fn parse_array(expr: impl ParserOptional<Ctx, Json>) -> impl ParserOptional<Ctx, Json> {
    dummy_parser
}

fn parse_string() -> impl ParserOptional<Ctx, Json> {
    dummy_parser
}

fn parse_number() -> impl ParserOptional<Ctx, Json> {
    dummy_parser
}

fn parse_bool() -> impl ParserOptional<Ctx, Json> {
    dummy_parser
}

fn parse_null() -> impl ParserOptional<Ctx, Json> {
    dummy_parser
}

fn dummy_parser(ctx: &mut Ctx) -> Option<Json> {
    None
}
