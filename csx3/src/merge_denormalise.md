# Pattern matching: De-normalising control flow
 
The merge logic behind the [in-place](./merge_in_place.md) requires a control flow that needs to understand the following execution paths
1. **Phase 1**: compare and swap; both comparison indexes remain within bounds
2. **Phase 2**: swap remaining forward
   1. left comparison index out of bounds
   2. right comparison index out of bounds

Remember that we make use of a `pivot` point where 
* left of `pivot` everything is ordered
* right of `pivot`, are the items remaining to either be compared or carried over

## The challenge 
A common approach will be ... 
```rust
// Phase 1 : Exit Conditions
// right_index == total_length => right part has been exhausted
// pivot == left_index => everything in array[...pivot] << array[right index...], no more comparisons needed

while right_index < total_length && pivot != right_index {

    if ( array[left_index] <= array[right_index] ) {

            // A: swap left item with partition pivot position
            // point to the next in order left position
            left_index += 1;
            
        else {
        
            // B: swap right item with partition pivot position
            // point to the next in order item (right slice)
            right_index += 1;
        }
    }
    // Move partition by one
    pivot += 1;
}

// Phase 2 : Exit Conditions
// left_index == left_array_length => copied/swapped over the left side
// pivot == total_length => 

while left_index < left_array_length-1 && pivot < total_length-1 {

    // C: swap left item with partition pivot position
    // point to the next in order left position
    // Move partition by one
}
```
From the above we observe that `Phase 1:B` and `Phase 2:C` are more or less the same logic. Code that is repeated across multiple execution paths is normally cause for human error especially when someone isn't sufficiently familiar with the logic behind.

Hence, we need a way to eliminate such code for the benefit of **maintainability**.

## Rust pattern matching
We can unroll the execution flow in the following table

```
Conditions Definition
=====================
A: (right_index < total_length && pivot != right_index ) 
   => Any more comparisons required ? have we run out of elements to compare ?

B: (left_index < left_array_length-1 && pivot < total_length-1 )
   => Have all left slice elements been processed ? Have we reached the end where i == [c] ?
   
  +------+-------+----------+------------------------------------------------
  |   A  |   B   | if Guard | Action
  +------+-------+----------+------------------------------------------------
1 | true |  true |   l > r  | Phase 1: swap right with pivot
2 | true | false |    N/A   | Exit: Merge completed; finished left part, right part remaining is ordered
3 | true |  true |    N/A   | Phase 1: l<=r implied; swap left with pivot
4 |false |  true |    N/A   | Phase 2: move remaining items; swap with pivot
5 |false | false |    N/A   | Exit: Merge completed; we have reached the end
  +------+-------+----------+------------------------------------------------
```
This resembles a state-machine pattern which helps us understand
1. condition priority/order, i.e. exit condition is last
2. all execution paths and matching logic
3. path compression, i.e. Phase 1 & 2 for left copies/swaps

As a result we make the following observations
* Paths (1) & (3) only differ by the `Guard` condition
* Paths (3) & (4) only differ by condition `A` while the `Guard` condition is not relevant
* Paths (2) & (5) only differ by condition `A`

So we can re-prioritise the table's matching order and hence we can further simplify in the following way
```gitignore
  +------+-------+----------+------------------------------------------------
  |   A  |   B   | if Guard | Action
  +------+-------+----------+------------------------------------------------
1 | true |   _   |   l > r  | Phase 1: swap right with pivot
  +------+-------+----------+------------------------------------------------
3 |  _   |  true |    N/A   | Phase 1: l<=r implied; swap left with pivot
4 |  _   |  true |    N/A   | Phase 2: move remaining items; swap with pivot
  +------+-------+----------+------------------------------------------------
2 |  _   |   _   |    N/A   | Exit: Merge completed; finished all left part, right remaining is ordered
5 |  _   |   _   |    N/A   | Exit: Merge completed; we have reached the end
  +------+-------+----------+------------------------------------------------
```

With `match` offering a powerful matching expression mechanism we can use it to write the above table in the following way

```rust
loop {
    let a = right_index < total_length && pivot != right_index;
    let b = left_index < left_array_length-1 && pivot < total_length-1

    match (a, b) {
        (true, _) if array[left_index] > array[right_index] => {
            
            // Phase 1: swap right with pivot
        }  
        (_, true) => {
        
            // Phase 1: l<=r implied; swap left with pivot
            // Phase 2: move remaining items; swap with pivot
     
        }
        (_, _) => break; // Exit: Merge completed
    }
}
```
As a result of this analysis 
* all execution paths have been understood
* we have eliminated duplication of logic across the paths
* we have been documented the logic in an easily to understand way



