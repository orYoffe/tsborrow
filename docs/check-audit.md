# Diagnostic audit

This audit separates resource-lifetime checks that address real JavaScript and TypeScript failures from strict ownership experiments. A large fixture count is not evidence of value; each diagnostic must correspond to a realistic bug and have a path to low false-positive rates.

## Keep as core resource checks

| Code | Check | Decision | Rationale |
| --- | --- | --- | --- |
| `TSB001` | Resource leaves scope without cleanup | Keep | Leaked handles, sockets, timers, media tracks, and transactions are runtime problems. This needs AST/CFG and API-specific acquisition/cleanup rules before broad use. |
| `TSB002` | Use after explicit disposal | Keep | Calling APIs on closed or disposed resources is a realistic typestate error. |
| `TSB003` | Double disposal | Keep provisionally | Useful for non-idempotent resources, but many `close()` APIs are idempotent. This must become API-configurable rather than universally fatal. |

## Keep only in an explicit strict-ownership profile

| Code | Check | Decision | Rationale |
| --- | --- | --- | --- |
| `E0382` | Use after explicit move, including partial/conditional moves | Experimental | Meaningful only when a project opts into move semantics. JavaScript assignment does not move values. |
| `E0499` | Two overlapping mutable borrows | Experimental | Models Rust aliasing, not ordinary JS object behavior. Useful for opt-in concurrency or invariant-heavy APIs. |
| `E0502` | Shared/mutable borrow conflict | Experimental | Same limitation: this is a chosen project contract, not a TypeScript safety rule. |
| `E0503` | Owner used during mutable borrow | Experimental | Useful only with explicit exclusive-access semantics. |
| `E0505` | Move/dispose while borrowed | Experimental | Relevant when strict ownership or resource leases are explicitly enabled. |
| `TSB005` | Use after `endBorrow` | Experimental | Internally consistent, but `endBorrow` is not yet a production API and should not shape inferred JS behavior. |

## Removed as default errors

| Former code | Check | Reason for removal |
| --- | --- | --- |
| `E0515` | Returning a borrowed object | Returning object references is normal and memory-safe in garbage-collected JS. Resource escape analysis must track ownership transfer instead of rejecting returns. |
| `E0521` | Capturing a borrow in a callback | Closures routinely and safely retain objects. A useful rule must prove teardown can happen before callback execution. |
| `TSB004` | Borrow crossing every `await` | `await` alone does not invalidate a JS object. Future checks should model a concrete close/abort race. |
| `TSB006` | Move appearing syntactically inside a loop | The line scanner cannot know iteration count, reacquisition, breaks, or path feasibility. Defer until real CFG analysis exists. |

The removed cases remain passing regression fixtures. This is intentional: they prevent attractive but incorrect Rust analogies from returning as false positives.

## Release gate

The current analyzer is an explicit-contract prototype. It is not ready to advertise broad JS/TS validation until it uses a real parser, reports the number and kinds of resources tracked, and demonstrates findings on unmodified production code.
