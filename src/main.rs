use std::{
    env, fs,
    path::{Path, PathBuf},
};

use ts_borrow_checker::{
    AnalysisReport, analyze_report,
    html::{HtmlFile, HtmlReport, render},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OutputFormat {
    Human,
    Json,
    Html,
}

struct Options {
    path: PathBuf,
    format: OutputFormat,
    output: Option<PathBuf>,
}

struct CheckedFile {
    path: PathBuf,
    source: String,
    report: AnalysisReport,
}

fn main() {
    let options = parse_options(env::args().skip(1).collect())
        .unwrap_or_else(|error| fail(&format!("{error}\n\n{}", usage())));
    let mut files = source_files(&options.path);
    files.sort();
    if files.is_empty() {
        fail(&format!(
            "no JavaScript or TypeScript source files found in {}",
            options.path.display()
        ));
    }
    let checked = files
        .into_iter()
        .map(|path| {
            let source = fs::read_to_string(&path)
                .unwrap_or_else(|error| fail(&format!("cannot read {}: {error}", path.display())));
            let report = analyze_report(&source);
            CheckedFile {
                path,
                source,
                report,
            }
        })
        .collect::<Vec<_>>();
    let file_count = checked.len();
    let diagnostics = checked
        .iter()
        .map(|file| file.report.diagnostics.len())
        .sum::<usize>();
    let tracked_owners = checked
        .iter()
        .map(|file| file.report.tracked_owners)
        .sum::<usize>();
    let tracked_borrows = checked
        .iter()
        .map(|file| file.report.tracked_borrows)
        .sum::<usize>();

    match options.format {
        OutputFormat::Human => print_human(&checked),
        OutputFormat::Json => print_json(&checked),
        OutputFormat::Html => write_html(
            &checked,
            options
                .output
                .as_deref()
                .expect("HTML output was validated"),
            tracked_owners,
            tracked_borrows,
        ),
    }

    if diagnostics > 0 {
        match options.format {
            OutputFormat::Human => println!(
                "error: checked {file_count} source file(s); found {diagnostics} ownership violation(s) across {tracked_owners} ownership contract(s) and {tracked_borrows} borrow(s)"
            ),
            OutputFormat::Json => println!(
                "{{\"status\":\"violations\",\"files\":{file_count},\"diagnostics\":{diagnostics},\"trackedOwners\":{tracked_owners},\"trackedBorrows\":{tracked_borrows}}}"
            ),
            OutputFormat::Html => println!(
                "html report: checked {file_count} source file(s); found {diagnostics} ownership violation(s)"
            ),
        }
        std::process::exit(1);
    }
    if tracked_owners == 0 {
        warn_no_contracts(file_count);
    }
    match options.format {
        OutputFormat::Human => println!(
            "ok: checked {file_count} source file(s); analyzed {tracked_owners} ownership contract(s) and {tracked_borrows} borrow(s); no ownership violations found"
        ),
        OutputFormat::Json => println!(
            "{{\"status\":\"ok\",\"files\":{file_count},\"diagnostics\":0,\"trackedOwners\":{tracked_owners},\"trackedBorrows\":{tracked_borrows}}}"
        ),
        OutputFormat::Html => println!(
            "html report: checked {file_count} source file(s); no ownership violations found"
        ),
    }
}

fn parse_options(arguments: Vec<String>) -> Result<Options, String> {
    let mut arguments = arguments.into_iter();
    let command = arguments.next().unwrap_or_else(|| "check".to_owned());
    if command == "--help" || command == "-h" {
        return Err(usage().to_owned());
    }
    if command != "check" {
        return Err(format!("unknown command: {command}"));
    }
    let path = arguments
        .next()
        .filter(|argument| !argument.starts_with('-'))
        .ok_or_else(|| "a file or directory path is required".to_owned())?;
    let mut format = OutputFormat::Human;
    let mut output = None;
    while let Some(argument) = arguments.next() {
        match argument.as_str() {
            "--format" => {
                format = parse_format(
                    &arguments
                        .next()
                        .ok_or_else(|| "--format requires human, json, or html".to_owned())?,
                )?;
            }
            "--output" => {
                output = Some(PathBuf::from(
                    arguments
                        .next()
                        .ok_or_else(|| "--output requires a file path".to_owned())?,
                ));
            }
            value if value.starts_with("--format=") => {
                format = parse_format(value.trim_start_matches("--format="))?;
            }
            value if value.starts_with("--output=") => {
                output = Some(PathBuf::from(value.trim_start_matches("--output=")));
            }
            "--help" | "-h" => return Err(usage().to_owned()),
            _ => return Err(format!("unknown argument: {argument}")),
        }
    }
    if format == OutputFormat::Html && output.is_none() {
        return Err("--format html requires --output <report.html>".to_owned());
    }
    if format != OutputFormat::Html && output.is_some() {
        return Err("--output is only supported with --format html".to_owned());
    }
    Ok(Options {
        path: PathBuf::from(path),
        format,
        output,
    })
}

fn parse_format(value: &str) -> Result<OutputFormat, String> {
    match value {
        "human" => Ok(OutputFormat::Human),
        "json" => Ok(OutputFormat::Json),
        "html" => Ok(OutputFormat::Html),
        _ => Err(format!("unsupported output format: {value}")),
    }
}

fn print_human(files: &[CheckedFile]) {
    for file in files {
        for diagnostic in &file.report.diagnostics {
            println!(
                "{}:{}:{}: error[{}]: {}",
                file.path.display(),
                diagnostic.line,
                diagnostic.column,
                diagnostic.code,
                diagnostic.message
            );
        }
    }
}

fn print_json(files: &[CheckedFile]) {
    for file in files {
        for diagnostic in &file.report.diagnostics {
            println!(
                "{{\"file\":\"{}\",\"code\":\"{}\",\"line\":{},\"column\":{},\"message\":\"{}\"}}",
                json_escape(&file.path.to_string_lossy()),
                diagnostic.code,
                diagnostic.line,
                diagnostic.column,
                json_escape(&diagnostic.message)
            );
        }
    }
}

fn write_html(files: &[CheckedFile], output: &Path, tracked_owners: usize, tracked_borrows: usize) {
    let html_files = files
        .iter()
        .map(|file| HtmlFile {
            path: &file.path,
            source: &file.source,
            diagnostics: &file.report.diagnostics,
        })
        .collect::<Vec<_>>();
    let contents = render(&HtmlReport {
        files: &html_files,
        tracked_owners,
        tracked_borrows,
    });
    fs::write(output, contents)
        .unwrap_or_else(|error| fail(&format!("cannot write {}: {error}", output.display())));
    println!("wrote HTML report to {}", output.display());
}

fn warn_no_contracts(file_count: usize) {
    let message = format!(
        "checked {file_count} source file(s), but found no ownership contracts; add Owned<T>, Resource<T>, @owned, or @resource"
    );
    if env::var_os("GITHUB_ACTIONS").is_some() {
        eprintln!("::warning title=No ownership contracts::{message}");
    } else {
        eprintln!("warning: {message}");
    }
}

fn source_files(path: &Path) -> Vec<PathBuf> {
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

fn usage() -> &'static str {
    "usage: tsborrow check <path> [--format human|json|html] [--output report.html]"
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
