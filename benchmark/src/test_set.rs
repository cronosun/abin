const SRC_EMPTY: &str = "";
const SRC_SHORT1: &str = "a";
const SRC_SHORT2: &str = "Hello";
const SRC_SHORT3: &str = "Hello, World!";
const SRC_SHORT4: &str = "Some";
const SRC_SHORT5: &str = "text";
const SRC_SHORT6: &str = "end!";
const SRC_MEDIUM1: &str = "In order to generate plots, you must have gnuplot installed. See the \
gnuplot website for installation instructions. See Compatibility Policy for details on the minimum \
supported Rust version.";
const SRC_MEDIUM2: &str = "The primary goal of Criterion.rs is to provide a powerful and \
statistically rigorous tool for measuring the performance of code, preventing performance \
regressions and accurately measuring optimizations. Additionally, it should be as programmer-\
friendly as possible and make it easy to create reliable, useful benchmarks, even for programmers \
without an advanced background in statistics.";
const SRC_LARGE1: &str = include_str!("LARGE_STR1.txt");
const SRC_LARGE2: &str = include_str!("LARGE_STR2.txt");
const SRC_HUGE1: &str = include_str!("HUGE_STR1.txt");
const SRC_HUGE2: &str = include_str!("HUGE_STR2.txt");

const TEST_SET: &[&str] = &[
    SRC_EMPTY,
    SRC_SHORT1,
    SRC_SHORT2,
    SRC_SHORT3,
    SRC_SHORT4,
    SRC_SHORT5,
    SRC_SHORT6,
    SRC_MEDIUM1,
    SRC_MEDIUM2,
    SRC_LARGE1,
    SRC_LARGE2,
    SRC_HUGE1,
    SRC_HUGE2,
];

pub fn benchmark_test_set() -> &'static [&'static str] {
    TEST_SET
}
