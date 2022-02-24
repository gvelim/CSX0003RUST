# Lazy merge with deferred slice mutability
Need to perform "shallow" or lazy merge, that is,
* provide ordered access to the underlying slices without mutating them (see shallow merge)
* while allow such order to be superimposed upon the slices if we later decide to (see superimpose)

## Lazy merge operation 
* Swapping of references instead of the actual data (light operation)
* Ordering logic per iteration
```rust,noplayground
use csx3::merge::Merge;

let (s1, s2) = (&mut [5,6,7], &mut[1,2,3,4]);

let mut vs = s1.merge_virtual(s2);  // attach to s2 and do shallow merge with s1
 
vs.iter()                           // ordered access of attached slices
    .enumerate()                    // [&1, &2, &3, &4, &5, &6, &7]
    .for_each(|(i,x)| 
        assert_eq(*x,i+1) 
     );

assert_eq!(s1, &[5,6,7]);           // while s1 & s2 are unaffected
assert_eq!(s2, &[1,2,3,4]);
```
## Deferred Mutability; Superimpose order
* Straight swapping of data referenced (could end up a heavy heap operation)
* No ordering logic per iteration
```rust,noplayground
 use csx3::merge::Merge;

 let s1 = &mut [5,6,7];
 let s2 = &mut [1,2,3,4];

 let mut mask = s1.merge_virtual(s2); // mask mutably borrows s1 & s2

 mask.iter()                          // iterate over merged contents
     .enumerate()                     // while s1 and s2 are unaffected
     .for_each(|(i,x)| assert_eq!(*x,i+1) );

 mask.superimpose_shallow_merge();   // mutate the order back to s1 and s2
                                     // and drop mutable references
 assert_eq!(s1, &[1,2,3]);
 assert_eq!(s2, &[4,5,6,7]);
```
