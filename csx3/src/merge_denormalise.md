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
// left_index == total_length => right part has been exhausted
// pivot == left_index => everything in array[...pivot] << array[right index...], no more comparisons needed

while left_index < total_length && pivot != left_index {

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
// left_index == array_length => copied/swapped over the left side
// pivot == array_length => 

while left_index < array_length-1 && pivot < array_length-1 {

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
A: (left_index < total_length && pivot != left_index ) 
   => Any more comparisons required ? have we run out of elements to compare ?

B: (left_index < array_length-1 && pivot < array_length-1 )
   => Have all left slice elements been processed ? Have we reached the end where i == [c] ?
   
+------+-------+----------+---------------------------------------
|   A  |   B   | if Guard | Action
+------+-------+----------+---------------------------------------
| true |  true |   l > r  | Phase 1: swap right with pivot
| true | false |   l > r  | - nothing to do here, right > left -
| true |  true |    ANY   | Phase 1: l<=r implied; swap left with pivot
|false |  true |    ANY   | Phase 2: finish remaining left items
|false | false |    N/A   | Exit: Merge completed
+------+-------+----------+---------------------------------------
```
This resembles a state-machine pattern which helps us understand
1. condition priority/order, i.e. exit condition is last
2. all execution paths and matching logic
3. path compression, i.e. Phase 1 & 2 for left copies/swaps

With `match` offering a powerful matching expression mechanism we can write the above table in the following way

```rust
loop {
    let a = left_index < total_length && pivot != left_index;
    let b = left_index < array_length-1 && pivot < array_length-1

    match (a, b) {
        (true, _) => if left[left_index] > right[right_index] {
            
            // Phase 1: swap right with pivot
        }  
        (_, true) => {
        
            // Phase 1: l<=r iPhase; swap left with pivot
            // Phase 2: swap left with pivot
     
        }
        (_, _) => break; // Exit: Merge completed
    }
}
```