# Installation and integrations

The tagged release workflow prepares one canonical Rust binary for every distribution channel.

```sh
cargo install ts-borrow-checker
npm install --save-dev @oryoffe/tsborrow
brew install orYoffe/tap/tsborrow
choco install tsborrow
```

Linux and macOS can use the checksum-verifying installer:

```sh
wget https://raw.githubusercontent.com/orYoffe/tsborrow/main/install.sh
sh install.sh
```

These registry commands become available after the first tagged release and registry credentials are configured.

## npm

`@oryoffe/tsborrow` selects the current platform, downloads the same-version GitHub Release binary on first use, verifies SHA-256, and caches it. `TSBORROW_BINARY` selects an existing binary and `TSBORROW_CACHE_DIR` changes the cache.

## Go

```sh
go get github.com/orYoffe/tsborrow/clients/go@v0.1.0
```

The Go package invokes `tsborrow check --format json`, parses JSON Lines, and returns `ViolationsError` for analysis exit code `1`. `TSBORROW_BINARY` selects a non-default executable. It intentionally contains no duplicate analyzer logic.

## Homebrew and Chocolatey

Release jobs render package templates using artifact checksums. Publishing is conditional on `HOMEBREW_TAP_TOKEN` and `CHOCO_API_KEY`; see [internal release documentation](internal/releasing.md).
