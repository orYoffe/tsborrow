# CLI reference

## Check source

```sh
tsborrow check <path> [--format human|json|html] [--output report.html]
```

`<path>` may be one JS, JSX, TS, or TSX file, or a directory. Directory traversal skips `.git`, `node_modules`, and `target`. Files are sorted before analysis so reports remain deterministic.

## Formats

### Human

Human output is the default. Each violation uses `path:line:column: error[CODE]: message`, followed by an explicit success or failure summary.

### JSON Lines

```sh
tsborrow check src --format json
```

Each diagnostic is one JSON object. The final line is always a summary with `status`, `files`, `diagnostics`, `trackedOwners`, and `trackedBorrows`. Consumers should parse line by line rather than treating the stream as one JSON array.

### HTML

```sh
tsborrow check src --format html --output tsborrow-report.html
```

HTML requires an explicit output path and writes the report before returning the analysis exit status. This means a failing analysis still leaves a report that can be inspected or uploaded.

## Exit codes

| Code | Meaning |
| --- | --- |
| `0` | Analysis completed with no violations. |
| `1` | Analysis completed and found one or more violations. |
| `2` | Invocation, input, read, or output error. |

A project with no ownership contracts currently exits `0` with a warning. The summary reports zero tracked owners so CI and clients can distinguish no adoption from meaningful coverage.
