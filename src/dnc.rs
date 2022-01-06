use std::fmt::Debug;
use std::iter::Peekable;
use std::cmp::Ordering;
use std::ptr::{slice_from_raw_parts, slice_from_raw_parts_mut};


/// Takes two iterators as input with each iteration returning
/// the next in order item out of the two, plus its inversions' count
/// ```
/// let s1 = &[2, 4, 6];
/// let s2 = &[1, 3, 5];
///
/// let mut iter = MergeIterator::new(s1.iter(), s2.iter());
///
/// assert_eq!(iter.next(), Some( (3,&1) ));
/// assert_eq!(iter.next(), Some( (0,&2) ));
/// assert_eq!(iter.next(), Some( (2,&3) ));
/// assert_eq!(iter.next(), Some( (0,&4) ));
/// assert_eq!(iter.next(), Some( (1,&5) ));
/// assert_eq!(iter.next(), Some( (0,&6) ));
/// assert_eq!(iter.next(), None);
/// ```
struct MergeIterator<I: Iterator> {
    right: Peekable<I>,
    left: Peekable<I>,
    left_count: u32,
    left_len: u32,
}
impl<I: Iterator> MergeIterator<I> {
    /// Constructs a new MergeIterator given two iterators
    fn new(left: I, right: I) -> Self {
        let mut mi = MergeIterator {
            right: right.peekable(),
            left: left.peekable(),
            left_count: 0,
            left_len: 0,
        };
        mi.left_len = mi.left.size_hint().0 as u32;
        mi
    }
}
impl<I> Iterator for MergeIterator<I>
    where I: Iterator,
          I::Item: Ord, {
    // tuple returned = (number of inversions at position, value at position)
    type Item = (u32, I::Item);

    /// Outputs the next in order value out of the two iterators
    /// in the form of Some( tuple ), where
    /// tuple = ( inversions at position, value at position)
    fn next(&mut self) -> Option<Self::Item> {
        match (self.left.peek(), self.right.peek()) {
            // left & right parts remain within their bounds
            (Some(l), Some(r)) => {
                match l.cmp(r) {
                    // left is smaller hence move to output
                    // there are no inversions to count
                    // keep count of current position
                    Ordering::Less | Ordering::Equal=> {
                        self.left_count += 1;
                        Some((0, self.left.next().unwrap()))
                    },
                    // right is smaller hence move to output
                    // inversions are equal to left items remain to iterate over
                    Ordering::Greater => {
                        let inv = self.left_len-self.left_count;
                        Some( (inv, self.right.next().unwrap()) )
                    },
                }
            },
            // right part out of bounds, hence move left item to output
            (Some(_), None) => {
                Some( (0, self.left.next().unwrap()) )
            },
            // left part out of bounds, hence move right item to output
            (None, Some(_)) => {
                Some( (0,self.right.next().unwrap()) )
            },
            // both left & right parts out of bounds
            (None, None) => None,
        }
    }
}

/// Sort function based on the merge sort algorithm
/// returning a sorted vector plus the count of inversions
pub fn merge_sort<T>(v: &[T]) -> (u32, Vec<T>)
    where T: Copy + Clone + Ord + Debug {

    let len = v.len();

    println!("\tInput: ({}){:?} =>", len, v);
    match len {
        // unity slice, just return it
        0..=1 => (0, v.to_vec()),
        // sort the binary slice and exit
        // use a local variable to eliminate the need for &mut as input
        // and given we output a new vector
        2 => {
            let mut sorted_vec = Vec::from(v);
            let mut inv = 0;
            if sorted_vec[0] > sorted_vec[1] {
                sorted_vec.swap(0, 1);
                inv += 1;
            }
            (inv, sorted_vec)
        },
        // if slice length longer than 2 then split recursively
        _ => {
            let (left, right) = v.split_at(len >> 1);
            let (left_inv, left) = merge_sort(left);
            let (right_inv, right) = merge_sort(right);

            // return a vector of the merged but ordered slices
            // plus inversions vector; inversion count per position
            let (merge_vec, sorted_vec): (Vec<u32>, Vec<T>) = MergeIterator::new(left.iter(),right.iter()).unzip();

            println!("\tInversion Vector: {:?}", &merge_vec);

            // sum up the inversion count vector
            let merge_inv: u32 = merge_vec.into_iter().filter(|x| *x > 0).sum();

            println!("\tMerge: {}:{:?} <> {}:{:?} => {}:{:?}", left_inv, left, right_inv, right, left_inv + right_inv + merge_inv, sorted_vec);
            (left_inv + right_inv + merge_inv, sorted_vec)
        }
    }
}

fn partition_at_index<T>(v: &mut [T], idx: usize) -> (&mut [T], &T, &mut [T])
    where T: Copy + Clone + Ord + Debug  {

    use std::ptr::{slice_from_raw_parts};

    let mut i = 0usize;

    println!("\tInput: {:?}::{}",v, idx);

    // swap v[idx] to v[0] before entering the for loop
    v.swap(0, idx);

    // the for_each will own the &mut v anything we need within the loop
    // we'll have to get it before we get in
    let pivot = v[0];
    let ptr = v.as_mut_ptr();

    // v[0] holds the pivot point hence we start comparing from 2nd item v[1]
    // j : points to last element checked
    // i : position in array so that v[1..i] < v[i] < r[i+1..j]
    v.into_iter()
        .enumerate()
        .skip(1)
        .for_each( |(j, val)| {
            if pivot > *val {
                i+=1;
                // would be nice to make a call to v.swap(i, j) but &mut v is now owned by for_each
                // so we cannot use it in the loop as this increases its borrow counter hence we need another way
                // We extract a ptr before entering the loop to use for swapping the item
                // .. and unless we find a better way that doesn't need unsafe
                unsafe {
                    std::ptr::swap::<T>(
                        ptr.offset(i as isize),
                        ptr.offset(j as isize)
                    );
                }
                print!("\ts:");
            } else {
                print!("\t-:");
            }
            //
            println!("{:?},({},{})", unsafe{ &*slice_from_raw_parts(ptr, j+1) }, i, j);
        });
    // we found the correct order for pivot
    // hence swap v[i] with v[0]
    v.swap(0,i);
    println!("\tf:{:?}, ({})", v, i+1);

    let (l, r) = v.split_at_mut(i);
    let (m, r) = r.split_at_mut(1);

    (&mut l[..], &m[0], &mut r[..])

    //// since we already hold a mutable reference to 'v'
    //// we will violate rust's policy if we try to split it mutably
    //// since we know the partitions don't overlap we can resort to this
    // use std::ptr::{slice_from_raw_parts_mut};
    // unsafe {
    //     // since slice { ptr, len } the below
    //     // will return a mut pointer to the actual data
    //     let ptr = v.as_mut_ptr();
    //     (
    //         // return a slice pointer containing up to 0...i elements
    //         &mut *slice_from_raw_parts_mut(ptr, i),
    //         // return the partitioning position against the input array
    //         &v[i],
    //         // // return a slice pointer containing i+1 ... v.len() elements
    //         &mut *slice_from_raw_parts_mut(ptr.offset((i + 1) as isize), v.len() - i - 1),
    //     )
    // }
}

pub fn quick_sort<T>(v: &mut [T])
    where T: Copy + Clone + Ord + Debug {

    if v.len() < 2 {
        return;
    }
    // pick always an index in the middle+1 just for simplicity
    // partition the array into to mutable slices for further sorting
    let (left_partition,_ , right_partition) = partition_at_index(v, v.len() >> 1);

    quick_sort(left_partition);
    quick_sort(right_partition);
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_quick_sort() {
        let test_data: [(&mut [u32], &[u32]);6] = [
            (&mut [3,2,1],              &[1,2,3]),
            (&mut [4,1,3,2],            &[1,2,3,4]),
            (&mut [8, 4, 2, 1],         &[1,2,4,8]),
            (&mut [6,2,4,3,5,1],        &[1,2,3,4,5,6]),
            (&mut [7,6,5,4,3,2,1],      &[1,2,3,4,5,6,7]),
            (&mut [8,7,6,5,4,3,2,1],    &[1,2,3,4,5,6,7,8])
        ];

        for (input, output) in test_data {
            quick_sort(input);
            assert_eq!(input, output);
        }
    }
    #[test]
    fn test_partition_at_index() {
        let mut v = vec![6,12,5,9,7,8,11,3,1,4,2,10];
        let (l, idx, r) = partition_at_index(&mut v[..], 4);

        // [2, 5, 6, 3, 1, 4],7,[9, 12, 8, 11, 10]
        // idx = 6 (7th position)
        assert_eq!(l, &[2,5,6,3,1,4]);
        assert_eq!(idx, &7);
        assert_eq!(r, &[9,12,8,11,10]);
    }
    #[test]
    fn test_merge_sort() {
        let test_data: [(&[u32], (u32, &[u32]));6] = [
            (&[3,2,1],              (3, &[1,2,3])),
            (&[4,1,3,2],            (4, &[1,2,3,4])),
            (&[8, 4, 2, 1],         (6, &[1,2,4,8])),
            (&[6,2,4,3,5,1],        (10,&[1,2,3,4,5,6])),
            (&[7,6,5,4,3,2,1],      (21,&[1,2,3,4,5,6,7])),
            (&[8,7,6,5,4,3,2,1],    (28,&[1,2,3,4,5,6,7,8]))
        ];

        for (input,(inv_count, output)) in test_data {
            assert_eq!(
                merge_sort(&input.to_vec()),
                (inv_count, output.to_vec())
            );
        }
    }
    #[test]
    fn test_merge() {
        let s1 = &[2, 4, 6];
        let s2 = &[1, 3, 5];

        let mut iter = MergeIterator::new(s1.iter(), s2.iter());

        assert_eq!(iter.next(), Some( (3,&1) ));
        assert_eq!(iter.next(), Some( (0,&2) ));
        assert_eq!(iter.next(), Some( (2,&3) ));
        assert_eq!(iter.next(), Some( (0,&4) ));
        assert_eq!(iter.next(), Some( (1,&5) ));
        assert_eq!(iter.next(), Some( (0,&6) ));
        assert_eq!(iter.next(), None);
    }
}
