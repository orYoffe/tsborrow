use std::{
    fs,
    path::{Path, PathBuf},
};

use ts_borrow_checker::{Diagnostic, analyze};

#[test]
fn fixture_contracts_are_met() {
    let fixtures = source_fixtures(Path::new("tests/fixtures"));
    assert!(
        fixtures.len() >= 20,
        "the conformance suite must remain substantial"
    );

    for fixture in fixtures {
        let source = fs::read_to_string(&fixture).unwrap();
        let expected = fs::read_to_string(format!("{}.expected", fixture.display())).unwrap();
        let diagnostics = analyze(&source);
        let actual = if diagnostics.is_empty() {
            "OK".to_owned()
        } else {
            diagnostics
                .into_iter()
                .map(format_diagnostic)
                .collect::<Vec<_>>()
                .join("\n")
        };
        assert_eq!(
            actual,
            expected.trim_end(),
            "fixture: {}",
            fixture.display()
        );
    }
}

fn source_fixtures(directory: &Path) -> Vec<PathBuf> {
    let mut fixtures = Vec::new();
    for entry in fs::read_dir(directory).unwrap().filter_map(Result::ok) {
        let path = entry.path();
        if path.is_dir() {
            fixtures.extend(source_fixtures(&path));
        } else if matches!(
            path.extension().and_then(|value| value.to_str()),
            Some("ts" | "js")
        ) {
            fixtures.push(path);
        }
    }
    fixtures.sort();
    fixtures
}

fn format_diagnostic(diagnostic: Diagnostic) -> String {
    format!(
        "{}:{}:{}: {}",
        diagnostic.code, diagnostic.line, diagnostic.column, diagnostic.message
    )
}
