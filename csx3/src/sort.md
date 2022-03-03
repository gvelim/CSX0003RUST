# Sort Algorithms
Collection of algorithms with implementation attention to rust's expressiveness in terms of Iterators, generic types, matching expressions, ownership and borrowing rules, safe `unsafe` use, etc

Below indicative benchmarks of the implemented sort functions compared to rust's standard implementation
```gitignore
test bench_countsort              ... bench:      92,429 ns/iter (+/- 11,676)
test bench_mergesort              ... bench:   1,047,933 ns/iter (+/- 129,582)
test bench_mergesort_mut          ... bench:     865,993 ns/iter (+/- 87,394)
test bench_mergesort_mut_adjacent ... bench:     501,945 ns/iter (+/- 23,939)
test bench_quicksort              ... bench:     280,301 ns/iter (+/- 10,760)
test bench_std_vector_sort        ... bench:     185,760 ns/iter (+/- 20,645)
```
