# @oryoffe/tsborrow

Node launcher for the native `tsborrow` CLI.

```sh
npm install --save-dev @oryoffe/tsborrow
npx tsborrow check src
```

The package downloads the matching binary from the same-version GitHub Release on first use and verifies its SHA-256 checksum. Set `TSBORROW_BINARY` to use an existing installation or `TSBORROW_CACHE_DIR` to choose the download cache.
