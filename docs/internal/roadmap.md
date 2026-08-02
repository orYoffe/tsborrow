# Generic analyzer roadmap

The target is a generic effect-and-lifetime analyzer, not a growing list of hard-coded acquisition/release method pairs.

## Intended pipeline

1. Parse JS, JSX, TS, and TSX with a real Rust frontend.
2. Resolve lexical symbols, references, imports, and places.
3. Lower functions into control-flow graphs including exceptions and `finally`.
4. Interpret generic effects: acquire, release, transfer, borrow, escape, register, unregister, spawn, retain, and suspend.
5. Compute path-sensitive local state with bounded loop fixed points.
6. Infer and cache interprocedural function summaries.
7. Load standardized, declared, inferred, and project-supplied protocol contracts.
8. Report acquisition-to-failure traces and confidence.

Oxc is the leading Rust-native frontend candidate because it provides JS/TS parsing, semantic symbols/references, and control-flow graph infrastructure. TypeScript compiler enrichment may later provide authoritative type and package-symbol information without making ordinary JavaScript analysis dependent on types.

## Delivery slices

- Replace line parsing while preserving every existing golden fixture.
- Introduce the effect IR without adding concrete browser APIs.
- Add transfers through return values, parameters, properties, promises, and wrapper functions.
- Add registrations and loop-backedge accumulation.
- Add spawned/aggregate obligations such as collections of child resources.
- Validate findings against unmodified real projects before promoting diagnostics to the default profile.

The current explicit operations remain a useful test language during migration. Compatibility is defined by outcomes and traces, not by preserving the scanner's internal structure.
