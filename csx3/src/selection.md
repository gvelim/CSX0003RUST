# Selection Algorithm
if you had an array entry of 4 elements, containing the numbers 10, 8, 2 and 4, and you were looking for the 3rd statistic that would be 8.

The first order statistic is just the minimum element of the array. That's easier to find with a linear scan. The nth order statistic is just the maximum, again easier, easy to find with a linear scan. The middle element is the median.

For all other cases, the selection algorithm returns the answers in O(n) time

## Implementation flavours
### Randomised approach
Where pivot selected is chosen randomly based on the 75/25 rule

```rust,no_run,noplayground
{{#include ../../src/select/mod.rs:selection_r}}
```

### Deterministic approach
Where the pivot selected is always the median of the recursive set provided by the below implementation

The idea is to
* break the array into chunks of 5 elements
* sort them and pick `chunk[3]` as the median
* Collect all medians into a new `array`
* recurse until you converge to the median

```rust,no_run,noplayground
{{#include ../../src/select/mod.rs:selection_median}}
```