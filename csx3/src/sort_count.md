# Count Sort
A near liner sorting algorithm with **O(n) time complexity**, however with usage limitations due to memory demands when used numbers larger than 16bit.

Key points
* Sorts the array in 2 passes
* Does not make use of comparisons
* Keeps a Bookkeeping array for counting number of occurences per array item.

## Challenges working with signed numbers
The algorithm requires the ability to **translate** 
* input array values (positive or negative) to BookKeeping indexes (positive)
* Index positions to negative or positive values

Both cases require knowledge of the `distance` between `min` & `max` values found in the input array
```
  0                          255
  |---------------------------|        Unsigned values to index conversion
                                       Distance calculation without overflow
  min <----- distance ----> max
  
-127            0            +127
  |-------------|-------------|        Integer values to index comversion
                                       Causes Overflow when distance > 127 
```
### Translating integer/unsigned values to index
For unsigned values the translation is straight forward and can be easily casted
```rust,noplayground
let idx = value as usize;
```
However, dealing with signed values we can easily cause an overflow and panic since the `distance` between `min` and `max` can exceed the `[-127..0]` or `[0..127]` ranges
```
-127            0            +127
  |-------------|-------------|        Both Min and Max are negative
   <-- len --->                        

-127            0            +127
  |-------------|-------------|        Min is negative, Max is positive
           <-- len --->                

-127            0            +127
  |-------------|-------------|        Both Min & Max are positive
                  <-- len --->         
```
Therefore, we conclude that
* when either both are positive or negative the distance falls within either the `[-127..0]` or `[0..127]` ranges, which is safe
* otherwise, we are at risk of an overflow, and therefore we have to covert both to `usize` before we calculate the `distance`

The following implementation covers all cases incl. those with unsigned numbers
```rust,no_run,noplayground
{{#include ../../src/sort/mod.rs:sort_count_diff}}
```

### Translating the index back to integer/unsigned values
Here we have to cater for the case where the `index > 127` as such condition will cause an overflow.

The following implementation, ensures under such scenario we
* add `127` to avoid the overflow condition, then
* add the remainder of distance given by `index - 127` 
```rust,noplayground
let val = if i > i8::MAX as usize {
        (min + i8::MAX).wrapping_add((i - i8::MAX as usize) as i8)
    } else {
        min + i as i8
    };
```
## Implementation
Therefore, this implementation 
* Might take lots of memory even if the input array is [0x00, 0xFFFF_FFFF]
* Is better suited to unsigned numerical types up to 16bit size.

```rust,no_run,noplayground
{{#include ../../src/sort/mod.rs:sort_count}}
```