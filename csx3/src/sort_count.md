# Count Sort
A near liner sorting algorithm with **O(n) time complexity**, however with use limitations due to memory demands when larger than 16bit numbers are in use

The algorithm does not make comparisons, rather it uses a Bookkeeping array to count the occurences per array element. 

Therefore, this implementation 
* Might take lots of memory even if the input array is [0x00, 0xFFFF_FFFF]
* Is better suited to unsigned numerical types up to 16bit size.

```rust,no_run,noplayground
{{#include ../../src/sort/mod.rs:sort_count}}
```