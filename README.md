# TypeScript Borrow Checker

An experimental Rust CLI and GitHub Action for explicit ownership and resource-lifetime checks in TypeScript and JavaScript.

## Status

The first contract is intentionally small: values declared as `Owned<T>` in TypeScript, or with an immediately preceding `@owned` JSDoc marker in JavaScript, are checked for `move(value)`, `dispose(value)`, use-after-move, use-after-disposal, and double disposal.

## Run locally

```sh
cargo run -- check path/to/source
cargo run -- check src --format json
```

## Use in GitHub Actions

Until release binaries are published, the Action compiles its pinned source checkout with Cargo. Projects using it need a Rust toolchain available on the runner.

```yaml
jobs:
  borrow-check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: YOUR_ORG/ts-borrow-checker@v0
        with:
          path: src
```

The Action exits non-zero when it finds diagnostics, so it works directly as a CI gate.
