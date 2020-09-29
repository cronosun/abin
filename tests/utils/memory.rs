use std::alloc::GlobalAlloc;

use stats_alloc::{Region, Stats, StatsAlloc};

pub struct Memory<'a, T: GlobalAlloc + 'a> {
    alloc: &'a StatsAlloc<T>
}

pub fn mem_scoped<'b, TGa, A, TFn>(alloc: &'b StatsAlloc<TGa>, mem_assert: &A, fun: TFn) where TGa: GlobalAlloc + 'b, A: MemAssert, TFn: FnOnce() {
    let this = Memory::new(alloc);
    this.scoped(mem_assert, fun);
}

impl<'a, T: GlobalAlloc + 'a> Memory<'a, T> {
    pub fn new(alloc: &'a StatsAlloc<T>) -> Self {
        Self {
            alloc,
        }
    }

    pub fn scoped<A, TFn>(&self, mem_assert: &A, fun: TFn)
        where A: MemAssert, TFn: FnOnce() {
        let region = Region::new(self.alloc);
        fun();
        let change = region.change();
        if let Err(err) = mem_assert.assert(change) {
            panic!("Memory assertion error: '{}' (change: {:?})", err, change)
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
            Err(format!("Memory leak: allocations != de-allocations ({} != {})", alloc, de_alloc))
        } else {
            Ok(())
        }
    }
}

pub struct MaNoAllocation;

impl MemAssert for MaNoAllocation {
    fn assert(&self, change: Stats) -> Result<(), String> {
        let num_allocations = change.allocations;
        let num_de_allocations = change.deallocations;
        if num_allocations == 0 && num_de_allocations == 0 {
            Err(format!("Expected to have no allocation/de-allocation (#op alloc: {}, \
            #op de-alloc {})", num_allocations, num_de_allocations))
        } else {
            Ok(())
        }
    }
}
