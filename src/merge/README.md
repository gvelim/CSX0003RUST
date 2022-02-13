# Sections
- [In-place Merge Algorithm using efficient swapping](#in-place-merge-algorithm-using-efficient-swapping)
- [Sequential access against multiple slice segments](#sequential-access-against-multiple-slice-segments)
- [Lazy merge and delayed slice mutability](#shallow-merge-across-non-adjacent-slices)

# In-place Merge Algorithm using efficient swapping
## General Approach
In an "In place" merge of two ordered arrays it is always required to maintain a pivot between merged and unmerged sub-arrays as we go over the process of
1. Use comparison indexes `(c,j)` to find the smallest element between (a) the left and (b) right ordered arrays
2. Swap the next smallest element of the left and right sub-arrays against a pivot position `(p)`
3. Repeat until we've exhausted comparing and swapping of all elements

```
Start                               Finish
==============================     ==========================
Left array       Right array       Ordered elements across arrays
+---+---+---+    +---+---+---+     +---+---+---+  +---+---+---+
| 1 | 3 | 5 | <> | 2 | 4 | 6 | =>  | 1 | 2 | 3 |  | 4 | 5 | 6 |
+---+---+---+    +---+---+---+     +---+---+---+  +---+---+---+
  c                j

Generic Approach of using a pivot to separate 
"merged" from "yet to be merged" regions
=============================================

         | left        Right  |
         | [c]         [j]    |
+---+---+|+---+---+   +---+---+ 
| 1 | 2 ||| 3 | 5 |   | 4 | 6 | 
+---+---+|+---+---+   +---+---+ 
Merged   |     Unmerged region  
region   p: Pivot
```

## Challenge
### Taking a naive first step
By trying to swap the smallest element of the two arrays with the pivot we quickly realise that things are getting out of control very soon. For example,

```
                              c  j  p   [c] > [j]  Action
 c/p             j            =======   =========  ========================
[(1 , 3 , 5)]  [(2 , 4 , 6)]  1  1  1    1     2   left swap(c,p) incr(c,p)
     c/p         j                  
[ 1 ,(3 , 5)]  [(2 , 4 , 6)]  2  1  2    3     2   right swap(j,p) incr(j,p)
      c   p          j                   
[ 1 , 2 ,(5 ]  [ 3),(4 , 6)]  2  2  3    2!!   4   Fail: We lost control here! 2 isn't part of the left array
```
At this stage our partitioned region is left of `p` and equal to `[1,2]` while the unmerged region is `[(5!!,3),(4,6)]` which is clearly **_out-of-order_** and the result from then on is unpredictable. During the 2nd iteration, the left comparison index `[c]` points to a `2` rather `3` which is now at the 4th position in the right array, or the 2nd position in the unmerged partition.
Therefore, we need to find a way to maintain a solid comparison index reference `[c]` for the left array while we iterate through

## Problem Solution
### Canceling the Rotation during right swaps
It becomes obvious that during the right swap operation our left array is rotated left as seen below
```
                              c  j  p   [c] > [j]  Action
 c/p             j            =======   =========  ========================
[(1 , 3 , 5)]  [(2 , 4 , 6)]  1  1  1    1     2   left swap(c,p) incr(c,p)
     c/p         j                   
[ 1 ,(3 , 5)]  [(2 , 4 , 6)]  2  1  2    3     2   right swap(j,p) incr(j,p)
      c   p          j                  
[ 1 , 2 ,(5 ]  [ 3),(4 , 6)]  <-- Here instead of [3,5] we have [5,3]
```
Moreover, the partition point `[p]` more or less points to the where the left comparison index `[c]` should have been, that is, the unmerged partition. Let's try this time with
* reverting the rotation effect after each right swap hence bringing the left unmerged part back to order
* using `[c]` as both the partition and the left comparison index
```
                              c  j    [c] > [j]  Action
  c              j            ====    =========  ============================
[(1 , 3 , 5)]  [(2 , 4 , 6)]  1  1     1     2   No swap, just incr(c)
      c          j                   
[ 1 ,(3 , 5)]  [(2 , 4 , 6)]  2  1     3     2   right swap(j,c), incr(c,j)
          c          j 
[ 1 , 2 ,(5 ]  [ 3),(4 , 6)]  3  2               rotate right by 1, from c to j excluded 
          c          j                   
[ 1 , 2 ,(3 ]  [ 5),(4 , 6)]  3  2     3     4   No swap, just incr(c)
                 c   j                   
[ 1 , 2 , 3 ]  [(5),(4 , 6)]  4  2     5     4   right swap(j,c), incr(c,j)
                     c   j                   
[ 1 , 2 , 3 ]  [ 4 ,(5),(6)]  5  3               rotate right by 1, from c to j excluded 
                     c   j                   
[ 1 , 2 , 3 ]  [ 4 ,(5),(6)]  5  3     5     6   No swap, just incr(c) 
                        c/j                   
[ 1 , 2 , 3 ]  [ 4 , 5 ,(6)]  6  3               c == j (!) nothing more to compare... we've finished !!
```
Nice! It works, but only on paper. Although we overcame the conflict between pivot `[p]` and left comparison index `[c]` the obvious issues here is that our indexing across the two arrays is broken. Definitely `6 == 3` isn't correct, because `[c]` has to operate in both arrays while `[j]` operates solely in the right array.

However, we do know that mergesort, performs merge on memory adjacent array segments hence this can be mitigated by reconstructing the parent array out of the two fragments so that, `working array = *left_array[0] .. *left_array[0] + (left_array.len() + right_array.len())`

```
Left Array    Right Array
+---+---+---+ +---+---+---+     
| 2 | 4 | 6 | | 1 | 3 | 5 |   Adjacent array segments
+---+---+---+ +---+---+---+     
  |   |   |    |   |   |
+---+---+---+---+---+---+     
|&2 |&4 |&6 |&1 |&3 |&5 |   Memory reconstructed and operated as a continuous array i.e.
+---+---+---+---+---+---+   we recast a slice with start pointer left_array[0] 
  c           j             and length = left (len + right len)*sizeof()

```
Let's repeat the example but through the memory reconstructed array.
```
                           c  j    [c] > [j]  Action
  c           j            ====    =========  ============================
[(1 , 3 , 5),(2 , 4 , 6)]  1  4     1     2   No swap, just incr(c)
      c       j                   
[ 1 ,(3 , 5),(2 , 4 , 6)]  2  4     3     2   right swap(j,c), incr(c,j)
          c       j 
[ 1 , 2 ,(5 , 3),(4 , 6)]  3  5               rotate right by 1, from c to j excluded
          c       j                   
[ 1 , 2 ,(3 , 5), 4 , 6)]  3  5     3     4   No swap, just incr(c)
              c   j                   
[ 1 , 2 , 3 ,(5),(4 , 6)]  4  6     5     4   right swap(j,c), incr(c,j)
                  c   j                   
[ 1 , 2 , 3 , 4 ,(5),(6)]  5  6               rotate right by 1, from c to j excluded 
                  c   j                   
[ 1 , 2 , 3 , 4 ,(5),(6)]  5  6     5     6   no swap, just incr(c) 
                     c/j                   
[ 1 , 2 , 3 , 4 , 5 , 6 ]  6  6               c == j (!) nothing more to compare... we've finished !!
```
So far so good. We have a working approach that however is dependent on adjacent-to-memory arrays for achieving the rotations

However, there are some things we need to be aware of
1. Rotations won't work between non-adjacent arrays without additional code complexity to deal with the gap
2. Rotation will be computationally significant against large datasets

So can we do better without need for rotations and non-adjacent to memory arrays ?

It appears that we can. `Virtual Slice` & `Index Reflector` come to the rescue.

## Virtual Slice - continuous access over array fragments
A `VirtualSlice` is composed out of one or more array fragments, adjacent to memory or not, and enables transparently operating over the **attached** array fragments.
```
Left Array       Right Array
+---+---+---+    +---+---+---+     
| 2 | 4 | 6 | <> | 1 | 3 | 5 |   Non-adjacent array segments
+---+---+---+    +---+---+---+     
  c       ^        j
          |__
       ...  | ...
+----+----+----+----+----+----+
| &2 | &4 | &6 | &1 | &3 | &5 |  Array of mutable references : Virtual Slice
+----+----+----+----+----+----+  i.e. &2 = pointer/reference to left array[0]
 p/c             j
```
While the VirtualSlice will ensure we can operate transparently over the array fragments, hence retain index consistency, we still need to tackle eliminating the costly rotations.

## Index Reflector - from absolute to derived indexing
We know that `[c]` and `[p]` indexes are getting mixed up, as right swaps tend to move `[c]` non-sequentially causing left merge to go **_out-of-order_**.


What if we could somehow, had a way such that when incrementing `c` by `1`, `c` points to the next in "logical order" element of the left array, 100% of the times and irrelevant of where `[c]` is positioned within the VirtualSlice ?

This is where the `IndexReflector` comes handy. The *Index Reflector* becomes the **absolute reference** in terms of the **ordered sequence** that `c` & `j` indexes have to follow and irrelevant of the non-sequential movement of `[c]` caused by every right swap.

```
Left Array       Right Array
+---+---+---+    +---+---+---+     
| 2 | 4 | 6 | <> | 1 | 3 | 5 |   Non-adjacent array segments
+---+---+---+    +---+---+---+     
          ^       
          |_
       ...  | ...
+----+----+----+----+----+----+
| &2 | &4 | &6 | &1 | &3 | &5 |  Virtual Slice with derived indexes
+----+----+----+----+----+----+  c' = Index Reflector[c], j' = Index Reflector[j]
 p/c'        |   j'    |    |
         ... | ...     |    |
+----+----+----+----+----+----+
| 1  | 2  | 3  | 4  | 5  | 6  |  Index Reflector captures VirtualSlice's elements latest  positions against their starting position
+----+----+----+----+----+----+  i.e. if IndexReflector[3] == 4, it would imply that VirtualSlice[4] was in the 3rd position
 p'/c            j               [p'] = x, such that Index Reflector[x] == p, where x E {c..j} 
                                 i.e. if p == 3 given IndexReflector[x] == 3, then p' == 5 if IndexReflector[5] == 3

```
In the diagram above, the Index Reflector holds the **starting position** of the VirtualSlice elements. Order Comparison indexes `[c]` and `[j]` are operated against the index reflector and are **projected** over to VirtualSlice as `[c']` and `[j']` using the transformations described in the diagram.

Reversely, Pivot index `[p]` is operated on the VirtualSlice and is projected over the Index Reflector as `[p']` using the transformation provided in the diagram.

Let's see how this is going to work; pay attention to the non-sequencial movements of `c'` and `p'`.
```
Phase 1: Merge the two arrays until a comparison index goes out of bounds 

Left Arr      Rght Arr       VirtualSlice                     Index Reflector                  Compare        Action
=========     ===========    =============================    =============================    ===========    ===================
                             c'/p          j'                  c/p'         j                  [c'] > [j']
[ 5, 6, 7] <> [ 1, 2, 3, 4]  [(5 , 6 , 7),(1 , 2 , 3 , 4)]    [(1 , 2 , 3),(4 , 5 , 6 , 7)]      5      1     swap(j', p), swap(j, p'), incr(p,j)
                                   p       c'  j'               c   p'          j                             
[ 1, 6, 7] <> [ 5, 2, 3, 4]  [ 1 ,(6 , 7 , 5),(2 , 3 , 4)]    [(4 , 2 , 3),(1 , 5 , 6 , 7)]      5      2     swap(j', p), swap(j, p'), incr(p,j) 
                                       p   c'      j'           c       p'          j                             
[ 1, 2, 7] <> [ 5, 6, 3, 4]  [ 1 , 2 ,(7 , 5 , 6),(3 , 4)]    [(4 , 5 , 3),(1 , 2 , 6 , 7)]      5      3     swap(j', p), swap(j, p'), incr(p,j)
                                          c'/p         j'      c/p'                     j                             
[ 1, 2, 3] <> [ 5, 6, 7, 4]  [ 1 , 2 , 3 ,(5 , 6 , 7),(4)]    [(4 , 5 , 6),(1 , 2 , 3 , 7)]      5      4     swap(j', p), swap(j, p'), incr(p,j)
                                               p       c'  j'   c   p'                       j                             
[ 1, 2, 3] <> [ 4, 6, 7, 5]  [ 1 , 2 , 3 , 4 ,(6 , 7 , 5)]    [(7 , 5 , 6),(1 , 2 , 3 , 4)]      x      x     <-- j'/j got out of bounds ! Phase 1 completed
```
We ran-out of right array elements (`j`is over bound), which means anything below `[p]` is merged and anything including and above `[p]` just needs to be carried over. But we cannot complete as we have **_out-of-order_** elements in the unmerged partition.

Index Reflector to the rescue!

The index reflector tells us exactly what we need to do to complete the work. if you look at `[c .. left_array.len()]` / `[7,5,6]` in the index reflector, it tells us
1. next comes the 7th element from virtual slice,
2. then the 5th element from virtual slice, and
3. finally, the 6th element from virtual slice

So if we get the remainder from the VirtualSlice `[6,7,5]` and apply the above steps we'll get `[5,6,7]`. Nice !! Let's see it in action.
```
Phase 2: Finishing off the remainder unmerged partition

Left Arr      Right Arr      VirtualSlice                     Index Reflector                  Compare        Action
=========     ===========    =============================    =============================    ===========    ===================
                                               p       c'  j'   c   p'                       j                             
[ 1, 2, 3] <> [ 4, 6, 7, 5]  [ 1 , 2 , 3 , 4 ,(6 , 7 , 5)]    [(7 , 5 , 6),(1 , 2 , 3 , 4)]      x      x     swap(c', p), swap(c, p') incr(i,c)
                                                   p   c'  j'       c   p'                   j                             
[ 1, 2, 3] <> [ 4, 5, 7, 6]  [ 1 , 2 , 3 , 4 , 5 ,(7 , 6)]    [(5 , 7 , 6),(1 , 2 , 3 , 4)]      x      x     swap(c', p), swap(c, p') incr(i,c)
                                                     c'/p  j'          c/p'                  j                             
[ 1, 2, 3] <> [ 4, 5, 6, 7]  [ 1 , 2 , 3 , 4 , 5 , 6 ,(7)]    [(5 , 6 , 7),(1 , 2 , 3 , 4)]      x      x     <-- We finished ! c' and p are both on the last position
```
Phase 2 is now complete. **As if by magic** everything is now in position and ordered after `O(n+m)` iterations

## Useful Index Reflector Properties
1. At completion the Index Reflector **reflects** the final position per element and given its starting order i.e the 4th element in VirtualSlice ends up in the 1st position, the 1st in the 5th, and so on
```
  Left Arr      Right Arr      VirtualSlice                     Index Reflector                  
  =========     ===========    =============================    =============================    
                               c'/p          j'                  c/p'         j                  
  [ 5, 6, 7] <> [ 1, 2, 3, 4]  [ 5 , 6 , 7 , 1 , 2 , 3 , 4 ]    [ 1 , 2 , 3 , 4 , 5 , 6 , 7 ]      
  ...
  ...
                                                        p/c' j'          c/p'                  j                             
  [ 1, 2, 3] <> [ 4, 5, 6, 7]  [ 1 , 2 , 3 , 4 , 5 , 6 , 7 ]    [ 5 , 6 , 7 , 1 , 2 , 3 , 4 ]      
```
2. `[c]` index is bound by `[0 .. left array.len]` range
3. `[p']` index is bound by `[c .. left array.len]` range
4. Always `[j'] == [j]`

## Optimisations & other uses
1. Given the 4th property we can reduce the Index Reflector to `left_array.len()` reducing the memory requirements from 2(n+m) to (2n+m) in case of mergesort
2. In addition to 4th property and given the arrays are adjacent the VirtualSlice becomes a pointer to a reconstructed parent array hence the overall memory impact becomes O(n) * sizeof(usize)
3. Given the 1st property we can
  1. Develop a "sort mask array" through which we can access the source array segments in order and without the need of permanently mutating them
    1. [VirtualSlice::merge_shallow](../merge/README.md?3)
  2. Such "sort mask" can be imposed or "played onto" the source segments hence mutating them only when is needed
    1. [VirtualSlice::impose_shallow_merge](../merge/README.md?4)


# Sequential access against multiple slice segments
A `VirtualSlice` is composed out of one or more slice segments, adjacent to memory or not, and enables transparently operating over them. 

The virtualslice operates in two mode. Adjacent and non-adjacent.

It solves the following needs:

* Need to access two or more slices as a single continuous one
* Need to use memory efficiently for cases where such slices are adjacent in memory
* Need to perform a merge of two or more ordered slices
* Need to perform "shallow" or lazy merge, that is, 
  * provide ordered access to the underlying slices without mutating them (see shallow merge)
  * while allow such order to be superimposed upon the slices if we later decide to (see superimpose)

## Memory layout
### Non-Adjacent arrays Mode
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
### Adjacent arrays Mode
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
### Merging two adjacent slices O(n+m)
* No additional memory used for holding references
* Uses (n + m) * usize for dynamic indexing
  * can be further optimised to hold only (n) * size of additional memory
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
* Uses n + m memory for holding references 
* Uses (n + m) * usize for dynamic indexing 
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
### Superimpose merged order O(n+m-1)
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