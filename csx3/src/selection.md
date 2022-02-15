# Selection Algorithm
if you had an array entry of 4 elements, containing the numbers 10, 8, 2 and 4, and you were looking for the 3rd statistic that would be 8.

The first order statistic is just the minimum element of the array. That's easier to find with a linear scan. The nth order statistic is just the maximum, again easier, easy to find with a linear scan. The middle element is the median.

For all other cases, the selection algorithm returns the answers in O(n) time

## Implementation flavours
### Randomised approach
Where pivot selected is chosen randomly based on the 75/25 rule

### Deterministic approach
Where the pivot selected is always the median of the set
