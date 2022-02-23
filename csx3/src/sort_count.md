# Count Sort
A sorting algorithm with **O(n) time complexity** with the following key points
* Sorts the array in 2 passes
* Does not make use of comparisons
* Keeps a Bookkeeping array for counting number of occurrences per array item.

The algorithm's limitation is that in its worst case it could consume `2 ^ bits` of memory especially when 
* 32 & 64 bit types are used,
* with arrays where the distance between the array's MIN and MAX values is significantly large
## Challenges working with integer arrays
The algorithm requires the ability to **translate** 
* Array values (positive or negative) to BookKeeping indexes (positive)
* BookKeeping Index positions (positive) to negative or positive values
```
+---+---+---+---+---+---+---+---+     Type: Integer or Unsigned
| 2 | 2 | 5 | 5 | 8 | 1 | 5 | 3 |     (Min, Max) = (1,8)
+---+---+---+---+---+---+---+---+     (distance) = Min - Max + 1 = 8
  |   |   |   |    \______|__
   \__|    \___\_________/   \
      |           |           |
+---+---+---+---+---+---+---+---+    Type: Unsigned
| 1 | 2 | 1 | 0 | 3 | 0 | 0 | 1 |    BookKeeping (BK) capacity(8)
+---+---+---+---+---+---+---+---+    holding counts from BK[min..max]   
min(1)  BK['5'] = ^         max(8)       
```
### Distance calculation
Therefore, knowing the `distance` between `min` & `max` values is fundamental to the algorithm's logic.

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
Therefore, when `min` and `max` have opposite signs we have to covert both to `usize` before we calculate the `distance`.
In all other cases, incl. unsigned types, `(max - min)` is sufficient.
```rust,noplayground
fn dist(max: i8, min: i8) -> usize {
    match (min < 0, max < 0) {
        (true, false) => max as usize + min.unsigned_abs() as usize,
        (_, _) => (max - min) as usize,
    }
}
```
However, since `unsigned` types do not support the `unsigned_abs()` method we'll have to abstract the `distance` function onto a `trait` and use `macro` implementations for the all numerical primitives. Using generics here would add significant complexity.

```rust,no_run,noplayground
{{#include ../../src/sort/mod.rs:sort_count_diff}}
```
Now that we know how to calculate the `distance` we can proceed with **value-to-index** and **index-to-value** translations.

### Value-to-index translation
We know that 
* `Min` maps to `Bookkeeping[0]` position and
* `Max` maps to `BookKeeping[distance]` position
* where `Min <= array[..] <= Max`

Therefore, the index is found as `index = value - Min` which more or less is the `distance` from `min`, which we already know how to calculate.
As a result and for the `i8` type, we get the following implementation ...

```rust,noplayground
let idx = i8::dist(value, min);     // Map array value -> BK index 
BookKeeping[idx] += 1;              // increment count by 1
```

### Index-to-value translation
This is the reverse effect, where we need to translate the `index` from the BookKeeping onto the corresponding array `value`, since we know that BookKeeping position `[0]` is the `min` value wihtin the input array.

For example, if `min == 6` then the array's `value` at position `index` will be given as
* for `index = 0`, `array[0] = MIN + 0`
* for `index = 1`, `array[1] = MIN + 1`
* for `index = 2`, `array[2] = MIN + 2`
* etc

Recall that the `max(index) == distance` and `distance` 
* always **_fits_** the unsigned numbers value range
* **_overflows_** the signed numbers value range as shown below
```
-127            0            +127
  |-------------|-------------|        (Min,Max) = (-123,122)
    -123 <----- dist -----> 122        distance = 245
     min                    max        value = (Min: -123 + index: 245)
                                       ^^^^^ ** OVERFLOW **
```
For example, `i8` has `i8::MIN` value of `-128` plus and `index` with value `245` will cause an overflow of `-11`; this is equivalent to `245 % i8::MIN`.
However, the trick here is that by adding `-11` to `min` and wrapping around, will yield the desired `value`.

Therefore, the steps  to translate `index/unsigned` to `value/signed` are
1. Convert `index` to `i8` given by `index % i8:MIN`
2. and do a **modular add** with `min`
```
Value = (Min  + (   index as i8  )) % 128                
=====   =====   ===================   ===
82    = (-123 + (-51 = 205 % -128)) % 128
113   = (-123 + (-20 = 236 % -128)) % 128
122   = (-123 + (-11 = 245 % -128)) % 128
```
Rust performs then above operation with the following statement and implemented as`Distance::add_index()`
```rust,noplayground
array[index] = min.wrapping_add( index as i8 );
```
## Final implementation
Hence, by putting all the above together, we have the following implementation for the `count_sort()` method
```rust,no_run,noplayground
{{#include ../../src/sort/mod.rs:sort_count}}
```
