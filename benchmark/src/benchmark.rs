use core::iter;
use std::collections::VecDeque;
use std::marker::PhantomData;
use std::thread;

use enum_iterator::IntoEnumIterator;

use crate::BenchStr;

pub fn run_benchmark<TBenchStr: BenchStr>(
    number_of_threads: usize,
    test_set: &'static [&'static str],
) {
    if number_of_threads == 1 {
        run_benchmark_single_threaded::<TBenchStr>(test_set);
    } else {
        let mut handles = Vec::new();
        for _ in 0..number_of_threads {
            handles.push(thread::spawn(move || {
                run_benchmark_single_threaded::<TBenchStr>(test_set);
            }));
        }
        for handle in handles {
            handle.join().unwrap();
        }
    }
}

pub struct Benchmark<'a, TBenchStr> {
    test_set: &'a [&'static str],
    // we keep some references around to make sure allocator has to do more than just work like a stack.
    collector: Collector<TBenchStr>,
    _phantom: PhantomData<TBenchStr>,
}

pub fn run_benchmark_single_threaded<TBenchStr: BenchStr>(test_set: &[&'static str]) {
    let mut benchmark = Benchmark::<TBenchStr>::new(test_set);
    benchmark.perform();
}

impl<'a, TBenchStr: BenchStr> Benchmark<'a, TBenchStr> {
    fn new(test_set: &'a [&'static str]) -> Self {
        Self {
            test_set,
            collector: Collector::new(1024 * 1024 * 8, 300),
            _phantom: Default::default(),
        }
    }

    fn perform(&mut self) {
        // small items are the usual case, so we over-represent them
        let smallest = self.collect_smallest(self.test_set.len() / 4);

        // perform a few times to make sure the collector gets filled.
        for _ in 0..50 {
            self.perform_with_test_set(&smallest);
            self.perform_with_test_set(self.test_set);
            self.perform_with_test_set(&smallest);
            self.perform_with_test_set(&smallest);
        }
    }

    fn perform_with_test_set(&mut self, items: &[&'static str]) {
        if items.is_empty() {
            return;
        }
        for item in items {
            self.join_single(item);
            self.do_clone(item);
            self.slice(item);
        }
        self.join(items);
    }

    fn join(&mut self, items: &[&'static str]) {
        for from_fn in FromFunction::into_enum_iter() {
            let mut strings: Vec<TBenchStr> = Vec::with_capacity(items.len());
            for item in items {
                strings.push(Self::bench_str_from(from_fn, item));
            }
            let string = TBenchStr::from_multiple(strings.into_iter());
            let cloned = string.clone();
            self.collector.keep(string);
            self.collector.keep(cloned);
        }
    }

    fn join_single(&mut self, item: &'static str) {
        // make sure join is also efficient if there's just one single item
        for from_fn in FromFunction::into_enum_iter() {
            let string = TBenchStr::from_multiple(iter::once(Self::bench_str_from(from_fn, item)));
            assert_eq!(string.as_slice(), item);
            self.collector.keep(string);
        }
    }

    fn do_clone(&mut self, item: &'static str) {
        for from_fn in FromFunction::into_enum_iter() {
            let string = Self::bench_str_from(from_fn, item);
            assert_eq!(string.as_slice(), string.clone().as_slice());
            self.collector.keep(string);
        }
    }

    fn slice(&mut self, item: &'static str) {
        for from_fn in FromFunction::into_enum_iter() {
            let string = Self::bench_str_from(from_fn, item);
            let len = string.as_slice().len();

            // first slice full.
            let slice = string.slice(0, len);
            if let Some(slice) = slice {
                self.collector.keep(slice);
            }
            // from start / small
            let slice = string.slice(0, 3);
            if let Some(slice) = slice {
                self.collector.keep(slice);
            }
            // not from start (still small)
            let slice = string.slice(25, 29);
            if let Some(slice) = slice {
                self.collector.keep(slice);
            }
            // not from start bigger
            let slice = string.slice(14, 350);
            if let Some(slice) = slice {
                self.collector.keep(slice);
            }

            self.collector.keep(string);
        }
    }

    fn bench_str_from(from_fn: FromFunction, item: &'static str) -> TBenchStr {
        match from_fn {
            FromFunction::Static => TBenchStr::from_static(item),
            FromFunction::BinIter => {
                TBenchStr::from_bin_iter(item.as_bytes().iter().cloned()).unwrap()
            }
            FromFunction::Str => TBenchStr::from_str(item),
        }
    }

    fn collect_smallest(&self, max: usize) -> Vec<&'static str> {
        let mut vec: Vec<&'static str> = self.test_set.to_owned();
        vec.sort_by(|a, b| a.len().cmp(&(*b).len()));
        let too_many = vec.len() as isize - max as isize;
        if too_many > 0 {
            for _ in 0..too_many {
                let len = vec.len();
                vec.remove(len - 1);
            }
        }
        vec
    }
}

#[derive(Copy, Clone, IntoEnumIterator)]
enum FromFunction {
    Static,
    BinIter,
    Str,
}

struct Collector<TBenchStr> {
    queue_0: CollectorQueue<TBenchStr>,
    queue_1: CollectorQueue<TBenchStr>,
    queue_2: CollectorQueue<TBenchStr>,
    counter: usize,
}

impl<TBenchStr: BenchStr> Collector<TBenchStr> {
    fn new(max_number_of_bytes: usize, max_number_of_items: usize) -> Self {
        let number_of_bytes_per_queue = max_number_of_bytes / 3;
        let number_of_items_per_queue = max_number_of_items / 3;
        Self {
            queue_0: CollectorQueue::new(number_of_bytes_per_queue, number_of_items_per_queue),
            queue_1: CollectorQueue::new(number_of_bytes_per_queue, number_of_items_per_queue),
            queue_2: CollectorQueue::new(number_of_bytes_per_queue, number_of_items_per_queue),
            counter: 0,
        }
    }

    fn keep(&mut self, string: TBenchStr) {
        self.counter += 1;
        let remainder = self.counter % 4;
        match remainder {
            0 => self.queue_0.push(string),
            1 => self.queue_1.push(string),
            2 => self.queue_2.push(string),
            3 => { /* drop right away */ }
            _ => unreachable!(),
        }
    }
}

// we keep some references around to make sure allocator has to do more than just work like a stack.
struct CollectorQueue<TBenchStr> {
    queue: VecDeque<TBenchStr>,
    number_of_bytes: usize,
    max_number_of_bytes: usize,
    max_number_of_items: usize,
}

impl<TBenchStr: BenchStr> CollectorQueue<TBenchStr> {
    pub fn new(max_number_of_bytes: usize, max_number_of_items: usize) -> Self {
        Self {
            queue: Default::default(),
            number_of_bytes: 0,
            max_number_of_bytes,
            max_number_of_items,
        }
    }

    fn push(&mut self, str: TBenchStr) {
        self.number_of_bytes += str.as_slice().len();
        self.queue.push_back(str);

        while self.number_of_bytes > self.max_number_of_bytes
            || self.queue.len() > self.max_number_of_items
        {
            self.remove_front()
        }
    }

    fn remove_front(&mut self) {
        let removed = self.queue.pop_front();
        if let Some(removed) = removed {
            self.number_of_bytes -= removed.as_slice().len();
        }
    }
}
