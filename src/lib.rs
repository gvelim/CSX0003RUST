use rand::distributions::Standard;
use rand::prelude::Distribution;

/// Tree algorithms
pub mod trees;
/// Divide and Conquere algorithms
pub mod sort;
/// Selection algorithms
pub mod select;
pub mod linkedlists;
pub mod utils;


pub fn random_sequence<T, B>(n: usize) -> B
    where B: FromIterator<T>,
          Standard: Distribution<T>
{
    use std::iter::from_fn;

    from_fn(|| { Some(rand::random::<T>()) })
        .take(n)
        .collect::<B>()
}