
/// Merge subroutine
/// Join to slices while in the right Order
fn merge(left: &[i32], right: &[i32]) -> Vec<i32> {

    use std::iter::Peekable;
    use std::cmp::Ordering;

    struct MergeIterator<I: Iterator> {
        left: Peekable<I>,
        right: Peekable<I>,
    }
    impl<I: Iterator> MergeIterator<I> {
        fn new(left: I, right: I) -> Self {
            MergeIterator {
                left: left.peekable(),
                right: right.peekable(),
            }
        }
    }
    impl<I> Iterator for MergeIterator<I>
        where I: Iterator, I::Item: Ord, {
        type Item = I::Item;

        fn next(&mut self) -> Option<Self::Item> {
            match
            match (self.left.peek(), self.right.peek()) {
                (Some(l), Some(r)) => { Some(l.cmp(r)) },
                (Some(_), None) => Some(Ordering::Less),
                (None, Some(_)) => Some(Ordering::Greater),
                (None, None) => None,
            }
            {
                Some(Ordering::Equal) => self.left.next(),
                Some(Ordering::Less) => self.left.next(),
                Some(Ordering::Greater) => self.right.next(),
                None => None,
            }
        }
    }

    MergeIterator::new(left.iter(),right.iter())
        .map(|&x| x)
        .collect()
}

/// Sort function based on the merge sort algorithm
pub fn merge_sort(v: &[i32]) -> Vec<i32> {


    let len = v.len();
    println!("Input: ({}){:?} =>", len, v);
    match len {
        // unity slice, just return it
        0..=1 => v.to_vec(),
        // sort the binary slice and exit
        // use a local variable to eliminate the need for &mut as input
        // and given we output a new vector
        2 => {
            let mut out = Vec::from(v);
            if out[0] > out[1] {
                out.swap(0, 1);
            }
            out
        },
        // if slice length longer than 2 then split recursively
        _ => {
            let (left,right) = v.split_at(len >> 1);
            let left = merge_sort(left);
            let right = merge_sort(right);

            // return a vector of the merged but ordered slices
            let out = merge(&left, &right);
            println!("Merged: {:?} <> {:?} => {:?}", left, right, out);
            out
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_sort() {
        let v = vec![9,2,8,3,7,4,6,5];
        assert_eq!(merge_sort(&v), vec![2,3,4,5,6,7,8,9]);
    }
    #[test]
    fn test_merge() {
        let s1 = &[2, 5, 7, 9];
        let s2 = &[1, 3, 6, 8];
        assert_eq!(merge(s1, s2), vec![1, 2, 3, 5, 6, 7, 8, 9]);
    }
}
