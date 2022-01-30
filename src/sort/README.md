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
"In place" merge of two ordered arrays requires to maintain always a pivot between merged and unmerged sub-arrays as we 
1. use indexes `(c,j)` to find the smallest element between (a) the left and (b) right ordered arrays
2. Swap the next smallest element of the left and right sub-arrays with the pivot (p) position
3. Repeat until we've exhausted comparing and swapping all elements 

```
Start                               Finish
==============================     ==========================
Left array       Right array       Merged array
+---+---+---+    +---+---+---+     +---+---+---+---+---+---+
| 1 | 3 | 5 | <> | 2 | 4 | 6 | =>  | 1 | 2 | 3 | 4 | 5 | 6 |
+---+---+---+    +---+---+---+     +---+---+---+---+---+---+
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
#### Taking a naive approach
By trying to swap the smallest element of the two arrays with the pivot we quickly realise that things get are getting out of control very soon. For example,

```
[c/p]           [j]           c  j  p   [c] > [j]  Action
[ 1 , 3 , 5 ]  [ 2 , 4 , 6 ]  1  1  1    1     2   left swap(c,p)  <-- move 1:[c] to pivot [p] and move c & p by one
    [c/p]       [j]                  
[ 1 , 3 , 5 ]  [ 2 , 4 , 6 ]  2  1  2    3     2   right swap(j,p)  <-- move 2:[j] to pivot [p] and move j & p  by one
     [c] [p]        [j]                  
[ 1 , 2 , 5 ]  [ 3 , 4 , 6 ]  2  2  3    2!!   4   Fail: We lost control here! 2 isn't part of the left array
```
At this stage our partitioned region is `[1,2]` while the unmerged region is `[5!!,3,4,6]` which is clearly **_out-of-order_** and the result from then on, unpredictable. During the 2nd iteration, left comparison index `[c]` points to a `2` rather `3` which is now at the 4th position in the right array, or 2nd position in the unmerged partition. 
Therefore, we need to find a way to maintain a solid comparison index reference [c] for the left array while we iterate

### Solution Options
#### Canceling the Rotation during right swaps
It becomes obvius that during right swap operation our left array is somehow rotated left, so if we take a look
```
[c/p]           [j]           c  j  p   [c] > [j]  Action
[ 1 , 3 , 5 ]  [ 2 , 4 , 6 ]  1  1  1    1     2   left swap(c,p), inc c & p by 1
    [c/p]       [j]                  
[ 1 , 3 , 5 ]  [ 2 , 4 , 6 ]  2  1  2    3     2   right swap(j,p)  <-- move 2:[j] to pivot [p] and move j & p  by one
     [c]  [p]          [j]                  
[ 1 , 2 , (5 ]  [ 3) , 4 , 6 ]  <-- Here instead of [3,5] we have [5,3]
```
Moreover, the partition point [p] and left index reference [c] are more or less point to the start of the left array or unmerge partition. Let's try this time with 
* reverting the rotation and
* use [c] as both partition and left comparison index
```
 [c]            [j]           c  j    [c] > [j]  Action
[ 1 , 3 , 5 ]  [ 2 , 4 , 6 ]  1  1     1     2   No swap, just inc c by 1
     [c]        [j]                  
[ 1 , 3 , 5 ]  [ 2 , 4 , 6 ]  2  1     3     2   right swap(j,p), inc c & j by 1
         [c]        [j]
[ 1 , 2 ,(5 ]  [ 3), 4 , 6 ]  3  2               revert left rotation with right rotate [c .. j] (from c to j excluded) 
         [c]        [j]                  
[ 1 , 2 , 3 ]  [ 5 , 4 , 6 ]  3  2     3     4   No swap, just inc c by 1
                [c] [j]                  
[ 1 , 2 , 3 ]  [ 5 , 4 , 6 ]  4  2     5     4   right swap(j,p), inc c & j by 1
                    [c] [j]                  
[ 1 , 2 , 3 ]  [ 4 ,(5), 6 ]  5  3               revert left rotation with right rotate [c .. j] (from c to j excluded) 
                    [c] [j]                  
[ 1 , 2 , 3 ]  [ 4 ,(5), 6 ]  5  3     5     6   no swap, just inc c by 1 
                       [c/j]                  
[ 1 , 2 , 3 ]  [ 4 , 5 , 6 ]  6  3               c == j nothing more to compare... we finished !!
```
Nice! It works, but on paper. Although we overcame the confict between pivot [p] and left index reference [c] the obvious issue here is that our indexing across the two arrays is broken. Definately 6 == 3 isn't correct, because [c] has to operate in both arrays while [j] lives in the right array only. 
So what if we could operate across both arrays as if it was one ? VirtualSlice comes to the rescue.

### Virtual Slice - from fragmented to linear access

```
Left Array       Right Array
+---+---+---+    +---+---+---+     
| 2 | 4 | 6 | <> | 1 | 3 | 5 |   Non-adjacent array segments
+---+---+---+    +---+---+---+     
 [c]      ^       [j]
          |_
       ...  | ...
+----+----+----+----+----+----+
| &2 | &4 | &6 | &1 | &3 | &5 |  Array of mutable references : Virtual Slice
+----+----+----+----+----+----+  i.e. &2 = pointer/reference to left array[0]
[p][c]           [j]
```

#### Index Reflector - from absolute to derived indexing

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
| 1  | 2  | 3  | 4  | 5  | 6  |  Index Reflector captures latest index positions against "fixed" starting positions
+----+----+----+----+----+----+  i.e. [3] indicates &6 is in 3rd "starting" position, 
[p'][c]         [j]                   thereafter, `[3]` indicates latest position of &6 within VirtualSlice 
                                 [p'] such that Index Reflector[x] == p, where x {c..j} 

```

### Observations / Properties
1. At completion the Index Reflector "reflects" the final positions given the starting order
2. `[c]` index is bound by `[0 .. left array.len]` range 
3. `[i']` index is bound by `[c .. left array.len]` range
4. Always `[j'] == [j]` 

### Index Reflector Optimisations
1. Given the 4th property we can reduce the Index Reflector to left_array.len() reducing the additional memory required
2. Given the 1st property we can 
   1. Develop a "sort mask array" through which we can access the source array segments in order and without the need of permanently mutating them.
   2. Such "sort mask" can be imposed or "played onto" the source segments hence mutating them only when is needed
