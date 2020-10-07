use criterion::{criterion_group, criterion_main, Criterion};

use abin_benchmark::{
    benchmark_test_set, run_benchmark, BenchStr, BytesBenchStr, SStrBenchStr, StdLibArcStrOnly,
    StdLibOptimized, StdLibStringOnly, StrBenchStr,
};

fn perform<TBenchStr: BenchStr>() {
    run_benchmark::<TBenchStr>(2, benchmark_test_set());
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Implementation: abin (SStr)", |b| {
        b.iter(|| perform::<SStrBenchStr>())
    });
    c.bench_function("Implementation: abin (Str)", |b| {
        b.iter(|| perform::<StrBenchStr>())
    });
    c.bench_function("Implementation: StdLibOptimized", |b| {
        b.iter(|| perform::<StdLibOptimized>())
    });
    c.bench_function("Implementation: BytesBenchStr", |b| {
        b.iter(|| perform::<BytesBenchStr>())
    });
    c.bench_function("Implementation: StdLibStringOnly", |b| {
        b.iter(|| perform::<StdLibStringOnly>())
    });
    c.bench_function("Implementation: StdLibArcStrOnly", |b| {
        b.iter(|| perform::<StdLibArcStrOnly>())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
