# Merge Sort

## Implementation Flavours

### Immutable sorting returning a sorted copy of the input
   * Performance: O(n log n) 
   * Memory: O(n log n) * heap alloc/dealloc sizeof(n)
   
### Mutable sorting using in-place merge operations
   * Performance: O(n log n)
   * Approach to in-place merge operations, 
      * [In-place Merge Algorithm using efficient swapping](./merge_in_place.md)
      * O(n+m) performance
      * [Merge memory adjacent sub-arrays](./merge_sequencial_access.md)
        * `fn merge_mut_adjacent()`
        * Memory impact: O(n) * usize
      * [Merge Non-adjacent sub-arrays](./merge_sequencial_access.md)
        * `fn merge_mut()`
        * Memory: O(2n+m) * usize

