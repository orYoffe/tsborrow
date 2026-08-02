//! Ownership analysis for explicit TypeScript and JavaScript ownership contracts.
//!
//! This phase intentionally uses explicit operations (`move`, `borrow`, `borrowMut`,
//! `endBorrow`, and `dispose`). The analysis models moves, places, non-lexical borrow
//! lifetimes, resource cleanup, and conservative control-flow joins.

use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub code: &'static str,
    pub line: usize,
    pub column: usize,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnalysisReport {
    pub diagnostics: Vec<Diagnostic>,
    pub tracked_owners: usize,
    pub tracked_borrows: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OwnerKind {
    Owned,
    Resource,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OwnerState {
    Available,
    Moved,
    Disposed,
}

#[derive(Debug, Clone)]
struct Owner {
    kind: OwnerKind,
    state: OwnerState,
    declaration_line: usize,
    declaration_column: usize,
    maybe_moved: bool,
    moved_places: BTreeSet<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BorrowKind {
    Shared,
    Mutable,
}

#[derive(Debug, Clone)]
struct Borrow {
    name: String,
    owner: String,
    kind: BorrowKind,
    created_line: usize,
    last_use_line: usize,
    ended_line: Option<usize>,
}

impl Borrow {
    fn is_active_at(&self, line: usize) -> bool {
        let active_until = self
            .ended_line
            .map_or(self.last_use_line, |ended| ended.min(self.last_use_line));
        self.created_line < line && line <= active_until
    }

    fn crosses_line(&self, line: usize) -> bool {
        self.created_line < line
            && self.last_use_line > line
            && self.ended_line.is_none_or(|end| end > line)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Context {
    Block,
    Conditional,
    Loop,
}

/// Analyze one JavaScript or TypeScript source file.
///
/// Ownership is opt-in through `Owned<T>` / `Resource<T>` TypeScript types or an
/// immediately preceding `@owned` / `@resource` JSDoc marker in JavaScript.
pub fn analyze(source: &str) -> Vec<Diagnostic> {
    analyze_report(source).diagnostics
}

/// Analyze source and return diagnostics together with adoption statistics.
///
/// The statistics let CI distinguish a clean analysis from a source tree that
/// has not declared any ownership contracts yet.
pub fn analyze_report(source: &str) -> AnalysisReport {
    let source_lines = source.lines().collect::<Vec<_>>();
    let code_lines = source_lines
        .iter()
        .map(|line| strip_line_comment(line).trim().to_owned())
        .collect::<Vec<_>>();
    let borrows = collect_borrows(&code_lines);
    let mut owners = BTreeMap::<String, Owner>::new();
    let mut diagnostics = Vec::new();
    let mut pending_jsdoc = None;
    let mut contexts = Vec::<Context>::new();

    for (index, code) in code_lines.iter().enumerate() {
        let line = index + 1;
        let source_line = source_lines[index];

        if code.contains("@resource") {
            pending_jsdoc = Some(OwnerKind::Resource);
            continue;
        }
        if code.contains("@owned") {
            pending_jsdoc = Some(OwnerKind::Owned);
            continue;
        }
        if code.is_empty() {
            continue;
        }

        if let Some((name, kind)) = owned_declaration(code, pending_jsdoc) {
            owners.insert(
                name.clone(),
                Owner {
                    kind,
                    state: OwnerState::Available,
                    declaration_line: line,
                    declaration_column: identifier_column(source_line, &name),
                    maybe_moved: false,
                    moved_places: BTreeSet::new(),
                },
            );
            pending_jsdoc = None;
            update_contexts(code, &mut contexts);
            continue;
        }
        pending_jsdoc = None;

        check_borrow_uses(code, source_line, line, &borrows, &mut diagnostics);
        check_async_boundary(code, source_line, line, &borrows, &mut diagnostics);

        if let Some((borrow_name, owner_name, kind)) = borrow_declaration(code) {
            check_new_borrow(
                source_line,
                line,
                &owner_name,
                kind,
                &owners,
                &borrows,
                &mut diagnostics,
            );
            // The borrow name is retained in the precomputed borrow table. Keeping this
            // assertion close to parsing protects the two analysis phases from drifting.
            debug_assert!(borrows.contains_key(&borrow_name));
            update_contexts(code, &mut contexts);
            continue;
        }

        if extract_call_argument(code, "endBorrow").is_some() {
            update_contexts(code, &mut contexts);
            continue;
        }

        if let Some(place) = extract_call_argument(code, "move") {
            check_move(
                source_line,
                line,
                &place,
                &mut owners,
                &borrows,
                &contexts,
                &mut diagnostics,
            );
            update_contexts(code, &mut contexts);
            continue;
        }

        if let Some(place) = extract_call_argument(code, "dispose") {
            check_dispose(
                source_line,
                line,
                &place,
                &mut owners,
                &borrows,
                &mut diagnostics,
            );
            update_contexts(code, &mut contexts);
            continue;
        }

        check_owner_uses(code, source_line, line, &owners, &borrows, &mut diagnostics);
        update_contexts(code, &mut contexts);
    }

    for (name, owner) in &owners {
        if owner.kind == OwnerKind::Resource && owner.state == OwnerState::Available {
            diagnostics.push(Diagnostic {
                code: "TSB001",
                line: owner.declaration_line,
                column: owner.declaration_column,
                message: format!("resource may leave scope without being disposed: `{name}`"),
            });
        }
    }

    AnalysisReport {
        diagnostics,
        tracked_owners: owners.len(),
        tracked_borrows: borrows.len(),
    }
}

fn collect_borrows(code_lines: &[String]) -> BTreeMap<String, Borrow> {
    let mut borrows = BTreeMap::new();
    for (index, code) in code_lines.iter().enumerate() {
        if let Some((name, owner, kind)) = borrow_declaration(code) {
            let created_line = index + 1;
            let mut last_use_line = created_line;
            let mut ended_line = None;
            for (later_index, later_code) in code_lines.iter().enumerate().skip(index + 1) {
                let later_line = later_index + 1;
                if contains_identifier(later_code, &name) {
                    last_use_line = later_line;
                }
                if extract_call_argument(later_code, "endBorrow").as_deref() == Some(&name)
                    && ended_line.is_none()
                {
                    ended_line = Some(later_line);
                }
            }
            borrows.insert(
                name.clone(),
                Borrow {
                    name,
                    owner: base_owner(&owner).to_owned(),
                    kind,
                    created_line,
                    last_use_line,
                    ended_line,
                },
            );
        }
    }
    borrows
}

fn check_new_borrow(
    source_line: &str,
    line: usize,
    owner_name: &str,
    requested: BorrowKind,
    owners: &BTreeMap<String, Owner>,
    borrows: &BTreeMap<String, Borrow>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let base = base_owner(owner_name);
    let Some(owner) = owners.get(base) else {
        return;
    };
    let column = identifier_column(source_line, owner_name);
    match owner.state {
        OwnerState::Moved => diagnostics.push(diagnostic(
            "E0382",
            line,
            column,
            format!("use of moved value: `{base}`"),
        )),
        OwnerState::Disposed => diagnostics.push(diagnostic(
            "TSB002",
            line,
            column,
            format!("use of disposed value: `{base}`"),
        )),
        OwnerState::Available => {
            let active = active_borrows(base, line, borrows);
            let has_mutable = active
                .iter()
                .any(|borrow| borrow.kind == BorrowKind::Mutable);
            let has_shared = active
                .iter()
                .any(|borrow| borrow.kind == BorrowKind::Shared);
            match requested {
                BorrowKind::Mutable if has_mutable => diagnostics.push(diagnostic(
                    "E0499",
                    line,
                    column,
                    format!("cannot borrow `{base}` as mutable more than once at a time"),
                )),
                BorrowKind::Mutable if has_shared => diagnostics.push(diagnostic(
                    "E0502",
                    line,
                    column,
                    format!(
                        "cannot borrow `{base}` as mutable because it is also borrowed as shared"
                    ),
                )),
                BorrowKind::Shared if has_mutable => diagnostics.push(diagnostic(
                    "E0502",
                    line,
                    column,
                    format!(
                        "cannot borrow `{base}` as shared because it is also borrowed as mutable"
                    ),
                )),
                _ => {}
            }
        }
    }
}

fn check_move(
    source_line: &str,
    line: usize,
    place: &str,
    owners: &mut BTreeMap<String, Owner>,
    borrows: &BTreeMap<String, Borrow>,
    contexts: &[Context],
    diagnostics: &mut Vec<Diagnostic>,
) {
    let base = base_owner(place);
    let Some(owner) = owners.get_mut(base) else {
        return;
    };
    let column = identifier_column(source_line, place);

    if contexts.contains(&Context::Loop) {
        diagnostics.push(diagnostic(
            "TSB006",
            line,
            column,
            format!("move of `{place}` may execute more than once inside a loop"),
        ));
        return;
    }
    if !active_borrows(base, line, borrows).is_empty() {
        diagnostics.push(diagnostic(
            "E0505",
            line,
            column,
            format!("cannot move `{place}` because it is borrowed"),
        ));
        return;
    }
    match owner.state {
        OwnerState::Moved => diagnostics.push(diagnostic(
            "E0382",
            line,
            column,
            format!("use of moved value: `{place}`"),
        )),
        OwnerState::Disposed => diagnostics.push(diagnostic(
            "TSB002",
            line,
            column,
            format!("use of disposed value: `{place}`"),
        )),
        OwnerState::Available if place == base => {
            if contexts.contains(&Context::Conditional) {
                owner.maybe_moved = true;
            } else {
                owner.state = OwnerState::Moved;
            }
        }
        OwnerState::Available => {
            owner.moved_places.insert(place.to_owned());
        }
    }
}

fn check_dispose(
    source_line: &str,
    line: usize,
    place: &str,
    owners: &mut BTreeMap<String, Owner>,
    borrows: &BTreeMap<String, Borrow>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let base = base_owner(place);
    let Some(owner) = owners.get_mut(base) else {
        return;
    };
    let column = identifier_column(source_line, place);
    if !active_borrows(base, line, borrows).is_empty() {
        diagnostics.push(diagnostic(
            "E0505",
            line,
            column,
            format!("cannot dispose `{place}` because it is borrowed"),
        ));
        return;
    }
    match owner.state {
        OwnerState::Available => owner.state = OwnerState::Disposed,
        OwnerState::Moved => diagnostics.push(diagnostic(
            "E0382",
            line,
            column,
            format!("use of moved value: `{place}`"),
        )),
        OwnerState::Disposed => diagnostics.push(diagnostic(
            "TSB003",
            line,
            column,
            format!("value disposed more than once: `{place}`"),
        )),
    }
}

fn check_owner_uses(
    code: &str,
    source_line: &str,
    line: usize,
    owners: &BTreeMap<String, Owner>,
    borrows: &BTreeMap<String, Borrow>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    for (name, owner) in owners {
        let Some(used_place) = used_place(code, name) else {
            continue;
        };
        let column = identifier_column(source_line, &used_place);

        if owner.state == OwnerState::Moved {
            diagnostics.push(diagnostic(
                "E0382",
                line,
                column,
                format!("use of moved value: `{name}`"),
            ));
            continue;
        }
        if owner.state == OwnerState::Disposed {
            diagnostics.push(diagnostic(
                "TSB002",
                line,
                column,
                format!("use of disposed value: `{name}`"),
            ));
            continue;
        }
        if owner.maybe_moved {
            diagnostics.push(diagnostic(
                "E0382",
                line,
                column,
                format!("use of possibly moved value: `{name}`"),
            ));
            continue;
        }
        if used_place == *name && !owner.moved_places.is_empty() {
            diagnostics.push(diagnostic(
                "E0382",
                line,
                column,
                format!("use of partially moved value: `{name}`"),
            ));
            continue;
        }
        if let Some(moved) = owner
            .moved_places
            .iter()
            .find(|moved| places_overlap(moved, &used_place))
        {
            diagnostics.push(diagnostic(
                "E0382",
                line,
                column,
                format!("use of moved value: `{moved}`"),
            ));
            continue;
        }
        if active_borrows(name, line, borrows)
            .iter()
            .any(|borrow| borrow.kind == BorrowKind::Mutable)
        {
            diagnostics.push(diagnostic(
                "E0503",
                line,
                column,
                format!("cannot use `{name}` because it is mutably borrowed"),
            ));
        }
    }
}

fn check_borrow_uses(
    code: &str,
    source_line: &str,
    line: usize,
    borrows: &BTreeMap<String, Borrow>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    for borrow in borrows.values() {
        if line <= borrow.created_line
            || !contains_identifier(code, &borrow.name)
            || extract_call_argument(code, "endBorrow").as_deref() == Some(&borrow.name)
        {
            continue;
        }
        let column = identifier_column(source_line, &borrow.name);
        if borrow.ended_line.is_some_and(|ended| line > ended) {
            diagnostics.push(diagnostic(
                "TSB005",
                line,
                column,
                format!("use of ended borrow: `{}`", borrow.name),
            ));
        } else if code.trim_start().starts_with("return ") {
            diagnostics.push(diagnostic(
                "E0515",
                line,
                column,
                format!(
                    "cannot return borrow `{}` of local value `{}`",
                    borrow.name, borrow.owner
                ),
            ));
        } else if code.contains("=>") {
            diagnostics.push(diagnostic(
                "E0521",
                line,
                column,
                format!("borrowed value `{}` escapes into a callback", borrow.name),
            ));
        }
    }
}

fn check_async_boundary(
    code: &str,
    source_line: &str,
    line: usize,
    borrows: &BTreeMap<String, Borrow>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if !contains_identifier(code, "await") {
        return;
    }
    let column = identifier_column(source_line, "await");
    for borrow in borrows.values().filter(|borrow| borrow.crosses_line(line)) {
        diagnostics.push(diagnostic(
            "TSB004",
            line,
            column,
            format!(
                "borrow `{}` of `{}` crosses an async suspension point",
                borrow.name, borrow.owner
            ),
        ));
    }
}

fn active_borrows<'a>(
    owner: &str,
    line: usize,
    borrows: &'a BTreeMap<String, Borrow>,
) -> Vec<&'a Borrow> {
    borrows
        .values()
        .filter(|borrow| borrow.owner == owner && borrow.is_active_at(line))
        .collect()
}

fn owned_declaration(code: &str, marker: Option<OwnerKind>) -> Option<(String, OwnerKind)> {
    let kind = if code.contains("Resource<") {
        Some(OwnerKind::Resource)
    } else if code.contains("Owned<") {
        Some(OwnerKind::Owned)
    } else {
        marker
    }?;
    if !(code.starts_with("let ") || code.starts_with("const ") || code.starts_with("var ")) {
        return None;
    }
    let name = code
        .split_whitespace()
        .nth(1)?
        .trim_end_matches(':')
        .split(':')
        .next()?
        .to_owned();
    Some((name, kind))
}

fn borrow_declaration(code: &str) -> Option<(String, String, BorrowKind)> {
    let equals = code.find('=')?;
    let binding = code[..equals]
        .split_whitespace()
        .last()?
        .trim_end_matches(':')
        .to_owned();
    if let Some(owner) = extract_call_argument(&code[equals + 1..], "borrowMut") {
        return Some((binding, owner, BorrowKind::Mutable));
    }
    extract_call_argument(&code[equals + 1..], "borrow")
        .map(|owner| (binding, owner, BorrowKind::Shared))
}

fn extract_call_argument(code: &str, function: &str) -> Option<String> {
    let needle = format!("{function}(");
    let start = code.find(&needle)? + needle.len();
    let tail = &code[start..];
    let end = tail.find(')')?;
    let argument = tail[..end].trim();
    is_place(argument).then(|| argument.to_owned())
}

fn is_place(value: &str) -> bool {
    !value.is_empty()
        && value
            .split('.')
            .all(|part| !part.is_empty() && part.chars().all(is_identifier_char))
}

fn base_owner(place: &str) -> &str {
    place.split('.').next().unwrap_or(place)
}

fn used_place(code: &str, owner: &str) -> Option<String> {
    for (start, _) in code.match_indices(owner) {
        let before = code[..start].chars().next_back();
        let after_owner = start + owner.len();
        let after = code[after_owner..].chars().next();
        if before.is_some_and(is_identifier_char) || after.is_some_and(is_identifier_char) {
            continue;
        }
        let mut end = after_owner;
        let bytes = code.as_bytes();
        while end < code.len() {
            let character = bytes[end] as char;
            if character == '.' || is_identifier_char(character) {
                end += 1;
            } else {
                break;
            }
        }
        return Some(code[start..end].to_owned());
    }
    None
}

fn places_overlap(moved: &str, used: &str) -> bool {
    moved == used
        || used
            .strip_prefix(moved)
            .is_some_and(|suffix| suffix.starts_with('.'))
        || moved
            .strip_prefix(used)
            .is_some_and(|suffix| suffix.starts_with('.'))
}

fn update_contexts(code: &str, contexts: &mut Vec<Context>) {
    let closing = code
        .chars()
        .take_while(|character| character.is_whitespace() || *character == '}')
        .filter(|character| *character == '}')
        .count();
    for _ in 0..closing {
        contexts.pop();
    }

    let opens = code.chars().filter(|character| *character == '{').count();
    let closes = code.chars().filter(|character| *character == '}').count();
    let net_opens = opens.saturating_sub(closes.saturating_sub(closing));
    for open_index in 0..net_opens {
        let context = if open_index == 0 && is_loop_header(code) {
            Context::Loop
        } else if open_index == 0 && is_conditional_header(code) {
            Context::Conditional
        } else {
            Context::Block
        };
        contexts.push(context);
    }
}

fn is_loop_header(code: &str) -> bool {
    let code = code.trim_start();
    code.starts_with("for (")
        || code.starts_with("for(")
        || code.starts_with("while (")
        || code.starts_with("while(")
}

fn is_conditional_header(code: &str) -> bool {
    let code = code.trim_start();
    code.starts_with("if (")
        || code.starts_with("if(")
        || code.starts_with("else")
        || code.starts_with("switch (")
}

fn strip_line_comment(line: &str) -> &str {
    line.split_once("//").map_or(line, |(code, _)| code)
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

fn identifier_column(source_line: &str, identifier: &str) -> usize {
    source_line
        .find(identifier)
        .map_or(1, |position| position + 1)
}

fn diagnostic(code: &'static str, line: usize, column: usize, message: String) -> Diagnostic {
    Diagnostic {
        code,
        line,
        column,
        message,
    }
}
