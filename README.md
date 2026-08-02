# TypeScript Borrow Checker

An experimental Rust CLI and GitHub Action for explicit ownership and resource-lifetime checks in TypeScript and JavaScript.

## Status

The checker currently uses explicit contracts so findings stay predictable while the AST and control-flow front end evolves. TypeScript values use `Owned<T>` or `Resource<T>`; JavaScript values use an immediately preceding `@owned` or `@resource` JSDoc marker.

The conformance suite covers moves, partial moves, shared and mutable borrows, non-lexical lifetime release, branch and loop behavior, async suspension, callback/return escapes, ended borrows, disposal, and resource leaks.

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
ok: checked 12 source file(s); no ownership violations found
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

The Action exits non-zero when it finds diagnostics, so it works directly as a CI gate. Repository CI runs the complete passing fixture project and asserts that the complete failing project is rejected.

## Architecture direction

The ownership state model follows Rust's move, aliasing, last-use lifetime, and drop-scope principles. The current explicit-contract front end is conservative and intentionally does not claim full JavaScript/TypeScript soundness. The next major boundary is a real JS/TS AST lowered into a control-flow graph; the golden conformance suite is the compatibility contract for that replacement.
