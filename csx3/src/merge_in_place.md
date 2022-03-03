# In-place Merge Algorithm with O(n+m) swaps
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
The VirtualSlice enables transparent operation over the array fragments, hence enable us to retain index consistency, we still need to tackle eliminating the costly rotations. For more detail go to the [internals and sequential access section](merge_sequencial_access.md) 

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
1. Given the non-sequential movement of `p'` and for less than 500 elements we can map `p -> p'` by searching serially within the `[c .. left_array.len()]` range. However, this approach and under larger sets, has O(n^2) performance hence an alternative is to forward pre-calculate `p'` during each swap. This eliminates the nested search loop which results to 10x increase of performance (current implementation)
2. Given the 4th property we can reduce the Index Reflector to `left_array.len()` reducing the memory requirements from 2(n+m) to (2n+m) in case of mergesort
3. In addition to 4th property and given the arrays are adjacent the VirtualSlice becomes a pointer to a reconstructed parent array hence the overall memory impact becomes O(n) * sizeof(usize)
4. Given the 1st property we can
5. Develop a "sort mask array" through which we can access the source array segments in order and without the need of permanently mutating them
   1. [VirtualSlice::merge_shallow](./merge_lazy.md)
6. Such "sort mask" can be imposed or "played onto" the source segments hence mutating them only when is needed
   1. [VirtualSlice::impose_shallow_merge](./merge_lazy.md)

