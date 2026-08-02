# Diagnostic maintenance

Every diagnostic needs:

1. A realistic runtime failure or explicitly opt-in strict ownership invariant.
2. At least one failing golden fixture with exact code, line, column, and message.
3. Passing fixtures for nearby valid patterns and likely false positives.
4. CLI behavior coverage when output serialization changes.
5. A decision in `docs/check-audit.md` describing confidence and profile.

Core diagnostics should address resource lifetime or typestate problems that can cause runtime errors, leaks, or retained work. Rust analogies that are safe and ordinary in garbage-collected JavaScript belong only in an explicit strict profile.

Do not assign a new code merely because a syntactic pattern is easy to detect. First define the abstract state transition and the execution path that makes the program unsafe. When the frontend cannot establish that path, prefer a documented limitation over a noisy error.

Removing a diagnostic requires converting its former failing cases into passing regression fixtures when the pattern is valid JavaScript or TypeScript. This prevents the same false positive from returning under a different implementation.
