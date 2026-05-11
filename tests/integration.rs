//! End-to-end tests: every `tests/cases/*.lox` file is run through `lox::run`
//! and its output is snapshotted with `insta`.

use std::panic::{AssertUnwindSafe, catch_unwind};

#[test]
fn lox_cases() {
    insta::glob!("cases/*.lox", |path| {
        let source = std::fs::read_to_string(path).unwrap();
        let result = catch_unwind(AssertUnwindSafe(|| lox::run(&source)));
        let rendered = match result {
            Ok(Ok(())) => "ok".to_string(),
            Ok(Err(e)) => format!("error: {e}"),
            Err(_) => "panic (not yet implemented)".to_string(),
        };
        insta::assert_snapshot!(rendered);
    });
}
