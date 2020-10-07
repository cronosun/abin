use std::alloc::System;

use rayon::prelude::*;
use stats_alloc::{StatsAlloc, INSTRUMENTED_SYSTEM};

use abin::{NewSStr, SStr, StrFactory};
use utils::*;

#[global_allocator]
static GLOBAL: &StatsAlloc<System> = &INSTRUMENTED_SYSTEM;

pub mod utils;

#[derive(Clone)]
struct SrcList(Vec<SrcItem>);

#[test]
fn send_sync_string_concat_using_rayon() {
    let end: u16 = 1000;
    let test_data = create_test_data();

    // compute again
    let mut string_expected = String::new();
    for number in 0u16..end {
        let expected = test_data.compute_expected_result(number);
        string_expected.push_str(expected.as_str());
    }

    // ... and must not leak memory, of course.
    mem_scoped(&GLOBAL, &MaNoLeak, || {
        // compute without rayon
        let string_without_rayon: String = (0u16..end)
            .map(|number| {
                let string: String = test_data
                    .0
                    .as_slice()
                    .iter()
                    .filter(|item| item.is_include(number))
                    .map(|src_item| {
                        // we clone it - just for fun :-) na... to test reference-counting
                        src_item.string.clone()
                    })
                    .map(|any_str| any_str.into_string())
                    .collect();
                NewSStr::from_given_string(string)
            })
            .map(|string| {
                // we clone it - just for fun :-) na... to test reference-counting
                string.clone()
            })
            .map(|string| string.into_string())
            .collect();

        assert_eq!(string_expected, string_without_rayon);
    });

    // note: I actually wanted the following code to be inside `mem_scoped` too ... but rayon seems to
    // leak memory itself... even with a custom thread pool... it seems to keep some memory
    // around (I guess it's some static data related to thread-pools).

    // and now this time compute using rayon
    let iter = (0u16..end).into_iter();
    let par_iter = iter.into_par_iter();
    let string_rayon: String = par_iter
        .map(|number| {
            let string: String = test_data
                .0
                .as_slice()
                .iter()
                .filter(|item| item.is_include(number))
                .map(|src_item| {
                    // we clone it - just for fun :-) na... to test reference-counting
                    src_item.string.clone()
                })
                .map(|any_str| any_str.into_string())
                .collect();
            NewSStr::from_given_string(string)
        })
        .map(|string| {
            // we clone it - just for fun :-) na... to test reference-counting
            string.clone()
        })
        .map(|string| string.into_string())
        .collect();
    assert_eq!(string_expected, string_rayon);
}

impl SrcList {
    fn compute_expected_result(&self, number: u16) -> String {
        let mut string = String::new();
        for src_segment in &self.0 {
            if src_segment.is_include(number) {
                string.push_str(src_segment.string.as_str());
            }
        }
        string
    }
}

#[derive(Clone)]
struct SrcItem {
    number: u8,
    string: SStr,
}

impl SrcItem {
    fn new(number: u8, item: impl Into<SStr>) -> Self {
        Self {
            number,
            string: item.into(),
        }
    }

    /// Include: 50% (even/odd) or remainder is 0.
    fn is_include(&self, number: u16) -> bool {
        let even_odd_same = self.number % 2 == 0 && number % 2 == 0;
        if even_odd_same {
            true
        } else {
            number > 0 && (self.number as u16) % number == 0
        }
    }
}

fn create_test_data() -> SrcList {
    let mut vec = Vec::<SrcItem>::new();

    vec.push(SrcItem::new(0, NewSStr::empty()));
    vec.push(SrcItem::new(
        1,
        NewSStr::from_static(
            "This is some static text (too long for stack... too long, too long...)",
        ),
    ));
    vec.push(SrcItem::new(2, NewSStr::empty()));
    vec.push(SrcItem::new(3, NewSStr::empty()));
    vec.push(SrcItem::new(4, NewSStr::from_static("small1")));
    vec.push(SrcItem::new(5, NewSStr::from_static("small2")));
    vec.push(SrcItem::new(6, NewSStr::from_static("small3")));
    vec.push(SrcItem::new(7, NewSStr::copy_from_str("short")));
    vec.push(SrcItem::new(8, NewSStr::copy_from_str("short")));
    vec.push(SrcItem::new(
        9,
        NewSStr::copy_from_str("Some non-static text. Longer, longer, longer. Even longer."),
    ));
    vec.push(SrcItem::new(
        10,
        NewSStr::copy_from_str("Some non-static text. Longer, longer, longer. Even longer."),
    ));

    for num in 10u8..35u8 {
        vec.push(SrcItem::new(num, NewSStr::empty()));
        vec.push(SrcItem::new(num + 1, NewSStr::from_static("small3")));
        vec.push(SrcItem::new(
            num + 2,
            NewSStr::from_static(
                "This is some static text (too long for stack... too long, too long...)",
            ),
        ));
        vec.push(SrcItem::new(num + 3, NewSStr::empty()));
        vec.push(SrcItem::new(num + 4, NewSStr::empty()));
        vec.push(SrcItem::new(
            num + 5,
            NewSStr::copy_from_str("Some non-static text. Longer, longer, longer. Even longer."),
        ));
        vec.push(SrcItem::new(num + 5, NewSStr::copy_from_str("short")));
        vec.push(SrcItem::new(num + 5, NewSStr::copy_from_str("short")));
    }

    SrcList(vec)
}
