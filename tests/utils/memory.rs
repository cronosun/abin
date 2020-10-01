use std::alloc::GlobalAlloc;

use stats_alloc::{Region, Stats, StatsAlloc};

pub struct Memory<'a, T: GlobalAlloc + 'a> {
    alloc: &'a StatsAlloc<T>,
}

pub fn mem_scoped<'b, TGa, A, TFn, TRet>(
    alloc: &'b StatsAlloc<TGa>,
    mem_assert: &A,
    fun: TFn,
) -> TRet
where
    TGa: GlobalAlloc + 'b,
    A: MemAssert,
    TFn: FnOnce() -> TRet,
{
    let this = Memory::new(alloc);
    this.scoped(mem_assert, fun)
}

impl<'a, T: GlobalAlloc + 'a> Memory<'a, T> {
    pub fn new(alloc: &'a StatsAlloc<T>) -> Self {
        Self { alloc }
    }

    pub fn scoped<A, TFn, TRet>(&self, mem_assert: &A, fun: TFn) -> TRet
    where
        A: MemAssert,
        TFn: FnOnce() -> TRet,
    {
        let region = Region::new(self.alloc);
        let ret = fun();
        let change = region.change();
        if let Err(err) = mem_assert.assert(change) {
            panic!("Memory assertion error: '{}' (change: {:?})", err, change)
        } else {
            ret
        }
    }
}

pub trait MemAssert {
    fn assert(&self, change: Stats) -> Result<(), String>;
}

pub struct MaNoLeak;

impl MemAssert for MaNoLeak {
    fn assert(&self, change: Stats) -> Result<(), String> {
        let alloc = change.bytes_allocated;
        let de_alloc = change.bytes_deallocated;
        if alloc != de_alloc {
            Err(format!(
                "Memory leak: allocations != de-allocations ({} != {})",
                alloc, de_alloc
            ))
        } else {
            Ok(())
        }
    }
}

/// No allocation and no de-allocation allowed.
pub struct MaNoAllocNoDealloc;

impl MemAssert for MaNoAllocNoDealloc {
    fn assert(&self, change: Stats) -> Result<(), String> {
        let num_allocations = change.allocations;
        let num_de_allocations = change.deallocations;
        if num_allocations != 0 || num_de_allocations != 0 {
            Err(format!(
                "Expected to have no allocation/de-allocation (#op alloc: {}, \
            #op de-alloc {})",
                num_allocations, num_de_allocations
            ))
        } else {
            Ok(())
        }
    }
}

/// Only de-allocations allowed.
pub struct MaOnlyDeAllocation;

impl MemAssert for MaOnlyDeAllocation {
    fn assert(&self, change: Stats) -> Result<(), String> {
        let num_allocations = change.allocations;
        if num_allocations != 0 {
            Err(format!(
                "Expected to have no allocation (#op alloc: {})",
                num_allocations
            ))
        } else {
            Ok(())
        }
    }
}

/// Must have at least one allocation.
pub struct MaDoesAllocate;

impl MemAssert for MaDoesAllocate {
    fn assert(&self, change: Stats) -> Result<(), String> {
        let num_allocations = change.allocations;
        if num_allocations == 0 {
            Err(format!(
                "Expected to have at least one allocation (#op alloc: {})",
                num_allocations
            ))
        } else {
            Ok(())
        }
    }
}

/// Must have exactly the number of given allocations.
pub struct MaExactNumberOfAllocations(pub usize);

impl MemAssert for MaExactNumberOfAllocations {
    fn assert(&self, change: Stats) -> Result<(), String> {
        let num_allocations = change.allocations;
        if num_allocations != self.0 {
            Err(format!(
                "Expected to have exactly {} allocations (got {} allocations instead)",
                self.0, num_allocations
            ))
        } else {
            Ok(())
        }
    }
}

/// Must have exactly the number of given de-allocations.
pub struct MaExactNumberOfDeAllocations(pub usize);

impl MemAssert for MaExactNumberOfDeAllocations {
    fn assert(&self, change: Stats) -> Result<(), String> {
        let num_de_allocations = change.deallocations;
        if num_de_allocations != self.0 {
            Err(format!(
                "Expected to have exactly {} de-allocations (got {} de-allocations instead)",
                self.0, num_de_allocations
            ))
        } else {
            Ok(())
        }
    }
}

/// Must have exactly the number of given re-allocations.
pub struct MaExactNumberOfReAllocations(pub usize);

impl MemAssert for MaExactNumberOfReAllocations {
    fn assert(&self, change: Stats) -> Result<(), String> {
        let num_re_allocations = change.reallocations;
        if num_re_allocations != self.0 {
            Err(format!(
                "Expected to have exactly {} re-allocations (got {} re-allocations instead)",
                self.0, num_re_allocations
            ))
        } else {
            Ok(())
        }
    }
}

/// All asserts must hold.
pub struct MaAnd<'a>(pub &'a [&'a dyn MemAssert]);

impl<'a> MemAssert for MaAnd<'a> {
    fn assert(&self, change: Stats) -> Result<(), String> {
        for assert in self.0 {
            assert.assert(change)?;
        }
        Ok(())
    }
}
