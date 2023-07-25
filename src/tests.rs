use std::fmt::Debug;

use crate::prelude::*;

fn run_tests<'a, Out: Debug + Eq>(
    parser: impl Parser<&'a str, Out, String>,
    cases: &[(&'a str, Option<Out>, Vec<String>)],
) {
    for (input, output, errors) in cases {
        println!("testing input: {input:?}");
        let (output_actual, errors_actual) = parser.parse_input(input);
        assert_eq!(output, &output_actual);
        assert_eq!(errors, &errors_actual);
    }
}

#[test]
fn test_ident() {
    run_tests(
        crate::utils::ident(),
        &[
            ("", None, vec![]),
            ("hello", Some("hello"), vec![]),
            ("hello world", Some("hello"), vec![]),
        ],
    );
}

#[test]
fn test_ident_whitespace() {
    run_tests(
        chain((crate::utils::ident(), crate::utils::space()))
            .map(|(ident, _)| ident)
            .repeat()
            .min(1)
            .collect(),
        &[
            ("", None, vec![]),
            ("hello", Some(vec!["hello"]), vec![]),
            ("hello \t world", Some(vec!["hello", "world"]), vec![]),
        ],
    );
}
