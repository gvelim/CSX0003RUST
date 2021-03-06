# Sort Algorithms
* MergeSort
* QuickSort
* CountSort

For more details visit here: 
https://gvelim.github.io/CSX0003RUST/sort.html

### Benchmarks
```gitignore
test bench_countsort              ... bench:      92,429 ns/iter (+/- 11,676)
test bench_mergesort              ... bench:   1,047,933 ns/iter (+/- 129,582)
test bench_mergesort_mut          ... bench:     865,993 ns/iter (+/- 87,394)
test bench_mergesort_mut_adjacent ... bench:     501,945 ns/iter (+/- 23,939)
test bench_quicksort              ... bench:     280,301 ns/iter (+/- 10,760)
test bench_std_vector_sort        ... bench:     185,760 ns/iter (+/- 20,645)
```

### Examples
In-place count sort
```rust
use csx3::sort::count::CountSort;

let v = &mut [3i8,5,8,1,2,4,6,0];

v.count_sort();

assert_eq!(v, &[0,1,2,3,4,5,6,8]);
```
In-place merge sort
```rust
use csx3::{ merge::Merge, sort::merge::MergeSort };

let input = &mut [8, 4, 2, 1];

assert_eq!( input.mergesort_mut(Merge::merge_mut_adjacent), 6 );
assert_eq!( input, &[1,2,4,8] );
```
Out-of-place merge sort
```rust
use csx3::sort::merge::MergeSort;

let input = &[8, 4, 2, 1];

assert_eq!( input.mergesort(), (6, vec![1,2,4,8]) );
```
In-place quick sort
```rust
use csx3::sort::quick::QuickSort;

let v = &mut [3,5,8,1,2,4,6,0];

v.quick_sort();

assert_eq!(v, &[0,1,2,3,4,5,6,8]);
```