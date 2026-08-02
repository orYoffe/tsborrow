# Benchmark budgets

The dependency-free benchmark executable measures moves, resource disposal, and borrow-lifetime analysis in Rust's optimized bench profile.

```sh
cargo bench --bench analyzer
```

Every named case must have a positive lines-per-second threshold in `benches/thresholds.conf`. Missing, invalid, or unknown thresholds exit `1`. Measured throughput below a threshold also exits `1`.

Use another complete configuration:

```sh
cargo bench --bench analyzer -- --config path/to/thresholds.conf
```

Override one or more entries after loading the configuration:

```sh
cargo bench --bench analyzer -- \
  --threshold moves=600000 \
  --threshold borrow-lifetimes=40000
```

CI first proves that an empty configuration fails specifically because thresholds are missing, then enforces the checked-in budgets. Thresholds should be changed in a reviewable commit supported by repeated runner measurements, not merely lowered to make a regression green.
