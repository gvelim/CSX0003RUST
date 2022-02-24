use crate::merge::MergeIterator;

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
/// use csx3::{ merge::Merge, sort::merge::mergesort_mut };
///
/// let input = &mut [8, 4, 2, 1];
///
/// assert_eq!( mergesort_mut(input, Merge::merge_mut_adjacent), 6 );
/// assert_eq!( input, &[1,2,4,8] );
/// ```
pub fn mergesort_mut<T, F>(v: &mut [T], mut fn_merge: F ) -> usize
    where T: Ord ,
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
/// use csx3::sort::merge::mergesort;
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
#[cfg(test)]
mod test {
    use crate::random_sequence;
    use crate::merge::Merge;
    use super::*;
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
                assert_eq!(mergesort_mut(input, Merge::merge_mut), inv_count );
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
                assert_eq!(mergesort_mut(input, Merge::merge_mut_adjacent), inv_count );
                assert_eq!( input, output );
            })
    }
    #[test]
    fn test_mergesort_head_to_head()
    {
        for _ in 0..127 {
            let v1: Vec<i8> = random_sequence(512);
            let mut v2 = v1.clone();

            let inv = mergesort_mut(&mut v2, Merge::merge_mut);
            assert_eq!( mergesort(&v1), (inv, v2) );
        }
    }
}