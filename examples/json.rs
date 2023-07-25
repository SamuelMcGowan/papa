use std::collections::HashMap;

use papa::context::VecContext;
use papa::prelude::*;

pub fn main() {
    let s = "{}";
    // TODO: avoid allocations where possible.
    let chars: Vec<char> = s.chars().collect();

    let mut ctx = Ctx::new(&chars);

    let json = parse_value().parse(&mut ctx);

    for error in ctx.errors() {
        eprintln!("{error:?}");
    }

    println!("{json:?}");
}

type Ctx<'a> = VecContext<'a, char, ()>;

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

fn parse_value<'a>() -> impl Parser<'a, Ctx<'a>, Json> {
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

fn parse_object<'a>(expr: impl Parser<'a, Ctx<'a>, Json>) -> impl Parser<'a, Ctx<'a>, Json> {
    dummy_parser()
}

fn parse_array<'a>(expr: impl Parser<'a, Ctx<'a>, Json>) -> impl Parser<'a, Ctx<'a>, Json> {
    dummy_parser()
}

fn parse_string<'a>() -> impl Parser<'a, Ctx<'a>, Json> {
    dummy_parser()
}

fn parse_number<'a>() -> impl Parser<'a, Ctx<'a>, Json> {
    dummy_parser()
}

fn parse_bool<'a>() -> impl Parser<'a, Ctx<'a>, Json> {
    dummy_parser()
}

fn parse_null<'a>() -> impl Parser<'a, Ctx<'a>, Json> {
    dummy_parser()
}

fn dummy_parser<'a>() -> impl Parser<'a, Ctx<'a>, Json> {
    nothing().map(|_| Json::Null)
}
