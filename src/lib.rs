extern crate core;

use rand::distributions::Standard;
use rand::prelude::Distribution;

/// Tree algorithms
pub mod trees;
/// Divide and Conquere algorithms
pub mod sort;
/// Selection algorithms
pub mod select;
pub mod linkedlists;
/// utility objects used across libraries like 'VirtualSlice'
pub mod merge;
/// Graph algorithms for Path search, strongly connected components,
pub mod graphs;
/// Greedy algorithms for scheduling, mimium spanning trees
pub mod greedy;
/// Greedy algorithms for scheduling, mimium spanning trees
pub mod hash_heap;
/// Dynamic Programming algorithms
pub mod dp;

/// Generates a list of random values based on the assigned variable type. The assigned variable must implement the `FromIterator` trait
///```
/// use csx3::random_sequence;
/// use csx3::linkedlists::List;
///
/// type MyType = u8;
/// let list : List<MyType> = random_sequence(5);
///```
pub fn random_sequence<T, B>(n: usize) -> B
    where B: FromIterator<T>,
          Standard: Distribution<T>
{
    use std::iter::from_fn;

    from_fn(|| { Some(rand::random::<T>()) })
        .take(n)
        .collect::<B>()
}