# Merge Sort
Generic implementation flavours covering
* in-place (mutable) or out-of-place (immutable) sorting of a given array
* calculation of number of inversion occurred

### In-place sorting (mutable)
The implementation re-orders/mutates the input array and returns the number if inversions occurred

In relation to time and space complexity, the implementation offers 
* [merging operations of O(n+m) swaps with no use of temp storage](./merge_in_place.md)
* Takes up to `O(n+m) * usize` memory space per merge cycle which can be further reduced to `O(n) * usize` 

```rust,no_run,noplayground
{{#include ../../src/sort/mod.rs:sort_merge_mut}}
```

### Out-of-place sorting (immutable)
The implementation returns a sorted copy of the input array along with the total number of inversions occurred.  

The implementation 
* Utilises `Iterator` traits of input slices to retrieve the next in order element through the `next()` function.
* Returns the inversion `count` per position and as part of calling `next()` function
* Takes `O(n+m) * typeof(array)` memory space per merge cycle

```rust,no_run,noplayground
{{#include ../../src/sort/mod.rs:sort_merge}}
```