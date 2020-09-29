** DO NOT USE YET - WIP - NOT YET RELEASED **

## Design decisions / faq

### No `Deref<Target=[u8]>` for `Bin`/`SyncBin`

I decided against implementing this for `Bin`/`SyncBin`. Reason: It's too easy to pick the wrong method if this is implemented; for instance there's `&[u8]::to_vec()` (which needs to allocate & copy) and there's `Bin::into_vec()` you most likely want to use. 

