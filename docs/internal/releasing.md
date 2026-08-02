# Releasing

The release workflow runs for semantic version tags such as `v0.1.0`.

## Before tagging

1. Merge a green PR and confirm the worktree version in `Cargo.toml` matches the intended tag.
2. Update `packages/npm/package.json` when preparing the release commit; the workflow also verifies/sets the tag version before publishing.
3. Confirm public documentation describes only shipped features.
4. Confirm `orYoffe/homebrew-tap` exists before enabling tap publication.
5. Verify registry ownership for the Cargo crate, npm scope, and Chocolatey package ID.

## Secrets

| Secret | Enables |
| --- | --- |
| `CARGO_REGISTRY_TOKEN` | crates.io publication. |
| `NPM_TOKEN` | npm publication with provenance. |
| `HOMEBREW_TAP_TOKEN` | Formula commit to the tap repository. |
| `CHOCO_API_KEY` | Chocolatey push. |

Missing secrets skip the relevant publication step; GitHub release artifacts are still built. Artifacts include raw binaries, archives, and SHA-256 files for Linux x64, macOS x64/ARM64, and Windows x64.

Do not reuse a tag after publishing. Correct a bad release with a new patch version and document any affected install channel.
