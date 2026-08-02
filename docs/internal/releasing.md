# Releasing

A push to `main` calls the release workflow only after every CI job succeeds. The workflow reads the version from `Cargo.toml`, requires `packages/npm/package.json` to match, and releases only when that semantic version has not already been published. An unchanged version makes later pushes safe release no-ops.

For version `0.1.0`, automation creates the immutable release and tag `v0.1.0`, then moves the supported major Action tag `v0` to the same commit. Consumer workflows should use `orYoffe/tsborrow@v0`. Trial workflows may use the active PR branch before the first release, but must be changed to `v0` after the foundation PR is merged and released.

## Before merging a release

1. Update the versions in both `Cargo.toml` and `packages/npm/package.json` when the intended version changes.
2. Confirm the pull request is green and public documentation describes only shipped features.
3. Confirm `orYoffe/homebrew-tap` exists before enabling tap publication.
4. Verify registry ownership for the Cargo crate, npm scope, and Chocolatey package ID.
5. For a major-version change, update documentation and downstream Action references to the new major tag.

## Secrets

| Secret | Enables |
| --- | --- |
| `CARGO_REGISTRY_TOKEN` | crates.io publication. |
| `NPM_TOKEN` | npm publication with provenance. |
| `HOMEBREW_TAP_TOKEN` | Formula commit to the tap repository. |
| `CHOCO_API_KEY` | Chocolatey push. |

Missing secrets skip the relevant publication step; GitHub release artifacts are still built. Artifacts include raw binaries, archives, and SHA-256 files for Linux x64, macOS x64/ARM64, and Windows x64.

Do not reuse an immutable release tag after publishing. Correct a bad release with a new patch version and document any affected install channel. The moving major tag is the only tag that automation force-updates.
