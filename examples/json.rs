use std::collections::HashMap;

use papa::prelude::*;

pub fn main() {
    let s = "{}";

    let (json, errors) = parse_value().parse_input(s);

    for error in errors {
        eprintln!("{error:?}");
    }

    println!("{json:?}");
}

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

fn parse_value<'a>() -> impl Parser<&'a str, Json, ()> {
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

fn parse_object<'a>(expr: impl Parser<&'a str, Json, ()>) -> impl Parser<&'a str, Json, ()> {
    dummy_parser()
}

fn parse_array<'a>(expr: impl Parser<&'a str, Json, ()>) -> impl Parser<&'a str, Json, ()> {
    dummy_parser()
}

fn parse_string<'a>() -> impl Parser<&'a str, Json, ()> {
    dummy_parser()
}

fn parse_number<'a>() -> impl Parser<&'a str, Json, ()> {
    dummy_parser()
}

fn parse_bool<'a>() -> impl Parser<&'a str, Json, ()> {
    dummy_parser()
}

fn parse_null<'a>() -> impl Parser<&'a str, Json, ()> {
    dummy_parser()
}

fn dummy_parser<'a>() -> impl Parser<&'a str, Json, ()> {
    nothing().map(|_| Json::Null)
}
