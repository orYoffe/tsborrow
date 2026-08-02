use std::{env, fs, path::Path};

use ts_borrow_checker::analyze_report;

fn main() {
    let mut arguments = env::args().skip(1);
    let command = arguments.next().unwrap_or_else(|| "check".to_owned());
    if command != "check" {
        eprintln!("usage: tsborrow check <path> [--format human|json]");
        std::process::exit(2);
    }
    let Some(path) = arguments.next() else {
        eprintln!("error: a file or directory path is required");
        std::process::exit(2);
    };
    let remaining = arguments.collect::<Vec<_>>();
    let json = remaining.iter().enumerate().any(|(index, argument)| {
        argument == "--format=json"
            || argument == "--format"
                && remaining
                    .get(index + 1)
                    .is_some_and(|value| value == "json")
    });
    let files = source_files(Path::new(&path));
    if files.is_empty() {
        fail(&format!(
            "no JavaScript or TypeScript source files found in {path}"
        ));
    }
    let file_count = files.len();
    let mut found = 0;
    let mut tracked_owners = 0;
    let mut tracked_borrows = 0;
    for file in files {
        let source = fs::read_to_string(&file)
            .unwrap_or_else(|error| fail(&format!("cannot read {}: {error}", file.display())));
        let report = analyze_report(&source);
        tracked_owners += report.tracked_owners;
        tracked_borrows += report.tracked_borrows;
        for diagnostic in report.diagnostics {
            found += 1;
            if json {
                println!(
                    "{{\"file\":\"{}\",\"code\":\"{}\",\"line\":{},\"column\":{},\"message\":\"{}\"}}",
                    json_escape(&file.to_string_lossy()),
                    diagnostic.code,
                    diagnostic.line,
                    diagnostic.column,
                    json_escape(&diagnostic.message)
                );
            } else {
                println!(
                    "{}:{}:{}: error[{}]: {}",
                    file.display(),
                    diagnostic.line,
                    diagnostic.column,
                    diagnostic.code,
                    diagnostic.message
                );
            }
        }
    }
    if found > 0 {
        std::process::exit(1);
    }
    if tracked_owners == 0 {
        let message = format!(
            "checked {file_count} source file(s), but found no ownership contracts; add Owned<T>, Resource<T>, @owned, or @resource"
        );
        if env::var_os("GITHUB_ACTIONS").is_some() {
            eprintln!("::warning title=No ownership contracts::{message}");
        } else {
            eprintln!("warning: {message}");
        }
    }
    if json {
        println!(
            "{{\"status\":\"ok\",\"files\":{file_count},\"diagnostics\":0,\"trackedOwners\":{tracked_owners},\"trackedBorrows\":{tracked_borrows}}}"
        );
    } else {
        println!(
            "ok: checked {file_count} source file(s); analyzed {tracked_owners} ownership contract(s) and {tracked_borrows} borrow(s); no ownership violations found"
        );
    }
}

fn source_files(path: &Path) -> Vec<std::path::PathBuf> {
    if path.is_file() {
        return is_source(path)
            .then(|| path.to_owned())
            .into_iter()
            .collect();
    }
    let entries = fs::read_dir(path)
        .unwrap_or_else(|error| fail(&format!("cannot list {}: {error}", path.display())));
    entries
        .filter_map(Result::ok)
        .filter(|entry| !is_ignored_directory(&entry.path()))
        .flat_map(|entry| source_files(&entry.path()))
        .collect()
}

fn is_ignored_directory(path: &Path) -> bool {
    path.is_dir()
        && matches!(
            path.file_name().and_then(|name| name.to_str()),
            Some(".git" | "node_modules" | "target")
        )
}

fn is_source(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|extension| extension.to_str()),
        Some("ts" | "tsx" | "js" | "jsx")
    )
}

fn fail(message: &str) -> ! {
    eprintln!("error: {message}");
    std::process::exit(2)
}

fn json_escape(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for character in value.chars() {
        match character {
            '"' => escaped.push_str("\\\""),
            '\\' => escaped.push_str("\\\\"),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            character if character.is_control() => {
                use std::fmt::Write;
                let _ = write!(escaped, "\\u{:04x}", character as u32);
            }
            character => escaped.push(character),
        }
    }
    escaped
}
