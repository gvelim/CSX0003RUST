
# Selection Algorithms
Find the nth statistic order item within an array at O(n).
```rust
    use csx3::select::Select;
    
    let (arr, nth_order) = (&mut [23,43,8,22,15,11], 1usize);
    
    let ret_val = arr.r_selection(nth_order);
    
    assert_eq!(ret_val, &8);
    assert_eq!(&arr[nth_order-1], &8);
```

For more detail visit here: https://gvelim.github.io/CSX0003RUST/selection.html
