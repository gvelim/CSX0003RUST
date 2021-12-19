

/// Divide and Conquere algorithms
pub mod divnconq {

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
                merge(&left, &right)
            }
        }
    }

    use std::iter::Peekable;
    use std::cmp::Ordering;

    struct MergeIterator<L,R>
        where L: Iterator<Item=i32>,
              R: Iterator<Item=i32>
    {
        left: Peekable<L>,
        right: Peekable<R>,
    }


    impl<L, R> MergeIterator<L, R>
        where L: Iterator<Item=i32>,
              R: Iterator<Item=i32>
    {
        fn new(left: L, right: R) -> MergeIterator<L,R> {
            MergeIterator {
                left: left.peekable(),
                right: right.peekable(),
            }
        }
    }

    impl<L, R> Iterator for MergeIterator<L, R>
        where L: Iterator<Item=i32>, L::Item: Ord,
              R: Iterator<Item=i32>, R::Item: Ord,
    {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            let which: Option<Ordering> = match (self.left.peek(), self.right.peek()) {
                (Some(l), Some(r)) => Some(l.cmp(r)),
                (Some(l), None) => Some(Ordering::Less),
                (None, Some(r)) => Some(Ordering::Greater),
                (None, None) => None,
            };

            match which {
                Some(Ordering::Equal) => self.left.next(),
                Some(Ordering::Less) => self.left.next(),
                Some(Ordering::Greater) => self.right.next(),
                None => None,
            }
        }
    }


    /// Merge subroutine
    /// Join to slices while in the right Order
    fn merge(left: &[i32], right: &[i32]) -> Vec<i32> {
        enum Condition {
            LeftExit,
            RightExit,
        }

        let (l_len,r_len,mut r,mut l) = (left.len() - 1, right.len() - 1, 0, 0);
        let mut output: Vec<i32> = Vec::with_capacity(l_len + r_len + 2);

        // go into a loop until one of the slices goes off bounds
        // indicate which slice caused the exit for use by the
        // following append instruction
        match loop {
            if right[r] > left[l] {
                output.push(left[l]);
                l += 1;
                if l > l_len {
                    break Condition::LeftExit;
                }
            } else {
                output.push(right[r]);
                r += 1;
                if r > r_len {
                    break Condition::RightExit;
                }
            }
        } {
            // append the remaining slice on the output vector
            // based on the loop exit condition
            Condition::LeftExit => output.extend_from_slice(&right[r..]),
            Condition::RightExit => output.extend_from_slice(&left[l..]),
        }

        print!("merge: {},{:?} <> {},{:?},", r_len, right, l_len, left);
        println!("=> {:?},", output);
        output
    }

}