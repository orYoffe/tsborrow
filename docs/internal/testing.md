# Testing and CI

## Test layers

- `tests/contracts.rs` recursively executes every JS/TS fixture and compares exact golden output. Passing fixtures contain `OK`.
- `scripts/check-fixtures-with-tsc.sh` runs the complete fixture corpus through strict `tsc --noEmit`. Every tsborrow-negative case must remain valid TypeScript/JavaScript, so the analyzer cannot claim a compiler error as its own finding. A deliberately invalid control must fail with `TS2322`, proving the compiler gate is active.
- `tests/cli.rs` validates exit codes, summaries, JSON Lines, warnings, HTML file generation, and required arguments.
- Unit tests beside the HTML renderer validate navigation and escaping.
- `tests/action-comment.sh` uses a fake GitHub CLI to prove create, update, and duplicate removal behavior.
- `clients/go` tests JSON Lines decoding and is checked with `gofmt`, `go vet`, and `go test`.
- The Action contract runs known passing/failing programs and compares exact summary fields.

## CI jobs

| Job | Gate |
| --- | --- |
| `quality` | rustfmt, check, Clippy, release compilation, Cargo packaging, and shell syntax. |
| `typescript-baseline` | Strict TypeScript/JavaScript type checking, plus a known-invalid compiler control. |
| `test` | Rust unit and integration tests. |
| `benchmark` | Missing-threshold rejection and release throughput budgets. |
| `duplicates` | jscpd threshold across production Rust, Go, and npm launcher code. |
| `clients` | Go and npm validation. |
| `action-contract` | Composite Action behavior, HTML generation, exact summaries, and PR reporting. |

Both `push` and `pull_request` run CI. The duplicate executions intentionally validate both triggers. A failing fixture Action step uses `continue-on-error` only so the next assertion can prove the exact failure outcome and summary.

`scripts/check-doc-links.py` validates relative Markdown links from the root README and public/internal documentation hubs without network access.

## Review rule

Never update a golden file until the semantic change is explained. A negative tsborrow fixture must pass the TypeScript baseline before its analyzer output is accepted. Never lower a benchmark threshold without multiple measurements and an explanation of the expected regression.
