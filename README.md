** DO NOT USE YET - WIP - NOT YET RELEASED **

## Introduction

A utility library for working with binaries. It provides multiple implementations that all share the same interface (`AnyBin`, `struct Bin`/`struct SyncBin`). `Bin` and `SyncBin` have no lifetime arguments, are sized (structs), easy to use, most operations are allocation-free, and they can be converted to each other. `SyncBin` is a version of `Bin` that implements `Send + Sync`. The available implementations are:

 * `StaticBin`: A binary pointing to static data.
 * `VecBin`: A binary backed by a `Vec<u8>`.
 * `RcBin`: Reference counted binary (without synchronization-overhead). (only implements `Bin`, not `SyncBin`).
 * `ArcBin`: Reference counted binary (with synchronization-overhead).
 * `StackBin`: Stores small binaries on the stack.
 * `EmpyBin`: For empty binaries (stack).  

It's similar to [https://crates.io/crates/bytes](https://crates.io/crates/bytes); these are the main differences:

 * It's extensible (you can provide your own binary type).
 * Stores small binaries on the stack.
 * Provides a reference-counted binary without synchronization-overhead (`RcBin`).
 * ... see *Details* below for more differences.

## Details / Highlights / Features

**Reduces allocations & memory usage**

Many operations do not need memory-allocation / are zero-copy operations. There's a reference counted binary that can be cloned without allocating memory. Small binaries (up to 3 words minus one byte; 23 bytes on 64-bit platforms) can be stored in-line (on the stack).

See [tests/no_alloc_guarantees.rs](tests/no_alloc_guarantees.rs) for operations that are guaranteed to be alloc-free.

**Reference counted binary**

There's a reference-counted binary that can be cloned without allocating memory. It can be constructed from a `Vec<u8>` without allocating memory (as long as the vec has some capacity left). It can be converted back to a `Vec<u8>` without allocating memory (as long as there are no more references to the binary).

**Reference-counted binary: No indirection / from Vec**

If you use `Rc<[u8]>`/`Arc<[u8]>` there's no possibility to convert `Vec<u8>` to `Rc<[u8]>`/`Arc<[u8]>` without memory-copy/allocation. If you use `Rc<Vec<u8>>`/`Arc<Vec<u8>>` another indirection is introduced (`Rc -> RcBox(Vec) -> VecData`).

This crate on the other hand supports reference counted binaries that can be constructed from a `Vec<u8>` without allocation*1 and still does not introduce another indirection. It does this by storing the metadata (like the reference-counter) inside the vector. 

(*1): The vector must have some capacity left (it's 3 word-aligned words; this is between 24 and 31 bytes on 64-bit platforms).

**No synchronization when not needed**

There are two versions of the reference-counted binary: A synchronized one and one that's not synchronized. As long as you use the non-synchronized binary, there's no need for synchronization.  

**Static binary**

There's a binary that can be used for static data. No allocation.

**Slices**

All binaries can be sliced. Some binaries (the static and the reference-counted ones) support allocation-free slicing. 

**Extensible**

If you don't like the implementations provided by this crate, you can implement your own binary type.

**Strings**

There's also s string implementation available that's backed by the binaries provided by this crate (see `AnyStr`, `Str` and `SyncStr`).

**Map-friendly**

The binaries (and strings) provided by this crate can be used in maps; they implement `Hash`/`Equals` and implement `Borrow<[u8]>` (for lookup operations using `&[u8]`). 

**Stack-size**

The binary provided by this crate uses 4 words on the stack (32 bytes on 64-bit platforms; 16 bytes on 32-bit platforms). It's one word more than `Vec<u8>`; 2 words more than `Rc<[u8]>`; 3 words more than `Rc<Vec<u8>>` (on the other hand it can store small binaries on the stack).
 
## Basic usage

```toml
[dependencies]
abin = "*"
```

See [tests/basic_usage.rs](tests/basic_usage.rs):

```rust
use abin::{AnyBin, AnyRc, ArcBin, Bin, EmptyBin, RcBin, StaticBin, UnSync, VecBin};

#[test]
pub fn usage() {
    // empty binary, stack-only.
    let bin1 = EmptyBin::new();
    // small binary; stack-only.
    let bin2 = RcBin::copy_from_slice(&[5, 10]);
    // reference-counted binary (not synchronized); from a slice; can also be constructed from a vec.
    let bin3 = RcBin::copy_from_slice("This is a binary; too large for the stack.".as_bytes());
    // reference-counted binary (synchronized); this time from a vector (does not allocate if the
    // vector has enough capacity for the meta-data).
    let bin4 = ArcBin::from_vec(
        "This is a binary; too large for the stack."
            .to_owned()
            .into_bytes(),
    );
    // binary backed by a Vec<u8>.
    let bin5 = VecBin::from_vec(
        "This is a vector binary, backed by a vector"
            .to_owned()
            .into_bytes(),
        true,
    );
    // no allocation for static data.
    let bin6 = StaticBin::from("Static data".as_bytes());

    use_bin(bin1.un_sync());
    use_bin(bin2);
    use_bin(bin3);
    use_bin(bin4.un_sync());
    use_bin(bin5.un_sync());
    use_bin(bin6.un_sync());
}

/// Just two interfaces for all binaries (`Bin`/`SyncBin`) - `SyncBin` can be converted to `Bin`.
pub fn use_bin(bin: Bin) {
    // length of the binary (cheap operation).
    let len = bin.len();
    // to &[u8] (cheap operation)
    let _u8_slice = bin.as_slice();
    // can be cloned (for reference-counted binaries, StaticBin and stack-binary, this is cheap).
    let cloned_bin = bin.clone();
    assert_eq!(bin, cloned_bin);
    assert_eq!(len, cloned_bin.len());
    // can be sliced (cheap operation for reference-counted binaries, StaticBin and stack-binary).
    let slice = bin.slice(0..10);
    if let Some(slice) = slice {
        assert_eq!(10, slice.len());
    }
    // ...and converted into vector (cheap operation for VecBin and for reference-counted
    // binaries with no more references).
    let vec = bin.into_vec();
    assert_eq!(cloned_bin.as_slice(), vec.as_slice());
}
```

## Important traits / structs

 * `Bin` / `SyncBin`: The interfaces (structs) for all binary types.
 * `AnyBin`: The trait `Bin` and `SyncBin` implement.
 * `RcBin`, `ArcBin`, `StaticBin`, `VecBin`, `EmptyBin`: Implementations; they provide methods to construct `Bin` / `SyncBin`.
 * `AnyRc`: The trait both reference-counted types implement.
 * `IntoSync`, `IntoUnSyncView`, `UnSyncRef`, `IntoUnSync`: Convert `Bin` to `SyncBin` and vice-versa.
 * `ChainSlicesIter`: Chain multiple binaries (slices) into one binary with just one single allocation.
 * `AnyStr` (`Str` / `SyncStr`): `Bin`/`SyncBin` backed utf-8 strings.
 
## Design decisions / faq

### No `Deref<Target=[u8]>` for `Bin`/`SyncBin`

I decided against implementing this for `Bin`/`SyncBin`. Reason: It's too easy to pick the wrong method if this is implemented; for instance there's `&[u8]::to_vec()` (which needs to allocate & copy) and there's `Bin::into_vec()` you most likely want to use. ... or `&[u8]::len()` and `Bin::len()` ... there's some change you pick the wrong operation.

### No `From<T>` for `Bin`/`SyncBin`

I want `Bin`/`SyncBin` (interface) to be decoupled from the implementation (`RcBin`/`VecBin`...). Implementing `From<Vec<u8>>` for `Bin`/`SyncBin` would couple the interface to a certain implementation... the next question would arise: Which implementation to take? A `Bin` can be constructed from a `Vec<u8>` using `RcBin`, `ArcBin` and `VecBin`, which one is the correct implementation?  
