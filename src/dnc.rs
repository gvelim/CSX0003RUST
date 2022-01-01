use std::fmt::Debug;

/// Merge subroutine
/// Join to slices while in the right Order
fn merge<T>(left: &[T], right: &[T]) -> (u32, Vec<T>)
    where T: Copy + Clone + Ord {

    use std::iter::Peekable;
    use std::cmp::Ordering;

    struct MergeIterator<I: Iterator> {
        left: Peekable<I>,
        left_count: u32,
        left_len: u32,
        right: Peekable<I>,
        right_count: u32,
        right_len: u32,
    }
    impl<I: Iterator> MergeIterator<I> {
        fn new(left: I, right: I) -> Self {
            let mut mi = MergeIterator {
                left: left.peekable(),
                left_count: 0,
                left_len: 0,
                right: right.peekable(),
                right_count: 0,
                right_len: 0,
            };
            mi.left_len = mi.left.size_hint().0 as u32;
            mi.right_len = mi.right.size_hint().0 as u32;
            print!("({},{})", mi.left_len, mi.right_len);
            mi
        }
    }
    impl<I> Iterator for MergeIterator<I>
        where I: Iterator,
              I::Item: Ord,
    {
        type Item = (u32, I::Item);

        fn next(&mut self) -> Option<Self::Item> {
            match (self.left.peek(), self.right.peek()) {
                (Some(l), Some(r)) => {
                    match l.cmp(r) {
                        Ordering::Less | Ordering::Equal=> {
                            self.left_count += 1;
                            print!("L{}",self.left_count);
                            Some((0, self.left.next().unwrap()))
                        },
                        Ordering::Greater => {
                            let inv = self.left_len-self.left_count;
                            self.right_count += 1;
                            print!("R{}",self.right_count);
                            Some( (inv, self.right.next().unwrap()) )
                        },
                    }
                },
                (Some(_), None) => {
                    self.left_count += 1;
                    print!("L{}",self.left_count);
                    Some( (0, self.left.next().unwrap()) )
                },
                (None, Some(_)) => {
                    self.right_count += 1;
                    print!("R{}",self.right_count);
                    Some( (0,self.right.next().unwrap()) )
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
        let v6 = vec![8,7,6,5,4,3,2,1];
        assert_eq!(merge_sort(&v1), (3, vec![1,2,3]));
        assert_eq!(merge_sort(&v2), (4, vec![1,2,3,4]));
        assert_eq!(merge_sort(&v3), (6, vec![1,2,4,8]));
        assert_eq!(merge_sort(&v4), (10, vec![1,2,3,4,5,6]));
        assert_eq!(merge_sort(&v5), (21, vec![1,2,3,4,5,6,7]));
        assert_eq!(merge_sort(&v6), (28, vec![1,2,3,4,5,6,7,8]));
    }
    #[test]
    fn test_merge() {
        let s1 = &[2, 4, 6];
        let s2 = &[1, 3, 5];
        assert_eq!(merge(s1, s2), (6,vec![1, 2, 3, 4, 5, 6]));
        assert_eq!(merge(s2, s1), (3,vec![1, 2, 3, 4, 5, 6]));
    }
}
