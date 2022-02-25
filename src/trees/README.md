# bTree
Basic bTree implementation, playing smart pointers in relation to ownership & borrowing particularities

### Examples
Inserting and iterating over
```rust
let mut a = BinaryTree::new(41);

a.add(50);
a.add(40);
a.add(60);
a.add(45);

assert_eq!(
    a.iter()
        .map( |x| *x )
        .collect::<Vec<i32>>(),
    vec![60,50,45,41,40]
);
```
