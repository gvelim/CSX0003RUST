# Sorting Algorithms
## Quick Sort
* Mutable sorting of input array using randomised pivot for partitioning
## Merge Sort
Versions
1. Immutable sorting returning a sorted copy of the input
   * Performance: O(n log n) 
   * Memory: O(n log n) * heap alloc/dealloc sizeof(n)
2. Mutable sorting with **_in-place_** merge operations
   * Overall Performance: O(n log n) with intelligent swapping
   * Approach to in-place merge operations, 
      * [In-place Merge Algorithm using efficient swapping](../merge/README.md#in-place-merge-algorithm-using-efficient-swapping)
      * O(n+m) performance
      * [Merge memory adjacent sub-arrays](../merge/README.md#sequential-access-against-multiple-slice-segments)
        * `fn merge_mut_adjacent()`
        * Memory impact: O(n) * usize
      * [Merge Non-adjacent sub-arrays](../merge/README.md#sequential-access-against-multiple-slice-segments)
        * `fn merge_mut()`
        * Memory: O(2n+m) * usize

