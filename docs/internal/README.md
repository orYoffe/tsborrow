# Internal documentation

These documents describe the repository as it exists and the constraints maintainers must preserve.

- [Architecture](architecture.md): component boundaries, data flow, invariants, and current debt.
- [Diagnostics](diagnostics.md): adding, changing, auditing, and removing checks.
- [Testing and CI](testing.md): fixture contracts, CLI tests, Action tests, clients, duplication, and benchmarks.
- [Security](security.md): untrusted source, HTML output, shell inputs, tokens, downloads, and publishing.
- [Releasing](releasing.md): versioning, artifacts, registries, secrets, and rollback expectations.
- [Roadmap](roadmap.md): migration from the scanner to a generic AST/CFG effect analyzer.

User-facing behavior belongs in the parent [documentation hub](../README.md). When behavior changes, update public and internal documentation in the same pull request.
