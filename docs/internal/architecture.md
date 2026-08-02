# Architecture

## Components

| Component | Location | Responsibility |
| --- | --- | --- |
| Analyzer library | `src/lib.rs` | Explicit ownership state, borrow collection, diagnostics, and adoption counts. |
| HTML renderer | `src/html.rs` | Pure self-contained report rendering with no filesystem access. |
| CLI | `src/main.rs` | Arguments, file discovery, aggregation, output formats, and exit status. |
| GitHub Action | `action.yml` | Run the pinned CLI, expose summary, update PR comment, and preserve status. |
| PR reporter | `scripts/upsert-pr-comment.sh` | Create/update/deduplicate the marker comment. |
| Go client | `clients/go` | Process invocation and JSON Lines decoding. |
| npm launcher | `packages/npm` | Platform binary download, checksum verification, caching, and execution. |
| Packaging | `packaging` | Homebrew and Chocolatey release templates. |

## Invariants

- Analyzer logic has one implementation: Rust. Clients invoke or download it.
- Human, JSON, and HTML use the same collected `AnalysisReport` values.
- A report is emitted before exit code `1` so failures remain inspectable.
- JSON's final line is always a summary after a completed analysis.
- HTML rendering is pure and escapes every path, message, code, and source fragment.
- The Action comment step cannot replace the analysis exit status.
- Every benchmark case has a reviewed positive threshold.

## Current debt

The analyzer discovers syntax with string operations. Scope, aliases, branches, and last use are approximations. Do not grow a large concrete-API rule registry on top of this frontend. The compatibility target is the fixture suite; the implementation target is the generic effect architecture in the roadmap.
