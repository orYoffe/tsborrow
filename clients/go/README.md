# tsborrow Go client

This package invokes the `tsborrow` Rust CLI and parses its JSON-lines output. It intentionally contains no analysis logic.

```go
client := tsborrow.New()
result, err := client.Check(context.Background(), "./src")
```

Install the CLI separately with Cargo, npm, Homebrew, Chocolatey, or a GitHub Release installer. Set `TSBORROW_BINARY` to use a non-default executable path.
