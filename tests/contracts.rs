use std::{fs, path::Path};

use ts_borrow_checker::{Diagnostic, analyze};

#[test]
fn fixture_contracts_are_met() {
    for fixture in [
        "use-after-move.ts",
        "double-dispose.js",
        "valid-ownership.ts",
    ] {
        let source = fs::read_to_string(Path::new("tests/fixtures").join(fixture)).unwrap();
        let expected =
            fs::read_to_string(Path::new("tests/fixtures").join(format!("{fixture}.expected")))
                .unwrap();
        let actual = analyze(&source)
            .into_iter()
            .map(format_diagnostic)
            .collect::<Vec<_>>()
            .join("\n");
        assert_eq!(actual, expected.trim_end(), "fixture: {fixture}");
    }
}

fn format_diagnostic(diagnostic: Diagnostic) -> String {
    format!(
        "{}:{}:{}: {}",
        diagnostic.code, diagnostic.line, diagnostic.column, diagnostic.message
    )
}
