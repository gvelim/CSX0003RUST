use std::fmt::Debug;

/// Merge subroutine
/// Join to slices while in the right Order
fn merge<T>(left: &[T], right: &[T]) -> (u32, Vec<T>)
    where T: Copy + Clone + Ord {

    use std::iter::Peekable;
    use std::cmp::Ordering;

    struct MergeIterator<I: Iterator> {
        left: Peekable<I>,
        leftcount: u32,
        right: Peekable<I>,
        rightcount: u32,
    }
    impl<I: Iterator> MergeIterator<I> {
        fn new(left: I, right: I) -> Self {
            MergeIterator {
                left: left.peekable(),
                leftcount: 0,
                right: right.peekable(),
                rightcount: 0,
            }
        }
    }
    impl<I> Iterator for MergeIterator<I>
        where I: Iterator, I::Item: Ord, {
        type Item = (u32, I::Item);

        fn next(&mut self) -> Option<Self::Item> {
            match (self.left.peek(), self.right.peek()) {
                (Some(l), Some(r)) => {
                    match l.cmp(r) {
                        Ordering::Less | Ordering::Equal=> {
                            self.leftcount += 1;
                            print!("L{}",self.leftcount);
                            Some((0, self.left.next().unwrap()))
                        },
                        Ordering::Greater => {
                            self.rightcount += 1;
                            print!("R{}",self.rightcount);
                            Some((1, self.right.next().unwrap()))
                        },
                    }
                },
                (Some(_), None) => {
                    print!("L{}",self.leftcount);
                    match (self.left.next(), self.left.peek()) {
                        (Some(val), Some(_)) => Some((self.rightcount,val)),
                        (Some(val), None) => Some((0,val)),
                        (None, _) => None,
                    }
                },
                (None, Some(_)) => {
                    print!("R{}",self.rightcount);
                    match (self.right.next(), self.right.peek()) {
                        (Some(val), Some(_)) => Some((self.leftcount,val)),
                        (Some(val), None) => Some((0,val)),
                        (None, _) => None,
                    }
                },
                (None, None) => None,
            }
        }
    }

    let (inv, vec): (Vec<u32>, Vec<T>) = MergeIterator::new(left.iter(),right.iter()).unzip();

    print!("Inv: {:?}::", inv);
    (inv.into_iter().sum(), vec)
}

/// Sort function based on the merge sort algorithm
pub fn merge_sort<T>(v: &[T]) -> (u32, Vec<T>)
    where T: Copy + Clone + Ord + Debug {

    let len = v.len();

    println!("Input: ({}){:?} =>", len, v);
    match len {
        // unity slice, just return it
        0..=1 => (0, v.to_vec()),
        // sort the binary slice and exit
        // use a local variable to eliminate the need for &mut as input
        // and given we output a new vector
        2 => {
            let mut out = Vec::from(v);
            let mut inv = 0;
            if out[0] > out[1] {
                out.swap(0, 1);
                inv += 1;
            }
            (inv, out)
        },
        // if slice length longer than 2 then split recursively
        _ => {
            let (left, right) = v.split_at(len >> 1);
            let (linv, left) = merge_sort(left);
            let (rinv, right) = merge_sort(right);

            // return a vector of the merged but ordered slices
            let (minv, out) = merge(&left, &right);
            println!("Merged: {}:{:?} <> {}:{:?} => {}:{:?}", linv, left, rinv, right, linv+rinv+minv, out);
            (linv + rinv + minv, out)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_sort() {
        let v1 = vec![3,2,1];
        let v2 = vec![4,1,3,2];
        let v3 = vec![8, 4, 2, 1];
        let v4 = vec![6,2,4,3,5,1];
        let v5 = vec![7,6,5,4,3,2,1];
        let v6 = vec![8,7, 6,5,4,3,2,1];
        assert_eq!(merge_sort(&v1), (3, vec![1,2,3]));
        assert_eq!(merge_sort(&v2), (4, vec![1,2,3,4]));
        assert_eq!(merge_sort(&v3), (6, vec![1,2,4,8]));
        assert_eq!(merge_sort(&v4), (7, vec![1,2,3,4,5,6]));
        assert_eq!(merge_sort(&v5), (21, vec![1,2,3,4,5,6,7]));
        assert_eq!(merge_sort(&v6), (28, vec![1,2,3,4,5,6,7,8]));
    }
    #[test]
    fn test_merge() {
        let s1 = &[2, 4, 6];
        let s2 = &[1, 3, 5];
        assert_eq!(merge(s1, s2), (3,vec![1, 2, 3, 4, 5, 6]));
    }
}
