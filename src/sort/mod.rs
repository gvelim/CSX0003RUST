use std::fmt::Debug;
use rand::Rng;
use crate::merge::{MergeIterator, VirtualSlice};

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
// ANCHOR: sort_merge_mut
/// Sort function based on the merge sort algorithm
/// Sorts the mutable vector with in-place operations
/// while it returns the total count of inversions occurred
///
/// The following functions are available to use as passing parameter
/// - merge_mut : safe to use with non-adjacent; time: O(n+m), space: O(2n+m)*usize
/// - merge_mut_adjacent : use only when slices are adjacent in memory: time: O(n+m), space: O(n)*usize
///
/// ```
/// use csx3::sort::{merge_mut_adjacent, mergesort_mut};
///
/// let input = &mut [8, 4, 2, 1];
///
/// assert_eq!( mergesort_mut(input, merge_mut_adjacent), 6 );
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
// ANCHOR_END: sort_merge_mut
// ANCHOR: sort_merge
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
// ANCHOR_END: sort_merge

// ANCHOR: sort_quick_partition
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
                } }
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
// ANCHOR_END: sort_quick_partition

// ANCHOR: sort_quick
/// Sorts a given array using the Quick Sort algorithm.
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
// ANCHOR_END: sort_quick

// ANCHOR: sort_count
/// Sorts a given array using the Count Sort algorithm.
/// Input array NuType shouldn't exceed u16 to avoid memory issues
/// ```
/// use csx3::sort::CountSort;
///
/// let v = &mut [3i8,5,8,1,2,4,6,0];
///
/// v.count_sort();
/// assert_eq!(v, &[0,1,2,3,4,5,6,8]);
/// ```

pub trait CountSort {
    type NumType;

    fn count_sort(&mut self);
    fn min_max(&self) -> (Self::NumType, Self::NumType);
    fn diff(max:Self::NumType, min: Self::NumType) -> usize;
}

macro_rules! impl_countsort {
    ( $($x:ty),* ) => {
        $(impl CountSort for [$x] {
            type NumType = $x;

            #[inline]
            fn min_max(&self) -> (Self::NumType, Self::NumType) {
                let (mut min, mut max) = (self[0], self[0]);
                self.into_iter()
                    .skip(1)
                    .for_each(|x| {
                        if *x > max { max = *x; } else if *x < min { min = *x; }
                    });
                (min, max)
            }
            #[inline]
            // ANCHOR: sort_count_diff
            fn diff(max: Self::NumType, min: Self::NumType) -> usize {
                match (min < 0, max < 0) {
                    (true, false) => max as usize + min.unsigned_abs() as usize,
                    (_, _) => (max - min) as usize,
                }
            }
            // ANCHOR_END: sort_count_diff
            fn count_sort(&mut self) {
                // find min and max elements
                // so we can construct the boundaries of the counting array
                // i.e. if (min,max) = (13232, 13233) then we need only an array with capacity(2)
                let (min, max) = self.min_max();

                // construct a counting array with length = Max - Min + 1
                let len: usize = Self::diff(max, min);
                // initialise it with zero counts
                let mut count = vec![0usize; len + 1];
                // and finally measure counts per item
                self.into_iter()
                    .for_each(|x| {
                        // construct index offset based on Min value, such as, Min is at [0] position
                        let idx: usize = Self::diff(*x, min);
                        count[idx] += 1;
                    });

                // play back onto the input slice the counts collected with Sum of all counts == slice.len()
                let mut s_idx = 0;
                count.into_iter()
                    .enumerate()
                    .filter(|(_, x)| *x > 0)
                    .for_each(|(i, mut x)| {
                        // reverse index offset mapping
                        // hence, output[i] = Min + i
                        let val = min.wrapping_add(i as Self::NumType);
                        while x > 0 {
                            self[s_idx] = val;
                            s_idx += 1;
                            x -= 1;
                        }
                    });
               }
        })*
    }
}

impl_countsort!(i8,i16,i32);
// ANCHOR_END: sort_count


#[cfg(test)]
mod test {
    use crate::random_sequence;
    use super::*;
    #[test]
    fn test_countsort_head_to_head()
    {
        for _i in 0..127 {
            let v1: Vec<i8> = random_sequence(512);
            let mut v2 = v1.clone();

            v2.as_mut_slice().count_sort();
            let (_, v) = mergesort(&v1);
            assert_eq!( &v, &v2 );
        }
    }
    #[test]
    fn test_count_sort() {
        let test_data: [(&mut [i16], &[i16]);6] = [
            (&mut [13,12,11],              &[11,12,13]),
            (&mut [14,11,13,12],           &[11,12,13,14]),
            (&mut [28, 24, 22, 21],        &[21,22,24,28]),
            (&mut [36,32,34,33,35,31],     &[31,32,33,34,35,36]),
            (&mut [7,6,5,4,3,2,1],         &[1,2,3,4,5,6,7]),
            (&mut [113, 82, 122, 58, 16, -123, -58, -110],   &[-123, -110, -58, 16, 58, 82, 113, 122])
        ];

        test_data.into_iter()
            .for_each( | (input, output) | {
                input.count_sort();
                assert_eq!( input, output);
            });
    }
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
