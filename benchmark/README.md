# abin-bench

Benchmark for the abin crate; this crate is not useful on its own.

The benchmarks have an own library and are not integrated in the `abin` create: I need some shared functionality (e.g. the bench string) for the benchmark (real benchmark) and for the memory-benchmark (actually a test). The benchmarks seem to be unable to access things from `tests` and I don't want the shared functionality to be inside the `abin` crate.