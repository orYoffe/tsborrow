# Security model

tsborrow processes untrusted repository paths and source text in CI.

## CLI and reports

- CLI arguments are parsed as values and never executed.
- JSON paths and messages escape quotes, backslashes, newlines, tabs, and control characters.
- HTML paths, source, codes, and messages are escaped before insertion.
- HTML is self-contained and carries a restrictive Content Security Policy.
- HTML output requires an explicit path, avoiding an unexpected default overwrite.

## GitHub Action

- Action inputs enter shell commands through environment variables and Bash arrays, not expression interpolation into executable text.
- PR comment report lines are rendered as indented code, preventing diagnostic paths from injecting Markdown structure.
- The reporter uses a stable marker, updates one comment, and removes marker duplicates.
- Comment permission failures warn and return success; the final step independently restores the checker exit code.
- Fork PRs must not use `pull_request_target` to execute untrusted checked-out code.

## Distribution

- The npm launcher and shell installer verify release SHA-256 checksums.
- Registry credentials exist only as release workflow secrets.
- Release assets are produced by tagged builds; version/tag equality is checked before Unix artifacts are published.

Any future HTML source-link feature must avoid `file://` links and path traversal. Any external contract pack must be data-only or executed in a sandbox.
