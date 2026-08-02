# HTML reports

HTML output is a self-contained report intended for local review and CI artifacts.

```sh
tsborrow check src --format html --output tsborrow-report.html
```

The report contains:

- Overall file, diagnostic, ownership-contract, and borrow counts.
- A sidebar linking to every analyzed file.
- Per-file diagnostic counts and clean-file status.
- Filtering by path, diagnostic code, or diagnostic message.
- Diagnostic code, location, message, and nearby source lines.
- Responsive styling for desktop and narrow screens.

No external scripts, fonts, stylesheets, or network requests are used. Paths, messages, and source text are HTML-escaped, and a restrictive Content Security Policy is embedded. The small inline script only filters already-rendered file sections.

HTML generation does not change result semantics: the CLI writes the report and exits `1` when violations exist.

## GitHub artifact example

```yaml
- uses: orYoffe/tsborrow@v0
  with:
    path: src
    format: html
    output: tsborrow-report.html
- uses: actions/upload-artifact@v4
  if: always()
  with:
    name: tsborrow-report
    path: tsborrow-report.html
```

Use `if: always()` because a report containing violations is intentionally produced by a failing Action step.
