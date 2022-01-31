# Sorting Algorithms
## Quick Sort
* Mutable sorting of input array using randomised pivot for partitioning
## Merge Sort
Versions
1. Immutable sorting returning a sorted copy of the input
   * Performance: O(n log n) 
   * Memory: O(n log n) * heap alloc/dealloc sizeof(n)
2. Mutable sorting with **_in-place_** merge operations
   * Overall Performence:O(n log n)
   * Approach to in-place merge operations, 
      * Merge memory adjacent sub-arrays : `fn merge_mut_adjacent()`
        * Use of Rotation/shift 
        * Performance: O(n * n/2 rotations per iteration), 
        * Memory: no additional memory
      * Merge Non adjacent sub-arrays : `fn merge_mut()`
        * Use of intelligent Swaps 
        * Performance: O(n)
        * Memory: 1.5 * 0(n) * usize

## In-place Merge Algorithm with intelligent swapping
### General Approach
In an "In place" merge of two ordered arrays it is always required to maintain a pivot between merged and unmerged sub-arrays as we 
1. use indexes `(c,j)` to find the smallest element between (a) the left and (b) right ordered arrays
2. Swap the next smallest element of the left and right sub-arrays with the pivot (p) position
3. Repeat until we've exhausted comparing and swapping all elements 

```
Start                               Finish
==============================     ==========================
Left array       Right array       Ordered elements across arrays
+---+---+---+    +---+---+---+     +---+---+---+  +---+---+---+
| 1 | 3 | 5 | <> | 2 | 4 | 6 | =>  | 1 | 2 | 3 |  | 4 | 5 | 6 |
+---+---+---+    +---+---+---+     +---+---+---+  +---+---+---+
 [c]              [j]

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

### Challenge
#### Taking a naive first step
By trying to swap the smallest element of the two arrays with the pivot we quickly realise that things are getting out of control very soon. For example,

```
[c/p]           [j]           c  j  p   [c] > [j]  Action
[(1 , 3 , 5)]  [(2 , 4 , 6)]  1  1  1    1     2   left swap(c,p)  <-- move 1:[c] to pivot [p] and increase c & p by one
    [c/p]       [j]                  
[ 1 ,(3 , 5)]  [(2 , 4 , 6)]  2  1  2    3     2   right swap(j,p)  <-- move 2:[j] to pivot [p] and increase j & p  by one
     [c] [p]        [j]                  
[ 1 , 2 ,(5 ]  [ 3),(4 , 6)]  2  2  3    2!!   4   Fail: We lost control here! 2 isn't part of the left array
```
At this stage our partitioned region is `[1,2]` while the unmerged region is `[(5!!,3),(4,6)]` which is clearly **_out-of-order_** and the result from then on is unpredictable. During the 2nd iteration, the left comparison index `[c]` points to a `2` rather `3` which is now at the 4th position in the right array, or the 2nd position in the unmerged partition. 
Therefore, we need to find a way to maintain a solid comparison index reference `[c]` for the left array while we iterate through

### Probelm Solution
#### Canceling the Rotation during right swaps
It becomes obvious that during the right swap operation our left array is rotated left as seen below
```
[c/p]           [j]           c  j  p   [c] > [j]  Action
[(1 , 3 , 5)]  [(2 , 4 , 6)]  1  1  1    1     2   left swap(c,p), inc c & p by 1
    [c/p]       [j]                  
[ 1 ,(3 , 5)]  [(2 , 4 , 6)]  2  1  2    3     2   right swap(j,p)  <-- move 2:[j] to pivot [p] and move j & p  by one
     [c] [p]        [j]                  
[ 1 , 2 ,(5 ]  [ 3),(4 , 6)]  <-- Here instead of [3,5] we have [5,3]
```
Moreover, the partition point `[p]` and left index reference `[c]` more or less point to the start of the left array, that is, the unmerged partition. Let's try this time with 
* reverting the rotation hence bringing the left sub array back to order
* using `[c]` as both the partition and the left comparison index
```
 [c]            [j]           c  j    [c] > [j]  Action
[(1 , 3 , 5)]  [(2 , 4 , 6)]  1  1     1     2   No swap, just inc c by 1
     [c]        [j]                  
[ 1 ,(3 , 5)]  [(2 , 4 , 6)]  2  1     3     2   right swap(j,p), inc c & j by 1
         [c]        [j]
[ 1 , 2 ,(5 ]  [ 3),(4 , 6)]  3  2               revert left rotation with rotate_right(c .. j-1] (from c to j excluded) by 1 ) 
         [c]        [j]                  
[ 1 , 2 ,(3 ]  [ 5),(4 , 6)]  3  2     3     4   No swap, just inc c by 1
                [c] [j]                  
[ 1 , 2 , 3 ]  [(5),(4 , 6)]  4  2     5     4   right swap(j,p), inc c & j by 1
                    [c] [j]                  
[ 1 , 2 , 3 ]  [ 4 ,(5),(6)]  5  3               revert left rotation with rotate_right(c .. j-1] (from c to j excluded) by 1 ) 
                    [c] [j]                  
[ 1 , 2 , 3 ]  [ 4 ,(5),(6)]  5  3     5     6   no swap, just inc c by 1 
                       [c/j]                  
[ 1 , 2 , 3 ]  [ 4 , 5 ,(6)]  6  3               c == j (!) nothing more to compare... we've finished !!
```
Nice! It works, but only on paper. Although we overcame the confict between pivot `[p]` and left index reference `[c]` the obvious issues here are:
1. Our indexing across the two arrays is broken. Definitely `6 == 3` isn't correct, because `[c]` has to operate in both arrays while `[j]` operates solely in the right array. 

However we do know that mergesort, performs merge on memory adjacent arrays hence (2) is somehow mitigated by deriving a continuous array out of the two so that, `working array = pointer to left_array[0] .. pointer to [left_array.len() + right_array.len()]`

```
Left Array    Right Array
+---+---+---+ +---+---+---+     
| 2 | 4 | 6 | | 1 | 3 | 5 |   Adjacent array segments
+---+---+---+ +---+---+---+     
  |   |   |    |   |   |
+---+---+---+---+---+---+     
|&2 |&4 |&6 |&1 |&3 |&5 |   Memory reconstructed and operated as a continuous array i.e.
+---+---+---+---+---+---+   we recast a slice with start pointer left_array[0] 
 [c]         [j]            and length = left (len + right len)*sizeof()

```
Let's repeat the example but through the memory reconstructed array.
```
 [c]         [j]           c  j    [c] > [j]  Action
[ 1 , 3 , 5 , 2 , 4 , 6 ]  1  4     1     2   No swap, just inc c by 1
     [c]     [j]                  
[ 1 , 3 , 5 , 2 , 4 , 6 ]  2  4     3     2   right swap(j,p), inc c & j by 1
         [c]    [j]
[ 1 , 2 ,(5, 3), 4 , 6 ]  3  5                revert left rotation with rotate_right(c .. j-1] (from c to j excluded) by 1 ) 
         [c]     [j]                  
[ 1 , 2 , 3 , 5 , 4 , 6 ]  3  5     3     4   No swap, just inc c by 1
             [c] [j]                  
[ 1 , 2 , 3 , 5 , 4 , 6 ]  4  6     5     4   right swap(j,p), inc c & j by 1
                 [c] [j]                  
[ 1 , 2 , 3 , 4 ,(5), 6 ]  5  6               revert left rotation with rotate_right(c .. j-1] (from c to j excluded) by 1 ) 
                 [c] [j]                  
[ 1 , 2 , 3 , 4 , 5 , 6 ]  5  6     5     6   no swap, just inc c by 1 
                    [c/j]                  
[ 1 , 2 , 3 , 4 , 5 , 6 ]  6  6               c == j (!) nothing more to compare... we finished !!
```
So far so good. We have a working approach that however is dependent on adjacent-to-memory arrays for achieving the rotations

However, there are some things we need to be aware of
1. Rotations won't work between non-adjacent arrays without additional code complexity to deal with the gap
2. Rotation will be computationaly significant against large datasets

So can we do better without need for rotations and non-adjacent to memory arrays ?

It appears that we can. `Virtual Slice` & `Index Reflector` come to the rescue.

### Virtual Slice - continuous access over array fragments
VirtualSlice is composed out of one or more array fragments, adjacent to memory or not, and enables transparently operating over the array contents.
```
Left Array       Right Array
+---+---+---+    +---+---+---+     
| 2 | 4 | 6 | <> | 1 | 3 | 5 |   Non-adjacent array segments
+---+---+---+    +---+---+---+     
 [c]      ^       [j]
          |__
       ...  | ...
+----+----+----+----+----+----+
| &2 | &4 | &6 | &1 | &3 | &5 |  Array of mutable references : Virtual Slice
+----+----+----+----+----+----+  i.e. &2 = pointer/reference to left array[0]
[p][c]           [j]
```
While the VirtualSlice will ensure we can operate transparently over the array fragments, hence retain index consistency, we still need to tackle eliminating the costly rotations.

### Index Reflector - from absolute to derived indexing
We know that `[c]` and `[p]` indices are getting mixed up, as right swaps tend to move `[c]` non-sequencially causing left merge to go **_out-of-order_**. What if we could somehow, become 100% certain of the next in order eleement pointed by `[c]` and irrelevant of its position in the VirtualSlice ? 

This is where the *Index Reflector* comes handy. The *Index Reflector* becomes our solid reference in terms of the **right ordered sequence** and irrelevant of the non-sequential movement of `[c]` after each right swap.

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
+----+----+----+----+----+----+  [c'] = Index Reflector[c], [j'] = Index Reflector[j]
[p][c']      |  [j']   |    |
         ... | ...     |    |
+----+----+----+----+----+----+
| 1  | 2  | 3  | 4  | 5  | 6  |  Index Reflector captures current index positions against inital ordered sequence
+----+----+----+----+----+----+  i.e. [3] indicates &6 is in 3rd "starting" position, 
[p'][c]         [j]                   thereafter, `[3]` indicates latest position of &6 within VirtualSlice 
                                 [p'] such that Index Reflector[x] == p, where x {c..j} 

```
In the above diagram, the index reflector holds the starting position of the VirtualSlice elements. Order Comparison indices `[c]` and `[j]` are operated against the index reflector and are **projected** over to VirtualSlice as `[c']` and `[j']`. Reversely, Pivot index `[p]` is operated on the VirtualSlice and is projected over the Index Reflector as `[p']`.

Let see how this is going to work.
```
Phase 1: Merge the two arrays until a comparison index goes out of bounds 

Slice 1       Slice 2      VirtualSlice                       Index Reflector                  Compare        Action
=========     ===========  ===============================    =============================    ===========    ===================
                             c'/i          j'                  c/i'         j                  [c'] > [j']
[ 5, 6, 7] <> [ 1, 2, 3, 4]  [ 5 , 6 , 7 , 1 , 2 , 3 , 4 ]    [ 1 , 2 , 3 , 4 , 5 , 6 , 7 ]      5      1     swap(j', i), swap(j, i'), incr(i,j)
                                   i       c'  j'               c   i'          j                             
[ 1, 6, 7] <> [ 5, 2, 3, 4]  [ 1 , 6 , 7 , 5 , 2 , 3 , 4 ]    [ 4 , 2 , 3 , 1 , 2 , 6 , 7 ]      5      2     swap(j', i), swap(j, i'), incr(i,j) 
                                       i   c'      j'           c       i'          j                             
[ 1, 2, 7] <> [ 5, 6, 3, 4]  [ 1 , 2 , 7 , 5 , 6 , 3 , 4 ]    [ 4 , 5 , 3 , 1 , 2 , 3 , 7 ]      5      3     swap(j', i), swap(j, i'), incr(i,j)
                                          c'/i         j'     c/i'                      j                             
[ 1, 2, 3] <> [ 5, 6, 7, 4]  [ 1 , 2 , 3 , 5 , 6 , 7 , 4 ]    [ 7 , 5 , 6 , 1 , 2 , 3 , 4 ]      5      4     swap(j', i), swap(j, i'), incr(i,j)
                                               i       c'  j'   c       i'                   j                             
[ 1, 2, 3] <> [ 4, 6, 7, 5]  [ 1 , 2 , 3 , 4 , 6 , 7 , 5 ]    [ 7 , 5 , 6 , 1 , 2 , 3 , 7 ]      x      x     swap(j', i), swap(j, i'), incr(i,j)
```
We ran-out of right array elements (`j`is over bound), which means anything below `[i]` is merged and anything including and above `[i]` just needs to be carried over. But we cannot complete as we have out-of-order elements hanging in the right unmerged partition.

Index Reflector to the rescue!

The index reflector tells us exactly what we need to do to complete the work. if you look at `[c .. i']` / `[7,5,6]` in the index reflector, it tells us to 
1. first get the 7th element from virtual slice, then
2. get the 5th element from virtual slice, and 
3. finally, get the 6th element from virtual slice

So if we get the remainder from the VirtualSlice `[6,7,5]` and apply the above steps we'll get `[5,6,7]`. Nice !! Let's see it in action.
```
Phase 2: Finishing off the remainder unmerged partition

Slice 1       Slice 2      VirtualSlice                       Index Reflector                  Compare        Action
=========     ===========  ===============================    =============================    ===========    ===================
                                               i       c'  j'   c   i'                       j                             
[ 1, 2, 3] <> [ 4, 6, 7, 5]  [ 1 , 2 , 3 , 4 , 6 , 7 , 5 ]    [ 7 , 5 , 6 , 1 , 2 , 3 , 7 ]      x      x     swap(c', i), swap(c, i') incr(i,c)
                                               5    i   c'  j'       c   i'                   j                             
[ 1, 2, 3] <> [ 4, 5, 7, 6]  [ 1 , 2 , 3 , 4 , 5 , 7 , 6 ]    [ 5 , 7 , 6 , 1 , 2 , 3 , 7 ]      x      x     swap(c', i), swap(c, i') incr(i,c)
                                                      i/c' j'          c/i'                   j                             
[ 1, 2, 3] <> [ 4, 5, 6, 7]  [ 1 , 2 , 3 , 4 , 5 , 6 , 7 ]    [ 5 , 6 , 7 , 1 , 2 , 3 , 7 ]      x      x     swap(c', i), swap(c, i') incr(i,c)
```
**As if by magic** everything is now in position and ordered after `O(n)` iterations

### Useful Index Reflector Properties
1. At completion the Index Reflector "reflects" the final position per element and given its starting order i.e the 4th element in virtualslice ends up in the 1st position, the 1st in the 5th, and so on
```
  Slice 1       Slice 2      VirtualSlice                       Index Reflector                  
  =========     ===========  ===============================    =============================    
                               c'/i          j'                  c/i'         j                  
  [ 5, 6, 7] <> [ 1, 2, 3, 4]  [ 5 , 6 , 7 , 1 , 2 , 3 , 4 ]    [ 1 , 2 , 3 , 4 , 5 , 6 , 7 ]      
  ...
  ...
                                                        i/c' j'          c/i'                   j                             
  [ 1, 2, 3] <> [ 4, 5, 6, 7]  [ 1 , 2 , 3 , 4 , 5 , 6 , 7 ]    [ 5 , 6 , 7 , 1 , 2 , 3 , 7 ]      
```
2. `[c]` index is bound by `[0 .. left array.len]` range 
3. `[i']` index is bound by `[c .. left array.len]` range
4. Always `[j'] == [j]` 

### Index Reflector Optimisations
1. Given the 4th property we can reduce the Index Reflector to `left_array.len()` reducing the additional memory required by half in case of mergesort
2. Given the 1st property we can 
   1. Develop a "sort mask array" through which we can access the source array segments in order and without the need of permanently mutating them.
   2. Such "sort mask" can be imposed or "played onto" the source segments hence mutating them only when is needed
