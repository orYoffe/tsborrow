# tsborrow documentation

tsborrow is a Rust CLI and GitHub Action for explicit ownership and lifetime contracts in JavaScript and TypeScript. The current analyzer is an experimental line-oriented implementation; the documented limitations are part of the product contract, not hidden caveats.

## User documentation

- [CLI reference](cli.md): commands, formats, output files, exit codes, and ignored directories.
- [HTML reports](reports.md): navigation, filtering, source context, security, and CI artifacts.
- [GitHub Action](github-action.md): inputs, outputs, PR comments, permissions, and artifact examples.
- [Analysis and diagnostics](analysis.md): contracts, operations, diagnostics, confidence, and known limitations.
- [Installation and integrations](integrations.md): Cargo, npm, Homebrew, Chocolatey, shell installer, and Go.
- [Benchmark budgets](benchmarks.md): required thresholds, configuration, overrides, and CI behavior.
- [Diagnostic audit](check-audit.md): decisions about useful, experimental, and removed checks.

## Feature matrix

| Feature | Available | Notes |
| --- | --- | --- |
| TypeScript and TSX input | Yes | Explicit `Owned<T>` and `Resource<T>` contracts. |
| JavaScript and JSX input | Yes | Explicit `@owned` and `@resource` markers. |
| Human diagnostics | Yes | Compiler-style lines and a final summary. |
| JSON Lines diagnostics | Yes | One diagnostic per line followed by a summary object. |
| Navigable HTML report | Yes | Self-contained file with filtering and source context. |
| GitHub Action CI gate | Yes | Preserves exit status and exposes a summary output. |
| One updating PR comment | Yes | Requires `pull-requests: write`. |
| Go process client | Yes | Invokes the canonical Rust CLI. |
| Release packaging | Prepared | Cargo, npm, Homebrew, Chocolatey and release binaries. |
| Performance budget | Yes | Every benchmark case requires a positive threshold. |
| Full JS/TS AST and CFG | Not yet | Planned replacement for the current scanner. |
| Automatic arbitrary API inference | Not yet | Requires the generic effect architecture in the roadmap. |

Maintainer-facing material is in [docs/internal](internal/README.md).
