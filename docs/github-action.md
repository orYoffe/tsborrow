# GitHub Action

```yaml
permissions:
  contents: read
  pull-requests: write

steps:
  - uses: actions/checkout@v6
  - uses: dtolnay/rust-toolchain@stable
  - id: tsborrow
    uses: orYoffe/tsborrow@v0
    with:
      path: src
      format: human
```

The Action currently compiles its pinned Rust source, so the runner needs a Rust toolchain.

## Inputs

| Input | Default | Meaning |
| --- | --- | --- |
| `path` | `.` | File or directory to analyze. |
| `format` | `human` | `human`, `json`, or `html`. |
| `output` | empty | Required destination when `format: html`. |
| `comment` | `true` | Add or update the PR results comment. |

## Outputs

`summary` contains the CLI's final result line. It remains available when a checker violation is handled with `continue-on-error`.

## Pull-request comments

On `pull_request` events, the Action finds its stable hidden marker and updates that comment. It performs a second lookup and removes matching duplicates so repeated and overlapping runs converge on one comment per PR. The comment contains status, summary, report details, commit, and workflow link.

Set `pull-requests: write`. Tokens on untrusted fork PRs are commonly read-only; in that case reporting emits a workflow warning while analysis and its exit status remain authoritative. Set `comment: "false"` when another reporting system owns PR feedback.
