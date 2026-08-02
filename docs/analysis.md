# Analysis and diagnostics

## Contracts

TypeScript declarations opt in with `Owned<T>` or `Resource<T>`. JavaScript declarations use an immediately preceding `@owned` or `@resource` marker.

## Operations

| Operation | Meaning |
| --- | --- |
| `move(value)` | Transfer ownership and invalidate the previous place. |
| `borrow(value)` | Create shared temporary access. |
| `borrowMut(value)` | Create exclusive mutable access. |
| `endBorrow(reference)` | Explicitly end temporary access. |
| `dispose(value)` | Satisfy a resource cleanup obligation. |

The analyzer tracks whole and partial places, last borrow use, conservative conditional moves, disposal state, and resources still live at the end of analysis.

## Diagnostic groups

- Core resource diagnostics: `TSB001` resource leak, `TSB002` use after disposal, and provisional `TSB003` double disposal.
- Experimental strict-ownership diagnostics: `E0382`, `E0499`, `E0502`, `E0503`, `E0505`, and `TSB005`.
- Removed false-positive-prone rules: generic return escape, callback capture, every borrow across `await`, and every move inside a loop.

See the [diagnostic audit](check-audit.md) for the rationale behind each decision.

## Current boundary

The implementation is a line-oriented explicit-contract prototype. It does not yet provide full parsing, type resolution, alias analysis, a real control-flow graph, interprocedural summaries, or automatic arbitrary resource discovery. Results must not be described as sound validation of unannotated JavaScript or TypeScript.
