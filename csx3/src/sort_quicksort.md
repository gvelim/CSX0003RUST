# Quick Sort
A generic implementation that mutably sorts the input array by recursively pivoting around a specific point that is randomly selected

```rust,no_run,noplayground
{{#include ../../src/sort/mod.rs:sort_quick}}
```
## Partitioning around a pivot
Splits an array into two mutable slices/partitions around a given pivot location such that

`[values in left partition] < [pivot] < [values in right partition]`


```rust,no_run,noplayground
{{#include ../../src/sort/mod.rs:sort_quick_partition}}
```