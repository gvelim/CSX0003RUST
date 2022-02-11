## Virtual Slice - continuous access over array fragments
A `VirtualSlice` is composed out of one or more array fragments, adjacent to memory or not, and enables transparently operating over the **attached** array fragments. The virtualslice operates in two mode. Adjacent and non-adjacent

* Need to access multiple slices as a single continuous one
* Need to handle efficiently cases where slices are adjacent in memory
* Need to perform a merge of two or more ordered slices
* Need to perform "shallow" merge, that is, access the slices in the right order, while I shall be able to mutate them later on if I want to

## Memory layout
#### Non-Adjacent arrays Mode
```
Left Array       Right Array
+---+---+---+    +---+---+---+     
| 2 | 4 | 6 | <> | 1 | 3 | 5 |   Memory non-adjacent array segments
+---+---+---+    +---+---+---+     
  c       ^        j
          |__
       ...  | ...
+----+----+----+----+----+----+
| &2 | &4 | &6 | &1 | &3 | &5 |  Array of mutable references : Virtual Slice
+----+----+----+----+----+----+  i.e. &2 = pointer/reference to left array[0]
```
#### Adjacent arrays Mode
```
    Left Array   Right Array
+----+----+----+----+----+----+
| +---+---+---++---+---+---+  |  VirtualSlice reconstructs the parent array   
| | 2 | 4 | 6 || 1 | 3 | 5 |  |  out of the two adjacent array segments for 
| +---+---+---++---+---+---+  |  sequencial access
+----+----+----+----+----+----+  
    c            j
```
## Examples
### Merging two adjacent slices
```
use csx3::utils::VirtualSlice;
let v = &mut [1, 3, 5, 7, 9, 2, 4, 6, 8, 10];
let (s1, s2) = v.split_at_mut(5);

let mut v = VirtualSlice::new_adjacent(s1)
v.merge(s2);

assert_eq!(s1, &mut [1, 2, 3, 4, 5]);
assert_eq!(s2, &mut [6, 7, 8, 9, 10]);
```

### Access & swap contents out of two non-adjacent slices
```
use csx3::utils::VirtualSlice;

let s1 = &mut [1, 3, 5, 7, 9];
let _s3 = &mut [0, 0, 0, 0, 0];   // Stack wedge 
let s4 = &mut [2, 4, 6, 8, 10];

let mut v = VirtualSlice::new();

v.attach(s1);
v.attach(s4);

v[0] = 11;
v[5] = 9;
v.swap(0, 5);

assert_eq!(s1, &mut [9, 3, 5, 7, 9]);
assert_eq!(s4, &mut [11, 4, 6, 8 , 10]);
```
### Shallow merge across non-adjacent slices
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

assert_eq!(s1, &[1,2,3]);       // while s1 & s2 are unaffected
assert_eq!(s2, &[4,5,6,7]);
```
### Superimpose merged order
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