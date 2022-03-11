# Merge Algorithms
## Merging two or more ordered sets
The basic use case for these algorithms is merging two ordered arrays into a single and ordered array.
Although simple, it becomes far more complicated when you consider 
* Very large datasets spanning many processing nodes (segmentation, map/reduce, etc)
* Memory and cpu constraints on embedded systems (in-place, out-of-place) 


### Benchmarks
The following benchmarks provide an indicative performance comparison between the different merge implementations. The input size used is 5,000 (2 x 2,500) elements.
```
Out of place merge function
===========================
test bench_merge_iterator          ... bench:      61,250 ns/iter (+/- 5,708)

In place merge functions
========================
test bench_merge_lazy              ... bench:      80,606 ns/iter (+/- 2,367)
test bench_merge_mut               ... bench:      68,282 ns/iter (+/- 8,597)
test bench_merge_mut_adjacent      ... bench:      43,533 ns/iter (+/- 655)
```
### Implementation 
The chapters that follow, provide a detailed explanation on how the below implementation works
```rust,no_run,noplayground
{{#include ../../src/merge/mod.rs:merge_adjacent}}
```
