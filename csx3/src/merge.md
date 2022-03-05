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
test bench_merge_iterator     ... bench:      52,403 ns/iter (+/- 1,824)

In place merge functions
========================
test bench_merge_lazy         ... bench:      51,617 ns/iter (+/- 1,391)
test bench_merge_mut          ... bench:      57,116 ns/iter (+/- 2,133)
test bench_merge_mut_adjacent ... bench:      45,202 ns/iter (+/- 1,340)
```
