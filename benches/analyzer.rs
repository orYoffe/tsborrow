use std::{hint::black_box, time::Instant};

use ts_borrow_checker::analyze_report;

fn main() {
    run("moves", ownership_moves(400), 30);
    run("resource-disposal", resource_disposals(400), 30);
    run("borrow-lifetimes", borrow_lifetimes(150), 20);
}

fn run(name: &str, source: String, iterations: usize) {
    let lines = source.lines().count();
    let started = Instant::now();
    for _ in 0..iterations {
        let report = analyze_report(black_box(&source));
        assert!(
            report.diagnostics.is_empty(),
            "benchmark input produced diagnostics: {:?}",
            report.diagnostics
        );
        black_box(report);
    }
    let elapsed = started.elapsed();
    let micros = elapsed.as_secs_f64() * 1_000_000.0 / iterations as f64;
    let lines_per_second = lines as f64 * iterations as f64 / elapsed.as_secs_f64();
    println!(
        "{name}: {lines} lines, {iterations} iteration(s), {micros:.0} us/iteration, {lines_per_second:.0} lines/s"
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
