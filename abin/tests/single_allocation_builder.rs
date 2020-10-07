use std::alloc::System;

use stats_alloc::{StatsAlloc, INSTRUMENTED_SYSTEM};

use abin::{Bin, BinSegment, NewStr, StrBuilder, StrFactory, StrSegment};
use utils::*;

#[global_allocator]
static GLOBAL: &StatsAlloc<System> = &INSTRUMENTED_SYSTEM;

const STACK_MAX_LEN: usize = 3 * core::mem::size_of::<usize>() - 1;

pub mod utils;

/// Make sure there's just one single allocation (and no re-allocation) using a builder.
///
/// Note:
/// single-allocation using a builder is not guaranteed: If there are too many items, there's
/// two allocations (one for the list of segments and one for the actual binary). This
/// limit is (in the current implementation) set to 12 segments (small segments however are
/// combined into one - so in reality it's a usually a bit more).
#[test]
fn single_allocation_using_builder() {
    // ... and must not leak memory, of course.
    mem_scoped(&GLOBAL, &MaNoLeak, || {
        let test_data = create_test_data();
        for number in 0u8..25u8 {
            let expected = test_data.compute_expected_result(number);
            single_allocation_for(test_data.clone(), number, expected.as_str());
        }
    });
}

fn single_allocation_for(list: SegmentList, number: u8, expected: &str) {
    // one single allocation is allowed; if it's short (fits onto the stack), we expect 0 allocations;
    let number_of_allocations = if expected.len() > STACK_MAX_LEN { 1 } else { 0 };
    mem_scoped(
        &GLOBAL,
        &MaAnd(&[
            &MaExactNumberOfAllocations(number_of_allocations),
            // no re-allocation.
            &MaExactNumberOfReAllocations(0),
        ]),
        || {
            // no allocations here.
            let mut builder = NewStr::builder();
            for src_segment in list.0 {
                if src_segment.is_include(number) {
                    builder.push(src_segment.item);
                }
            }
            // here the allocation happens.
            let built_str = builder.build();
            assert_eq!(expected, built_str.as_str());
            // here some de-allocations will happen
        },
    );
}

#[derive(Clone)]
struct SegmentList<'a>(Vec<SrcSegment<'a>>);

impl<'a> SegmentList<'a> {
    fn compute_expected_result(&self, number: u8) -> String {
        let mut vec = Vec::<u8>::new();
        for src_segment in &self.0 {
            if src_segment.is_include(number) {
                let bin: BinSegment<'a, Bin> = src_segment.item.clone().into();
                vec.extend_from_slice(bin.as_slice());
            }
        }
        String::from_utf8(vec).expect("Should only contain valid UTF-8")
    }
}

#[derive(Clone)]
struct SrcSegment<'a> {
    number: u8,
    item: StrSegment<'a, Bin>,
}

impl<'a> SrcSegment<'a> {
    fn new(number: u8, item: impl Into<StrSegment<'a, Bin>>) -> Self {
        Self {
            number,
            item: item.into(),
        }
    }

    /// Include: 50% (even/odd) or remainder is 0.
    fn is_include(&self, number: u8) -> bool {
        let even_odd_same = self.number % 2 == 0 && number % 2 == 0;
        if even_odd_same {
            true
        } else {
            number > 0 && self.number % number == 0
        }
    }
}

fn create_test_data() -> SegmentList<'static> {
    let mut vec = Vec::<SrcSegment<'static>>::new();

    vec.push(SrcSegment::new(0, StrSegment::Empty));
    vec.push(SrcSegment::new(1, StrSegment::Empty));
    vec.push(SrcSegment::new(2, StrSegment::Empty));
    vec.push(SrcSegment::new(3, StrSegment::Empty));
    vec.push(SrcSegment::new(4, StrSegment::Empty));
    vec.push(SrcSegment::new(
        5,
        StrSegment::Static("Hello, this is some content in the list"),
    ));
    vec.push(SrcSegment::new(6, StrSegment::Empty));
    vec.push(SrcSegment::new(7, StrSegment::Slice("stack")));
    vec.push(SrcSegment::new(8, StrSegment::Empty));
    vec.push(SrcSegment::new(9, StrSegment::Empty));
    vec.push(SrcSegment::new(
        10,
        StrSegment::Static("... and some more content."),
    ));
    vec.push(SrcSegment::new(11, StrSegment::Char('A')));
    vec.push(SrcSegment::new(12, StrSegment::Char('B')));
    vec.push(SrcSegment::new(
        13,
        StrSegment::Str(NewStr::from_static(
            "Some static content ... make this longer to make sure it does not fit onto the stack.",
        )),
    ));
    vec.push(SrcSegment::new(14, StrSegment::Char('C')));
    vec.push(SrcSegment::new(15, StrSegment::Char('D')));
    vec.push(SrcSegment::new(16, StrSegment::Empty));
    vec.push(SrcSegment::new(17, StrSegment::Empty));
    vec.push(SrcSegment::new(18, StrSegment::Slice("stack")));
    vec.push(SrcSegment::new(19, StrSegment::Empty));
    vec.push(SrcSegment::new(20, StrSegment::Char('E')));
    vec.push(SrcSegment::new(21, StrSegment::Char('F')));
    vec.push(SrcSegment::new(22, StrSegment::Char('G')));
    vec.push(SrcSegment::new(23, StrSegment::Char('G')));
    vec.push(SrcSegment::new(24, StrSegment::Empty));
    vec.push(SrcSegment::new(25, StrSegment::Empty));
    vec.push(SrcSegment::new(26, StrSegment::Slice("stack2")));
    vec.push(SrcSegment::new(27, StrSegment::Slice("stack3")));
    vec.push(SrcSegment::new(28, StrSegment::Empty));
    vec.push(SrcSegment::new(29, StrSegment::GivenString("This is content from a given string. Might be a bit longer. At least it's too long for the stack.".to_owned())));
    vec.push(SrcSegment::new(
        30,
        StrSegment::GivenString(
            "...and another string... longer ... even longer ... to long for the stack..."
                .to_owned(),
        ),
    ));
    vec.push(SrcSegment::new(31, StrSegment::Empty));

    SegmentList(vec)
}
