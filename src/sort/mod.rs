use std::fmt::Debug;
use std::iter::Peekable;
use std::cmp::Ordering;
use rand::Rng;
use crate::merge::VirtualSlice;

/// Takes two iterators as input with each iteration returning
/// the next in order item out of the two, plus its inversions' count
/// ```
/// use csx3::sort::*;
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
pub struct MergeIterator<I: Iterator> {
    right: Peekable<I>,
    left: Peekable<I>,
    left_count: usize,
    left_len: usize,
}
impl<I: Iterator> MergeIterator<I> {
    /// Constructs a new MergeIterator given two iterators
    pub fn new(left: I, right: I) -> Self {
        let mut mi = MergeIterator {
            right: right.peekable(),
            left: left.peekable(),
            left_count: 0,
            left_len: 0,
        };
        mi.left_len = mi.left.size_hint().0;
        mi
    }
}
impl<I> Iterator for MergeIterator<I>
    where I: Iterator,
          I::Item: Ord,
{
    // tuple returned = (number of inversions at position, value at position)
    type Item = (usize, I::Item);

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
                        let inv = self.left_len - self.left_count;
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

/// Applies memory efficient in-place merging when two slices are adjacent to each other.
/// ```
/// use csx3::sort::merge_mut_adjacent;
///
/// let mut input = vec![1, 3, 5, 7, 9, 2, 4, 6, 8, 10];
/// let (s1,s2) = input.split_at_mut(5);
///
/// merge_mut_adjacent(s1,s2);
/// assert_eq!(input, vec![1,2,3,4,5,6,7,8,9,10]);
/// ```
/// Panics in case the two slices are found not to be adjacent. For safety, always use *ONLY* against slices that have been mutable split from an existing slice
/// #[should_panic]
/// let s1 = &mut [3, 5, 7];
/// let s2 = &mut [1, 3, 5];   // wedge this between the two
/// let s3 = &mut [2, 4, 6];
///
/// merge_mut_adjacent(s1,s3); // this should throw a panic
///
/// There is no warranty that Rust will maintain two slice adjacent in a case like this.
/// let s1 = &mut [3, 5, 7];
/// let s3 = &mut [2, 4, 6];
///
/// merge_mut_adjacent(s1,s3); // this may not always work
///
pub fn merge_mut_adjacent<T>(s1: &mut[T], s2:&mut[T]) -> usize
    where T: Ord + Debug
{
    // println!("\tInput: {:?},{:?}", s1, s2);

    let mut ws = VirtualSlice::new_adjacent(s1);
    ws.merge(s2)
}

/// Merge two non-adjacent slices using in-place memory swaps and without use of rotations
/// ```
/// use csx3::sort::merge_mut;
///
/// let s1 = &mut [5,6,7];
/// let _s = &[0,0,0,0,0,0]; // wedge to break adjacency
/// let s2 = &mut [1,2,3,4];
///
/// let inv = merge_mut(s1,s2);
///
/// assert_eq!(s1, &[1,2,3]);
/// assert_eq!(s2, &[4,5,6,7]);
/// ```
pub fn merge_mut<T>(s1: &mut[T], s2:&mut[T]) -> usize
    where T: Ord + Debug {

    //println!("Merge Input: {:?},{:?}", s1, s2);

    let mut ws = VirtualSlice::new();

    ws.merge(s1);
    ws.merge(s2)
}

/// Sort function based on the merge sort algorithm
/// Sorts the mutable vector with no additional memory by applying in-place merging
/// while it returns the total count of inversions occurred
///
/// The following functions are available to use as passing parameter
/// - merge_mut : safe to use with non-adjacent (CPU: O(n), Memory: 2O(n))
/// - merge_mut_adjacent : only to be used when slices are memory adjacent (CPU: O(n log n), Memory: 0)
///
/// ```
/// use csx3::sort::{merge_mut, mergesort_mut};
///
/// let input = &mut [8, 4, 2, 1];
///
/// assert_eq!( mergesort_mut(input, merge_mut), 6 );
/// assert_eq!( input, &[1,2,4,8] );
/// ```
pub fn mergesort_mut<T, F>(v: &mut [T], mut fn_merge: F ) -> usize
    where T: Ord + Debug,
          F: Copy + FnMut(&mut[T], &mut[T]) -> usize {

    let len = v.len();

    //println!("\tInput: ({}){:?} =>", len, v);
    match len {
        // unity slice, just return it
        0..=1 => (0),
        // sort the binary slice and exit
        // use a local variable to eliminate the need for &mut as input
        // and given we output a new vector
        2 => {
            if v[0] > v[1] {
                v.swap(0, 1);
                return 1usize
            }
            0usize
        },
        // if slice length longer than 2 then split recursively
        _ => {
            let (left, right) = v.split_at_mut(len >> 1);
            let left_inv = mergesort_mut(left, fn_merge);
            let right_inv = mergesort_mut(right, fn_merge);

            // merge the two slices taking an in-place merging approach - no additional memory
            // plus return the total inversions occured
            let merge_inv = fn_merge(left, right);

            //println!("\tMerged: {:?}{:?} => {}", left, right, left_inv + right_inv + merge_inv);
            left_inv + right_inv + merge_inv
        }
    }
}
/// Sort function based on the merge sort algorithm
/// Returns a new sorted vector given an input reference slice - heap allocations
/// along with the total count of inversions occurred
/// ```
/// use csx3::sort::mergesort;
///
/// let input = &[8, 4, 2, 1];
///
/// assert_eq!( mergesort(input), (6, vec![1,2,4,8]) );
/// ```
pub fn mergesort<T>(v: &[T]) -> (usize, Vec<T>)
    where T: Copy + Clone + Ord {

    let len = v.len();

    //println!("\tInput: ({}){:?} =>", len, v);
    match len {
        // unity slice, just return it
        0..=1 => (0, v.to_vec()),
        // sort the binary slice and exit
        // use a local variable to eliminate the need for &mut as input
        // and given we output a new vector
        2 => {
            let mut inv_count = 0usize;
            let mut output = v.to_vec();
            if v[0] > v[1] {
                output.swap(0, 1);
                inv_count += 1;
            }
            (inv_count, output)
        },
        // if slice length longer than 2 then split recursively
        _ => {
            let (left, right) = v.split_at(len >> 1);
            let (left_inv, left) = mergesort(left);
            let (right_inv, right) = mergesort(right);

            // return a vector of the merged but ordered slices
            // plus inversions vector; inversion count per position
            let (merge_vec, output ):( Vec<_>, Vec<T>) = MergeIterator::new(left.iter(),right.iter()).unzip();
            // println!("\tInversion Vector: {:?}", &merge_vec);

            // sum up the inversion count vector
            let merge_inv : usize = merge_vec.into_iter().filter(|x| *x > 0).sum();
            //println!("\tInversion Vector: {:?}", &merge_vec);

            //println!("\tMerged: {:?}{:?} => {}", left, right, left_inv + right_inv + merge_inv);
            (left_inv + right_inv + merge_inv, output)
        }
    }
}

/// Splits an array into two mutable slices/partitions around a pivot location index
/// so that *[values in left partition] < [pivot] < [values in right partition]*
/// ```
/// use csx3::sort::*;
/// let mut v = vec![6,12,5,9,7,8,11,3,1,4,2,10];
/// let (l, idx, r) = partition_at_index(&mut v, 4);
///
/// // [2, 5, 6, 3, 1, 4],7,[9, 12, 8, 11, 10]
/// // idx = &7 (6th position using zero based index)
/// assert_eq!(l, &[2,5,6,3,1,4]);
/// assert_eq!(idx, &7);
/// assert_eq!(r, &[9,12,8,11,10]);
/// ```
pub fn partition_at_index<T>(v: &mut [T], idx: usize) -> (&mut [T], &mut T, &mut [T])
    where T: Copy + Clone + Ord  {

    let len = v.len();
    assert!(idx < len);
    //println!("\tInput: {:?}, (@{}{})",v, idx+1, match idx {0=>"st",1=>"nd",2=>"rd",_=>"th"});

    let mut i = 0usize;

    // swap v[idx] to v[0] before entering the for loop
    v.swap(0, idx);

    // the for_each will own the &mut v anything we need within the loop
    // we'll have to get it before we get in
    let pivot = v[0];
    let ptr = v.as_mut_ptr();

    // v[0] holds the pivot point hence we start comparing from 2nd item v[1]
    // j : points to last element checked
    // i : position in array so that v[1..i] < v[i] < r[i+1..j]
    v.iter_mut()
        .enumerate()
        .skip(1)
        .for_each( |(j, val)| {
            if pivot > *val {
                i+=1;
                // would be nice to make a call to v.swap(i, j) but &mut v is now owned by for_each
                // so we cannot use it in the loop as this increases its borrow counter hence we need another way
                // We extract a ptr before entering the loop to use for swapping the item
                // and unless we find a better way that doesn't need unsafe neither use of while or for loops
                unsafe {
                    std::ptr::swap::<T>(
                        ptr.wrapping_add(i),
                        ptr.wrapping_add(j)
                    );
                }
                //print!("\ts:");
            }
            // else {
            //     print!("\t-:");
            // }
            //
            // println!("{:?},({},{})", unsafe{ &*slice_from_raw_parts::<T>(ptr, len) }, i+1, j+1);
        });
    // we found the correct order for pivot
    // hence swap v[i] with v[0]
    v.swap(0,i);
    //println!("\tf:{:?}, ({})", v, i+1);

    // split the array into [left part], [pivot + right partition]
    let (l, r) = v.split_at_mut(i);
    // split further into [pivot], [right partition]
    let (p, r) = r.split_at_mut(1);

    (l, &mut p[0], r)
}
/// Short a given array using the Quick Sort algorithm.
/// The function rearranges the array contents rather than returning a new sorted copy of the input array
/// ```
/// use csx3::sort::quick_sort;
///
/// let v = &mut [3,5,8,1,2,4,6,0];
///
/// quick_sort(v);
/// assert_eq!(v, &[0,1,2,3,4,5,6,8]);
/// ```
pub fn quick_sort<T>(v: &mut [T])
    where T: Copy + Clone + Ord + Debug {

    // have we reached the end of the recursion ?
    if v.len() < 2 {
        return;
    }
    // pick an index at random based on a uniform distribution
    let idx = rand::thread_rng().gen_range(0..(v.len()-1) );
    // partition the array into to mutable slices for further sorting
    let (left_partition,_ , right_partition) = partition_at_index(v, idx);

    // Recurse against left an right partitions
    quick_sort(left_partition);
    quick_sort(right_partition);
}


#[cfg(test)]
mod test {
    use crate::random_sequence;
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

        test_data.into_iter()
            .for_each( | (input, output) | {
                quick_sort(input);
                assert_eq!(input, output);
            })
    }
    #[test]
    fn test_partition_at_index() {
        let mut v = vec![6,12,5,9,7,8,11,3,1,4,2,10];
        let (l, idx, r) = partition_at_index(&mut v, 4);

        // [2, 5, 6, 3, 1, 4],7,[9, 12, 8, 11, 10]
        // idx = &7 (7th position)
        assert_eq!(l, &[2,5,6,3,1,4]);
        assert_eq!(idx, &7);
        assert_eq!(r, &[9,12,8,11,10]);
    }
    #[test]
    fn test_merge_sort_mut() {
        let test_data: [(&mut [i32], (usize, &[i32]));7] = [
            (&mut [3,2,1],              (3, &[1,2,3])),
            (&mut [4,1,3,2],            (4, &[1,2,3,4])),
            (&mut [8, 4, 2, 1],         (6, &[1,2,4,8])),
            (&mut [6,2,4,3,5,1],        (10,&[1,2,3,4,5,6])),
            (&mut [7,6,5,4,3,2,1],      (21,&[1,2,3,4,5,6,7])),
            (&mut [8,7,6,5,4,3,2,1],    (28,&[1,2,3,4,5,6,7,8])),
            (&mut [-111, -52, -38, -13, 16, 26, 73, 103], (0,&[-111,-52,-38,-13,16,26,73,103]))
        ];

        test_data.into_iter()
            .for_each(|(input,(inv_count, output))| {
                assert_eq!(mergesort_mut(input, merge_mut), inv_count );
                assert_eq!( input, output );
            })
    }
    #[test]
    fn test_merge_sort_mut_adjacent() {
        let test_data: [(&mut [u32], (usize, &[u32]));6] = [
            (&mut [3,2,1],              (3, &[1,2,3])),
            (&mut [4,1,3,2],            (4, &[1,2,3,4])),
            (&mut [8, 4, 2, 1],         (6, &[1,2,4,8])),
            (&mut [6,2,4,3,5,1],        (10,&[1,2,3,4,5,6])),
            (&mut [7,6,5,4,3,2,1],      (21,&[1,2,3,4,5,6,7])),
            (&mut [8,7,6,5,4,3,2,1],    (28,&[1,2,3,4,5,6,7,8]))
        ];

        test_data.into_iter()
            .for_each(|(input,(inv_count, output))| {
                assert_eq!(mergesort_mut(input, merge_mut_adjacent), inv_count );
                assert_eq!( input, output );
        })
    }
    #[test]
    fn test_merge() {
        let s1 = &[34, 36, 80, 127];
        let s2 = &[-36, -22, -3, 109];

        let mut iter = MergeIterator::new(s1.iter(), s2.iter());

        assert_eq!(iter.next(), Some( (4,&-36) ));
        assert_eq!(iter.next(), Some( (4,&-22) ));
        assert_eq!(iter.next(), Some( (4,&-3) ));
        assert_eq!(iter.next(), Some( (0,&34) ));
        assert_eq!(iter.next(), Some( (0,&36) ));
        assert_eq!(iter.next(), Some( (0,&80) ));
        assert_eq!(iter.next(), Some( (1,&109) ));
        assert_eq!(iter.next(), Some( (0,&127) ));
        assert_eq!(iter.next(), None);
    }
    #[test]
    fn test_merge_mut_adjacent() {
        let arr:[(&mut[i32],&[i32]);11] = [
            (&mut [34, 36, 80, 127, -36, -22, -3, 109], &[-36, -22, -3, 34, 36, 80, 109, 127]),
            (&mut [2,4,6,1,3,5], &[1,2,3,4,5,6]),
            (&mut [1,3,5,2,4,6], &[1,2,3,4,5,6]),
            (&mut [2,4,1,3,5], &[1,2,3,4,5]),
            (&mut [1,3,2,4,5], &[1,2,3,4,5]),
            (&mut [1,2,3,4,5], &[1,2,3,4,5]),
            (&mut [2,1,4], &[1,2,4]),
            (&mut [3,1,2], &[1,2,3]),
            (&mut [1,2,3], &[1,2,3]),
            (&mut [2,1], &[1,2]),
            (&mut [1,2], &[1,2]),
        ];
        arr.into_iter()
            .for_each(| (input, output) | {
                let len = input.len();
                let (s1, s2) = input.split_at_mut(len >> 1);
                merge_mut_adjacent(s1, s2);
                assert_eq!(input, output);
        })
    }
    #[test]
    #[should_panic]
    fn test_merge_mut_panic() {
        let s1 = &mut [3, 5, 7];
        let _s2 = &mut [1, 3, 5];
        let s3 = &mut [2, 4, 6];

        // non-adjacent slices hence it should panic
        merge_mut_adjacent(s1, s3);
    }
    #[test]
    fn test_merge_mut() {
        let arr:[(&mut[i32],&[i32]);13] = [
            (&mut [34, 36, 80, 127, -36, -22, -3, 109], &[-36, -22, -3, 34, 36, 80, 109, 127]),
            (&mut [2,4,6,1,3,5], &[1,2,3,4,5,6]),
            (&mut [1,3,5,2,4,6], &[1,2,3,4,5,6]),
            (&mut [5,6,7,1,2,3,4], &[1,2,3,4,5,6,7]),
            (&mut [1,2,3,4,5,6,7], &[1,2,3,4,5,6,7]),
            (&mut [2,4,1,3,5], &[1,2,3,4,5]),
            (&mut [1,3,2,4,5], &[1,2,3,4,5]),
            (&mut [1,2,3,4,5], &[1,2,3,4,5]),
            (&mut [2,1,4], &[1,2,4]),
            (&mut [3,1,2], &[1,2,3]),
            (&mut [1,2,3], &[1,2,3]),
            (&mut [2,1], &[1,2]),
            (&mut [1,2], &[1,2]),
        ];
        arr.into_iter()
            .for_each(| (input, output) | {
                let len = input.len();
                let (s1, s2) = input.split_at_mut(len >> 1);
                merge_mut(s1, s2);
                assert_eq!(input, output);
            })
    }
    #[test]
    fn test_mergesort_head_to_head()
    {
        for _ in 0..127 {
            let v1: Vec<i8> = random_sequence(512);
            let mut v2 = v1.clone();

            let inv = mergesort_mut(&mut v2, merge_mut);
            assert_eq!( mergesort(&v1), (inv, v2) );
        }
    }
}
