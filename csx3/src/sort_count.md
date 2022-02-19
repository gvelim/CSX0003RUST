# Count Sort
A sorting algorithm with **O(n) time complexity** with the following key points
* Sorts the array in 2 passes
* Does not make use of comparisons
* Keeps a Bookkeeping array for counting number of occurrences per array item.

The algorithm's limitation is that memory demand grows to the `2 ^ sizeof(type)` hence it is not recommended for arrays holding values 16bit and over.

## Challenges working with integer arrays
The algorithm requires the ability to **translate** 
* input array values (positive or negative) to BookKeeping indexes (positive)
* Index positions to negative or positive values

```
+---+---+---+---+---+---+---+---+     Type: Integer or Unsigned
| 2 | 2 | 5 | 5 | 8 | 1 | 5 | 3 |     (Min, Max) = (1,8)
+---+---+---+---+---+---+---+---+     (distance) = Min - Max + 1 = 8
  |   |   |   |    \______|__
   \__|    \__|__________/   \
      |       |               |
+---+---+---+---+---+---+---+---+    Type: Unsigned
| 1 | 2 | 2 | 3 | 0 | 0 | 0 | 1 |    BookKeeping capacity(8)
+---+---+---+---+---+---+---+---+    holding counts from [min..max]   
min(1)        ^ = idx['5']   max(8)       
```
### Distance calculation
Therefore, knowing the `distance` between `min` & `max` values are fundamental to the algorithm's logic.

However, integer values can easily cause an overflow when the `distance` between `min` and `max` exceeds the `[-127..0]` or `[0..127]` ranges
```
-127            0            +127
  |-------------|-------------|        Both Min and Max are negative
   <-- dist --->                        Safe: Dist = (max - min)

-127            0            +127
  |-------------|-------------|        Min is negative, Max is positive
                                       Unsafe: (max - min) overflows
    <-------- dist --------->                

-127            0            +127
  |-------------|-------------|        Both Min & Max are positive
                  <-- dist -->         Safe: Dist = (max - min)
```
Therefore, when `min` and `max` have opposite signs we have to covert both to `usize` before we calculate the `distance`. In all other cases, incl. unsigned types, `(max - min)` is sufficient.

The following implementation covers the above
```rust,no_run,noplayground
{{#include ../../src/sort/mod.rs:sort_count_diff}}
```
Now that we know the `distance` we can use it to translate value-to-index and index-to-value.

### Value-to-index translation
We know that 
* `Min` is at `Bookkeeping[0]` position and
* `Max` is at `BookKeeping[distance]` position
* `Min < value < Max`

Therefore, the index is found as `index = value - Min` which more or less follows the same logic as the `distance` calculation, if instead of `value` you think `max`.

Therefore, for both integer and unsigned we have...

```rust,noplayground
let idx = diff(value, min);
BookKeeping[idx] += 1;
```

### Index-to-value translation
This is the reverse effect, where we need to translate the data from the BookKeeper array onto the input array. 

For example, with `Min = 1` we have
* `BookKeeper[0]`, `value = min`
* `BookKeeper[1]`, `value = min + 1`

As a result, the translation to `value` is given by `min + index`. Recall that the `index == distance` and `distance` 
* always fits the unsigned numbers value range
* overflows the signed numbers value range as shown below

```
-127            0            +127
  |-------------|-------------|        (Min,Max) = (-120,100)
    -120 <----- dist -----> 100        distance = 220
     min                    max        value = (Min: -120 + index: 220)
                                       ^^^^^ ** OVERFLOW **
```
For `i8` the `type::MAX` value is `127` hence when we add `220` we are causing an overflow of `93` that is `220 - type::MAX`.

Therefore, the trick here is to break the `index` down to`type::MAX` + `delta` and as shown by the below implementation
```rust,noplayground
let value = if i > i8::MAX as usize {
        (min + i8::MAX).wrapping_add((i - i8::MAX as usize) as i8)
    } else {
        min + i as i8
    };
```
## Final implementation
Putting all the above together, we are getting the following implementation of the `count_sort()` method
```rust,no_run,noplayground
{{#include ../../src/sort/mod.rs:sort_count}}
```