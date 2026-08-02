# TypeScript Borrow Checker

An experimental Rust CLI and GitHub Action for explicit ownership and resource-lifetime checks in TypeScript and JavaScript.

## Status

The checker currently uses explicit contracts so findings stay predictable while the AST and control-flow front end evolves. TypeScript values use `Owned<T>` or `Resource<T>`; JavaScript values use an immediately preceding `@owned` or `@resource` JSDoc marker.

The conformance suite covers moves, partial moves, shared and mutable borrows, non-lexical lifetime release, branch behavior, ended borrows, disposal, and resource leaks. It also protects ordinary JavaScript patterns such as callbacks, returned references, and borrows across `await` from false positives.

## Ownership operations

```ts
const file: Resource<FileHandle> = openFile("report.txt");
const reader = borrow(file);       // shared borrow
reader.read();                     // borrow ends at its last use
const writer = borrowMut(file);    // unique mutable borrow
writer.write("complete");
endBorrow(writer);                 // optional explicit early end
dispose(file);                     // resource obligation satisfied
```

- `move(value)` transfers an owned value and invalidates its previous place.
- `borrow(value)` creates a shared borrow; multiple shared borrows may overlap.
- `borrowMut(value)` creates a unique mutable borrow.
- `endBorrow(reference)` explicitly ends a borrow before its inferred last use.
- `dispose(value)` satisfies a `Resource<T>` cleanup obligation.

## Run locally

```sh
cargo run -- check path/to/source
cargo run -- check src --format json
```

A successful run is explicit:

```text
ok: checked 12 source file(s); analyzed 8 ownership contract(s) and 5 borrow(s); no ownership violations found
```

If a project has no `Owned<T>`, `Resource<T>`, `@owned`, or `@resource` contracts yet, the command still succeeds but prints a warning. This prevents a clean baseline from being mistaken for meaningful ownership coverage.

## Installation

These channels are prepared by the tagged-release workflow. Registry commands become available after the first release is published and the corresponding registry credentials are configured.

```sh
# Rust / Cargo
cargo install ts-borrow-checker

# npm
npm install --save-dev @oryoffe/tsborrow
npx tsborrow check src

# Homebrew
brew install orYoffe/tap/tsborrow

# Chocolatey
choco install tsborrow
```

Linux and macOS users can inspect and run the checksum-verifying installer:

```sh
wget https://raw.githubusercontent.com/orYoffe/tsborrow/main/install.sh
sh install.sh
```

Go applications can invoke the same Rust engine through the process client:

```sh
go get github.com/orYoffe/tsborrow/clients/go@v0.1.0
```

The npm and Go packages contain no analyzer implementation; they download or invoke the canonical Rust CLI.

## Use in GitHub Actions

Until release binaries are published, the Action compiles its pinned source checkout with Cargo. Projects using it need a Rust toolchain available on the runner.

```yaml
jobs:
  borrow-check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v6
      - uses: dtolnay/rust-toolchain@stable
      - uses: orYoffe/tsborrow@v0
        with:
          path: src
```

The Action exits non-zero when it finds diagnostics, so it works directly as a CI gate. Repository CI runs the complete passing fixture project and asserts that the complete failing project is rejected.

The Action also exposes its exact final report as the `summary` output. Repository CI asserts the complete JSON summary for known passing and failing programs, while Rust integration tests assert the individual diagnostics. A dependency-free release-mode smoke benchmark runs on every push and pull request so analyzer throughput stays visible without introducing a benchmarking framework.

## Architecture direction

The ownership state model follows Rust's move, aliasing, last-use lifetime, and drop-scope principles. The current explicit-contract front end is conservative and intentionally does not claim full JavaScript/TypeScript soundness. The next major boundary is a real JS/TS AST lowered into a control-flow graph; the golden conformance suite is the compatibility contract for that replacement.

See [the diagnostic audit](docs/check-audit.md) for which rules are core resource checks, which are experimental strict-ownership checks, and which Rust-like rules were removed as unrealistic for ordinary JavaScript.
