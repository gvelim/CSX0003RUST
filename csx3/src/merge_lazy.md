# Lazy merge with deferred slice mutability
Need to perform "shallow" or lazy merge, that is,
* provide ordered access to the underlying slices without mutating them (see shallow merge)
* while allow such order to be superimposed upon the slices if we later decide to (see superimpose)


## Lazy merge operation 
* Swapping of references instead of the actual data (light operation)
* Ordering logic per iteration
```
use csx3::utils::VirtualSlice;

let (s1, s2) = (&mut [5,6,7], &mut[1,2,3,4]);
let mut vs = VirtualSlice::new();

vs.attach(s1);                  // attach to s1
vs.merge_shallow(s2);           // attach to s2 and do shallow merge with s1
 
vs.iter()                       // ordered access of attached slices
    .enumerate()                // [&1, &2, &3, &4, &5, &6, &7]
    .for_each(|(i,x)| 
        assert_eq(*x,i+1) 
     );

assert_eq!(s1, &[5,6,7]);       // while s1 & s2 are unaffected
assert_eq!(s2, &[1,2,3,4]);
```
## Deferred Mutability; Superimpose order
* Straight swapping of data referenced (could end up a heavy heap operation)
* No ordering logic per iteration
```
use csx3::utils::VirtualSlice;

let (s1, s2) = (&mut [5,6,7], &mut[1,2,3,4]);
let mut vs = VirtualSlice::new();

vs.attach(s1);                  // attach to s1
vs.merge_shallow(s2);           // attach to s2 and do shallow merge with s1
                                
                                // vs == &[&1,&2,&3,&4,&5,&6,&7]
                                // s1 == &[5,6,7]
                                // s2 == &[1,2,3,4]
vs.impose_shallow_merge();      // superimpose order mask

assert_eq!(s1, &[1,2,3]);
assert_eq!(s2, &[4,5,6,7]);
```
