use core::sync::atomic;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::atomic::Ordering::{Relaxed, Release};

pub trait RcCounter {
    /// creates a new counter. The next call to `decrement` will return `Zero`.
    fn new() -> Self;
    fn decrement(&mut self) -> RcDecResult;
    fn increment(&mut self);
}

#[derive(Copy, Clone)]
pub enum RcDecResult {
    /// The counter has reached 'zero'. No more references. The counter must no longer be
    /// used (it's in an invalid state).
    Zero,
    /// The counter has not yet reached 'zero'.
    More,
}

/// a non-synchronized reference counter.
#[repr(transparent)]
pub struct NsRcCounter(usize);

const FINISHED_MARKER: usize = usize::MAX;

impl RcCounter for NsRcCounter {
    #[inline]
    fn new() -> Self {
        Self(0)
    }

    #[inline]
    fn decrement(&mut self) -> RcDecResult {
        let current = self.0;
        if current == 0 {
            self.0 = FINISHED_MARKER;
            RcDecResult::Zero
        } else if current == FINISHED_MARKER {
            panic!("Some implementation error. Reference counter has invalid state.")
        } else {
            self.0 = current - 1;
            RcDecResult::More
        }
    }

    #[inline]
    fn increment(&mut self) {
        let current = self.0;
        if current == FINISHED_MARKER {
            panic!("Too many reference counts or implementation error (reference counter has \
            invalid state).")
        }
        self.0 = current + 1;
    }
}

/// a synchronized reference counter.
#[repr(transparent)]
pub struct SyncRcCounter(AtomicUsize);

impl RcCounter for SyncRcCounter {
    #[inline]
    fn new() -> Self {
        Self(AtomicUsize::new(0))
    }

    #[inline]
    fn decrement(&mut self) -> RcDecResult {
        // 'Release' seems to be ok according to the sources from "Arc" (rust std lib).
        // Note: After this call, the ref count will be MAX (overflow) - but this should be
        // ok (since we do not need that value anymore).
        let previous_value = self.0.fetch_sub(1, Release);
        if previous_value == 0 {
            // According to "Arc" (rust std lib) we also need this (don't know exactly why).
            atomic::fence(Ordering::Acquire);
            RcDecResult::Zero
        } else {
            RcDecResult::More
        }
    }

    #[inline]
    fn increment(&mut self) {
        // 'Relaxed' seems to be ok according to the sources from "Arc" (rust std lib).
        let previous_value = self.0.fetch_add(1, Relaxed);
        if previous_value == core::usize::MAX {
            // we need to revert that
            self.0.fetch_sub(1, Release);
            panic!("Too many reference counts or implementation error (too many rc decrements).")
        }
    }
}