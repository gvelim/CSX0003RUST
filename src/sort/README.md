# Sorting Algorithms
## Quick Sort
* Mutable sorting of input array using randomised pivot for partitioning
## Merge Sort
Versions
1. Immutable sorting returning a sorted copy of the input
   * Performance: O(n log n) 
   * Memory: O(n log n) * heap alloc/dealloc sizeof(n)
2. Mutable sorting with **_in-place_** merge for
    * Adjacent ordered slices : `fn merge_mut_adjacent()`
      * Use of Rotation/shift (no swapping)
      * Performance: O(n * n/2 rotations per iteration), 
      * Memory: no additional memory
    * Non-adjacent ordered slices : `fn merge_mut()`
      * Use of intelligent Swaps 
      * Performance: O(n)
      * Memory: 1.5 * 0(n) * usize

## In-place Merge Algorithm with intelligent swapping
### General Approach
"In place" merge of two ordered sets requires to maintain always a pivot between merged and unmerged areas as we 
1. use indexes `(c,j)` to find the smallest element between (a) the left and (b) right ordered arrays
2. Swap the smallest element of the compared elements with the pivot (p) position
3. Repeat until we've exhausted comparing and swapping all elements 

```
Start                               Finish
==============================     ==========================
Left Set         Right Set          Merged sets
+---+---+---+    +---+---+---+     +---+---+---+---+---+---+
| 1 | 3 | 5 | <> | 2 | 4 | 6 | =>  | 1 | 2 | 3 | 4 | 5 | 6 |
+---+---+---+    +---+---+---+     +---+---+---+---+---+---+
 [c]              [j]

Generic Approach of using a pivot to separate 
"merged" from "yet to be merged" regions
=============================================

         | left       Right  |
         | [c]        [j]    |
+---+---+|+---+---+  +---+---+ 
| 1 | 2 ||| 3 | 5 |  | 4 | 6 | 
+---+---+|+---+---+  +---+---+ 
Merged   |   Unmerged region |
         p: Pivot
```

### Challenge - Swapping to disorder
The following challenges need to be addressed
1. The pivot has to move from start `[0]` to end `[left.len + right.len]`, which means we don't have **_continuity_** without added code complexity
2. Swapping, sooner or later, will cause the left unmerged region to become **_out-of-order_** hence the `[c]` index will not pick the next in order item for comparison

### Overcoming the challenges
#### Virtual Slice - from fragmented to linear access

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
