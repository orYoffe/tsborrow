use std::{
    collections::{BTreeMap, BTreeSet},
    env, fs,
    hint::black_box,
    path::PathBuf,
    process,
    time::Instant,
};

use ts_borrow_checker::analyze_report;

const DEFAULT_CONFIG: &str = "benches/thresholds.conf";

struct Case {
    name: &'static str,
    source: String,
    iterations: usize,
}

fn main() {
    if let Err(error) = run_benchmarks() {
        eprintln!("benchmark error: {error}");
        process::exit(1);
    }
}

fn run_benchmarks() -> Result<(), String> {
    let cases = vec![
        Case {
            name: "moves",
            source: ownership_moves(400),
            iterations: 30,
        },
        Case {
            name: "resource-disposal",
            source: resource_disposals(400),
            iterations: 30,
        },
        Case {
            name: "borrow-lifetimes",
            source: borrow_lifetimes(150),
            iterations: 20,
        },
    ];
    let (config_path, overrides, help) = arguments()?;
    if help {
        print_usage();
        return Ok(());
    }

    let mut thresholds = thresholds_from(&config_path)?;
    thresholds.extend(overrides);
    validate_thresholds(&cases, &thresholds)?;

    let mut failures = Vec::new();
    for case in cases {
        let measured = measure(&case)?;
        let threshold = thresholds[case.name];
        let status = if measured >= threshold {
            "PASS"
        } else {
            failures.push(format!(
                "{} measured {:.0} lines/s, below its {:.0} lines/s threshold",
                case.name, measured, threshold
            ));
            "FAIL"
        };
        println!(
            "{}: {:.0} lines/s (threshold {:.0}) {status}",
            case.name, measured, threshold
        );
    }

    if failures.is_empty() {
        Ok(())
    } else {
        Err(failures.join("; "))
    }
}

fn arguments() -> Result<(PathBuf, BTreeMap<String, f64>, bool), String> {
    let mut config = PathBuf::from(DEFAULT_CONFIG);
    let mut overrides = BTreeMap::new();
    let mut help = false;
    let mut arguments = env::args().skip(1);
    while let Some(argument) = arguments.next() {
        match argument.as_str() {
            "--config" => {
                config = PathBuf::from(
                    arguments
                        .next()
                        .ok_or_else(|| "--config requires a path".to_owned())?,
                );
            }
            "--threshold" => {
                let value = arguments
                    .next()
                    .ok_or_else(|| "--threshold requires CASE=LINES_PER_SECOND".to_owned())?;
                insert_threshold(&mut overrides, &value, "command line")?;
            }
            "--help" | "-h" => help = true,
            value if value.starts_with("--config=") => {
                config = PathBuf::from(value.trim_start_matches("--config="));
            }
            value if value.starts_with("--threshold=") => {
                insert_threshold(
                    &mut overrides,
                    value.trim_start_matches("--threshold="),
                    "command line",
                )?;
            }
            _ => return Err(format!("unknown argument: {argument}")),
        }
    }
    Ok((config, overrides, help))
}

fn thresholds_from(path: &PathBuf) -> Result<BTreeMap<String, f64>, String> {
    let source = fs::read_to_string(path)
        .map_err(|error| format!("cannot read threshold config {}: {error}", path.display()))?;
    let mut thresholds = BTreeMap::new();
    for (index, line) in source.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        insert_threshold(
            &mut thresholds,
            line,
            &format!("{}:{}", path.display(), index + 1),
        )?;
    }
    Ok(thresholds)
}

fn insert_threshold(
    thresholds: &mut BTreeMap<String, f64>,
    assignment: &str,
    source: &str,
) -> Result<(), String> {
    let (name, raw_threshold) = assignment
        .split_once('=')
        .ok_or_else(|| format!("invalid threshold in {source}: expected CASE=LINES_PER_SECOND"))?;
    let name = name.trim();
    let threshold = raw_threshold
        .trim()
        .parse::<f64>()
        .map_err(|_| format!("invalid threshold for {name} in {source}"))?;
    if name.is_empty() || !threshold.is_finite() || threshold <= 0.0 {
        return Err(format!(
            "thresholds must be named and greater than zero in {source}"
        ));
    }
    thresholds.insert(name.to_owned(), threshold);
    Ok(())
}

fn validate_thresholds(cases: &[Case], thresholds: &BTreeMap<String, f64>) -> Result<(), String> {
    let known = cases.iter().map(|case| case.name).collect::<BTreeSet<_>>();
    let missing = known
        .iter()
        .filter(|name| !thresholds.contains_key(**name))
        .copied()
        .collect::<Vec<_>>();
    let unknown = thresholds
        .keys()
        .filter(|name| !known.contains(name.as_str()))
        .cloned()
        .collect::<Vec<_>>();
    if !missing.is_empty() {
        return Err(format!("missing threshold(s): {}", missing.join(", ")));
    }
    if !unknown.is_empty() {
        return Err(format!("unknown benchmark case(s): {}", unknown.join(", ")));
    }
    Ok(())
}

fn measure(case: &Case) -> Result<f64, String> {
    let lines = case.source.lines().count();
    let started = Instant::now();
    for _ in 0..case.iterations {
        let report = analyze_report(black_box(&case.source));
        if !report.diagnostics.is_empty() {
            return Err(format!(
                "{} input produced diagnostics: {:?}",
                case.name, report.diagnostics
            ));
        }
        black_box(report);
    }
    let elapsed = started.elapsed();
    Ok(lines as f64 * case.iterations as f64 / elapsed.as_secs_f64())
}

fn print_usage() {
    println!(
        "usage: cargo bench --bench analyzer -- [--config PATH] [--threshold CASE=LINES_PER_SECOND]..."
    );
}

fn ownership_moves(count: usize) -> String {
    let mut source = String::new();
    for index in 0..count {
        source.push_str(&format!(
            "const value{index}: Owned<Value> = create();\nconst moved{index} = move(value{index});\n"
        ));
    }
    source
}

fn resource_disposals(count: usize) -> String {
    let mut source = String::new();
    for index in 0..count {
        source.push_str(&format!(
            "const resource{index}: Resource<Handle> = open();\ndispose(resource{index});\n"
        ));
    }
    source
}

fn borrow_lifetimes(count: usize) -> String {
    let mut source = String::new();
    for index in 0..count {
        source.push_str(&format!(
            "const owner{index}: Owned<Value> = create();\nconst view{index} = borrow(owner{index});\nview{index}.read();\n"
        ));
    }
    source
}
