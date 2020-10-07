use std::alloc::System;

use stats_alloc::{Region, StatsAlloc, INSTRUMENTED_SYSTEM};

use abin_benchmark::{
    benchmark_test_set, run_benchmark_single_threaded, BenchStr, BytesBenchStr, SStrBenchStr,
    StdLibArcStrOnly, StdLibOptimized, StdLibStringOnly,
};

#[global_allocator]
static GLOBAL: &StatsAlloc<System> = &INSTRUMENTED_SYSTEM;

/// Makes sure that the `abin` implementation is usually(*) better than the other
/// implementations. (*): See exceptions below.
#[test]
fn test_abin_is_better_or_equal_everywhere() {
    compare::<StdLibArcStrOnly, SStrBenchStr>(false, 0, "StdLibArcStrOnly");
    compare::<StdLibStringOnly, SStrBenchStr>(false, 0, "StdLibStringOnly");
    // exception: depending on the platform, the 'StdLibOptimized' is a bit better regarding to the number of re-allocations.
    compare::<StdLibOptimized, SStrBenchStr>(false, 4, "StdLibOptimized");
    // exception: 'bytes_reallocated' is worse here.
    compare::<BytesBenchStr, SStrBenchStr>(true, 0, "BytesBenchStr");
}

fn compare<TBenchStr: BenchStr, TBetterBenchStr: BenchStr>(
    ignore_bytes_reallocated: bool,
    number_of_additional_re_allocations_allowed: usize,
    vs_name: &str,
) {
    let change = {
        let region = Region::new(GLOBAL);
        run_benchmark_single_threaded::<TBenchStr>(benchmark_test_set());
        region.change()
    };
    let change_better = {
        let region = Region::new(GLOBAL);
        run_benchmark_single_threaded::<TBetterBenchStr>(benchmark_test_set());
        region.change()
    };

    //println!("ABIN: {:?}", change_better);
    //println!("OTHER: {:?}", change);

    // the "better" should be better (or at least equal) everywhere.
    assert!(
        change.reallocations + number_of_additional_re_allocations_allowed
            >= change_better.reallocations,
        "Expected to be better in number of re-allocations: {:?} ({}) vs. {:?} (better)",
        change,
        vs_name,
        change_better
    );
    assert!(
        change.allocations >= change_better.allocations,
        "Expected to be better in number of allocations: {:?} ({}) vs. {:?} (better)",
        change,
        vs_name,
        change_better
    );
    assert!(
        change.deallocations >= change_better.deallocations,
        "Expected to be better in number of de-allocations: {:?} ({}) vs. {:?} (better)",
        change,
        vs_name,
        change_better
    );
    if !ignore_bytes_reallocated {
        assert!(
            change.bytes_reallocated >= change_better.bytes_reallocated,
            "Expected to be better in bytes re-allocated: {:?} ({}) vs. {:?} (better)",
            change,
            vs_name,
            change_better
        );
    }
    assert!(
        change.bytes_allocated >= change_better.bytes_allocated,
        "Expected to be better in bytes allocated: {:?} ({}) vs. {:?} (better)",
        change,
        vs_name,
        change_better
    );
    assert!(
        change.bytes_deallocated >= change_better.bytes_deallocated,
        "Expected to be better in bytes de-allocated: {:?} ({}) vs. {:?} (better)",
        change,
        vs_name,
        change_better
    );
}
