use std::{env, fs, path::Path};

use ts_borrow_checker::analyze;

fn main() {
    let mut arguments = env::args().skip(1);
    let command = arguments.next().unwrap_or_else(|| "check".to_owned());
    if command != "check" {
        eprintln!("usage: ts-borrow-checker check <path> [--format human|json]");
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
    let mut found = 0;
    for file in files {
        let source = fs::read_to_string(&file)
            .unwrap_or_else(|error| fail(&format!("cannot read {}: {error}", file.display())));
        for diagnostic in analyze(&source) {
            found += 1;
            if json {
                println!(
                    "{{\"file\":\"{}\",\"code\":\"{}\",\"line\":{},\"column\":{},\"message\":\"{}\"}}",
                    file.display(),
                    diagnostic.code,
                    diagnostic.line,
                    diagnostic.column,
                    diagnostic.message
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
