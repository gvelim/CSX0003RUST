# Linked List
Basic linked list implementation to understand rust ownership & borrowing particularities

### Examples
Push and Pop
```rust
let mut l = List::new();

l.push_first(1);
l.push_first(2);
l.push_last(3);
l.push_last(4);
// [2,1,3,4]

assert_eq!(l.pop_last(), Some(4));
assert_eq!(l.pop_last(), Some(3));
assert_eq!(l.pop_first(), Some(2));
assert_eq!(l.pop_first(), Some(1));
assert_eq!(l.pop_first(), None);
```
Collect into Lists for iterating over
```rust
let v = vec![1,2,3];

let mut l : List<i32> = v.into_iter().collect();

assert_eq!(l.pop_last(), Some(1));
assert_eq!(l.pop_last(), Some(2));
assert_eq!(l.pop_last(), Some(3));
assert_eq!(l.pop_last(), None);
```