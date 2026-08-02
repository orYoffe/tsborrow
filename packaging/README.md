# Distribution packaging

All packages install or invoke the same Rust `tsborrow` binary from a tagged GitHub Release.

- `install.sh` supports inspected `wget`/`curl` installation on Linux and macOS with SHA-256 verification.
- `homebrew/tsborrow.rb.template` is rendered with release checksums and published to `orYoffe/homebrew-tap` when `HOMEBREW_TAP_TOKEN` is configured.
- `chocolatey` templates are rendered, packed, and published when `CHOCO_API_KEY` is configured.
- `packages/npm` downloads the same-version native binary on first execution and verifies its checksum.
- Cargo publishes the Rust crate when `CARGO_REGISTRY_TOKEN` is configured.

The release workflow always builds and validates artifacts. Registry publishing steps are skipped until their corresponding secrets are configured.
