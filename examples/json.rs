use std::collections::HashMap;

use papa::combinator::choice;
use papa::context::VecContext;
use papa::parser::Parser;
use papa::primitive::nothing;
use papa::recursive::recursive;

pub fn main() {
    let s = "{}";
    // TODO: avoid allocations where possible.
    let chars: Vec<char> = s.chars().collect();

    let mut ctx = Ctx::new(chars);

    let json = parse_value().parse(&mut ctx).to_result();

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

fn parse_value() -> impl Parser<Ctx, Json> {
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

fn parse_object(expr: impl Parser<Ctx, Json>) -> impl Parser<Ctx, Json> {
    dummy_parser()
}

fn parse_array(expr: impl Parser<Ctx, Json>) -> impl Parser<Ctx, Json> {
    dummy_parser()
}

fn parse_string() -> impl Parser<Ctx, Json> {
    dummy_parser()
}

fn parse_number() -> impl Parser<Ctx, Json> {
    dummy_parser()
}

fn parse_bool() -> impl Parser<Ctx, Json> {
    dummy_parser()
}

fn parse_null() -> impl Parser<Ctx, Json> {
    dummy_parser()
}

fn dummy_parser() -> impl Parser<Ctx, Json> {
    nothing().map(|_| Json::Null)
}
