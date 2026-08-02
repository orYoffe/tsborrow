//! The deliberately small, deterministic core of the first checker release.
//! It analyzes explicit ownership operations before attempting inference.

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub code: &'static str,
    pub line: usize,
    pub column: usize,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ValueState {
    Available,
    Moved,
    Disposed,
}

/// Checks a source file using explicit `move(value)` and `dispose(value)` operations.
///
/// A value becomes tracked when declared using `Owned<T>` in TypeScript or when its
/// immediately preceding JSDoc line carries an `@owned` marker in JavaScript.
pub fn analyze(source: &str) -> Vec<Diagnostic> {
    let mut states = HashMap::<String, ValueState>::new();
    let mut diagnostics = Vec::new();
    let mut next_declaration_is_owned = false;

    for (index, source_line) in source.lines().enumerate() {
        let line = index + 1;
        let code = strip_line_comment(source_line).trim();
        if code.contains("@owned") {
            next_declaration_is_owned = true;
            continue;
        }
        if code.is_empty() {
            continue;
        }

        if let Some(name) = owned_declaration(code, next_declaration_is_owned) {
            states.insert(name, ValueState::Available);
            next_declaration_is_owned = false;
            continue;
        }
        next_declaration_is_owned = false;

        for name in states.keys().cloned().collect::<Vec<_>>() {
            let operation = operation_on(code, &name);
            match (states[&name], operation) {
                (ValueState::Available, Some(Operation::Move)) => {
                    states.insert(name, ValueState::Moved);
                }
                (ValueState::Available, Some(Operation::Dispose)) => {
                    states.insert(name, ValueState::Disposed);
                }
                (ValueState::Moved, Some(Operation::Use)) => diagnostics.push(diagnostic(
                    "E2003",
                    line,
                    source_line,
                    &name,
                    "use after move",
                )),
                (ValueState::Disposed, Some(Operation::Use)) => diagnostics.push(diagnostic(
                    "E2004",
                    line,
                    source_line,
                    &name,
                    "use after disposal",
                )),
                (ValueState::Disposed, Some(Operation::Dispose)) => diagnostics.push(diagnostic(
                    "E2005",
                    line,
                    source_line,
                    &name,
                    "value disposed more than once",
                )),
                _ => {}
            }
        }
    }
    diagnostics
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operation {
    Move,
    Dispose,
    Use,
}

fn owned_declaration(code: &str, has_jsdoc_marker: bool) -> Option<String> {
    let is_owned = code.contains("Owned<") || has_jsdoc_marker;
    if !is_owned
        || !(code.starts_with("let ") || code.starts_with("const ") || code.starts_with("var "))
    {
        return None;
    }
    code.split_whitespace().nth(1).map(|name| {
        name.trim_end_matches(':')
            .split(':')
            .next()
            .unwrap_or(name)
            .to_owned()
    })
}

fn operation_on(code: &str, name: &str) -> Option<Operation> {
    let move_call = format!("move({name})");
    if code.contains(&move_call) {
        return Some(Operation::Move);
    }
    let dispose_call = format!("dispose({name})");
    if code.contains(&dispose_call) {
        return Some(Operation::Dispose);
    }
    if contains_identifier(code, name) {
        Some(Operation::Use)
    } else {
        None
    }
}

fn contains_identifier(text: &str, name: &str) -> bool {
    text.match_indices(name).any(|(start, _)| {
        let before = text[..start].chars().next_back();
        let after = text[start + name.len()..].chars().next();
        !before.is_some_and(is_identifier_char) && !after.is_some_and(is_identifier_char)
    })
}

fn is_identifier_char(character: char) -> bool {
    character.is_ascii_alphanumeric() || character == '_' || character == '$'
}

fn strip_line_comment(line: &str) -> &str {
    line.split_once("//").map_or(line, |(code, _)| code)
}

fn diagnostic(
    code: &'static str,
    line: usize,
    source: &str,
    name: &str,
    reason: &str,
) -> Diagnostic {
    Diagnostic {
        code,
        line,
        column: source.find(name).map_or(1, |position| position + 1),
        message: format!("{reason}: `{name}`"),
    }
}
