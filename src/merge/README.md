
# Merge Algorithm
Merge trait offering in-place & out-of-place merge capability between two or more slices

1. In-place Merge Algorithm using efficient swapping
2. Sequential access across multiple slices
3. Lazy merge and deferred slice mutability
4. Pattern matching: De-normalising control flow

For more details visit https://gvelim.github.io/CSX0003RUST/merge.html
### Benchmarks
```
test bench_merge_lazy         ... bench:      80,672 ns/iter (+/- 313)
test bench_merge_mut          ... bench:      98,679 ns/iter (+/- 3,988)
test bench_merge_mut_adjacent ... bench:      65,436 ns/iter (+/- 951)
```
### Examples
Out of place merge using iterators
```rust
use csx3::merge::MergeIterator;

let s1 = &[2, 4, 6];
let s2 = &[1, 3, 5];

let mut iter = MergeIterator::new(s1.iter(), s2.iter());

assert_eq!(iter.next(), Some( (3,&1) ));
assert_eq!(iter.next(), Some( (0,&2) ));
assert_eq!(iter.next(), Some( (2,&3) ));
assert_eq!(iter.next(), Some( (0,&4) ));
assert_eq!(iter.next(), Some( (1,&5) ));
assert_eq!(iter.next(), Some( (0,&6) ));
assert_eq!(iter.next(), None);
```
In-place lazy merge & state superimposing between two slices.
```rust
use csx3::merge::Merge;

let s1 = &mut [5,6,7];
let s2 = &mut [1,2,3,4];

let mut mask = s1.merge_lazy(s2);    // mask mutably borrows s1 & s2

mask.iter()                          // iterate over merged contents
    .enumerate()                     // while s1 and s2 are unaffected
    .for_each(|(i,x)| assert_eq!(*x,i+1) );

mask.superimpose_state();           // mutate the order back to s1 and s2
                                    // and drop mutable references
assert_eq!(s1, &[1,2,3]);
assert_eq!(s2, &[4,5,6,7]);
```