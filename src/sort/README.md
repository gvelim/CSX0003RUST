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
      * Merge memory adjacent sub-arrays : `fn merge_mut_adjacent()`
        * Use of intelligent swaps 
        * Performance: O(n+m), 
        * Memory impact: O(n) * usize
      * Merge Non-adjacent sub-arrays : `fn merge_mut()`
        * Use of intelligent Swaps 
        * Performance: O(n+m)
        * Memory: O(2n+m) * usize

