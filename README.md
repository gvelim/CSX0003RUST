# CSX0003RUST

Rust playground; familiarising with ownership, generics, trait objects, etc

- Select (Random, Deterministic)
- Merge (linear in-place, memory adjacent/non-adjacent, lazy merge)
  - [In-place Merge Algorithm using efficient swapping](./src/merge/README.md#in-place-merge-algorithm-using-efficient-swapping)
  - [Sequential access against multiple slice segments](./src/merge/README.md#sequential-access-against-multiple-slice-segments)
  - [Lazy merge and delayed slice mutability](./src/merge/README.md#sequential-access-against-multiple-slice-segments)
- [Merge sort (mutable slices, in-place merging)](./src/sort/README.md)
- [Quick sort (mutable slices, in-place partitioning)](./src/sort/README.md)
- Linked list 
- Binary tree


Sample output:
```
List           : [-104, 104, 123, 107, 89, 63, 5, -28]
>>
Random Selection
    1st order stat= -104
    2nd order stat= -28
    3rd order stat= 5
>>
MergeSort Immut: (19, [-104, -28, 5, 63, 89, 104, 107, 123])

>>
    Input: (8)[-104, 104, 123, 107, 89, 63, 5, -28] =>
    Input: (4)[-104, 104, 123, 107] =>
    Input: (2)[-104, 104] =>
    Input: (2)[123, 107] =>
    Merge Input: [-104, 104],[107, 123]
        -:Merge:[-104, 104]<>[107, 123] => [-104, 104, 107, 123] :: [0, 1, 2, 3] (0,2,0)
        l:Merge:[-104, 104]<>[107, 123] => [-104, 104, 107, 123] :: [0, 1, 2, 3] (1,2,1)
        l:Merge:[-104, 104]<>[107, 123] => [-104, 104, 107, 123] :: [0, 1, 2, 3] (2,2,2)
    Input: (4)[89, 63, 5, -28] =>
    Input: (2)[89, 63] =>
    Input: (2)[5, -28] =>
    Merge Input: [63, 89],[-28, 5]
        -:Merge:[63, 89]<>[-28, 5] => [63, 89, -28, 5] :: [0, 1, 2, 3] (0,2,0)
        r:Merge:[-28, 89]<>[63, 5] => [-28, 89, 63, 5] :: [2, 1, 0, 3] (1,3,0)
        r:Merge:[-28, 5]<>[63, 89] => [-28, 5, 63, 89] :: [2, 3, 0, 1] (2,4,0)
    Merge Input: [-104, 104, 107, 123],[-28, 5, 63, 89]
        -:Merge:[-104, 104, 107, 123]<>[-28, 5, 63, 89] => [-104, 104, 107, 123, -28, 5, 63, 89] :: [0, 1, 2, 3, 4, 5, 6, 7] (0,4,0)
        l:Merge:[-104, 104, 107, 123]<>[-28, 5, 63, 89] => [-104, 104, 107, 123, -28, 5, 63, 89] :: [0, 1, 2, 3, 4, 5, 6, 7] (1,4,1)
        r:Merge:[-104, -28, 107, 123]<>[104, 5, 63, 89] => [-104, -28, 107, 123, 104, 5, 63, 89] :: [0, 4, 2, 3, 1, 5, 6, 7] (2,5,1)
        r:Merge:[-104, -28, 5, 123]<>[104, 107, 63, 89] => [-104, -28, 5, 123, 104, 107, 63, 89] :: [0, 4, 5, 3, 1, 2, 6, 7] (3,6,1)
        r:Merge:[-104, -28, 5, 63]<>[104, 107, 123, 89] => [-104, -28, 5, 63, 104, 107, 123, 89] :: [0, 4, 5, 6, 1, 2, 3, 7] (4,7,1)
        r:Merge:[-104, -28, 5, 63]<>[89, 107, 123, 104] => [-104, -28, 5, 63, 89, 107, 123, 104] :: [0, 7, 5, 6, 1, 2, 3, 4] (5,8,1)
        f:Merge:[-104, -28, 5, 63]<>[89, 104, 123, 107] => [-104, -28, 5, 63, 89, 104, 123, 107] :: [0, 5, 7, 6, 1, 2, 3, 4] (6,8,2)
        f:Merge:[-104, -28, 5, 63]<>[89, 104, 107, 123] => [-104, -28, 5, 63, 89, 104, 107, 123] :: [0, 5, 6, 7, 1, 2, 3, 4] (7,8,3)
MergeSort Mut  : (19, [-104, -28, 5, 63, 89, 104, 107, 123])
>>
Quick Sort     : [-104, -28, 5, 63, 89, 104, 107, 123]
>>
bTree Sort     : [123, 107, 104, 89, 63, 5, -28, -104]
>>
List Sort      : (19, [-104, -28, 5, 63, 89, 104, 107, 123])
```
