use std::fmt::Debug;
use std::cmp::Ordering;
use rand::Rng;
use super::sort::{mergesort_mut, merge_mut_adjacent, partition_at_index};

// ANCHOR: selection_r
/// Find the nth order statistic within an unordered set with O(n) performance
/// using nth_min as 1 will return the smallest item; 2 the second smallest, etc
/// When function returns, the input array has been rearranged so that ```item == array[ nth order ]```
/// ```
/// use csx3::select::r_selection;
///
/// let (arr, nth_order) = (&mut [23,43,8,22,15,11], 1usize);
///
/// let ret_val = r_selection(arr, nth_order);
/// assert_eq!(ret_val, &8);
/// assert_eq!(&arr[nth_order-1], &8);
/// ```
pub fn r_selection<T>(v: &mut [T], nth_min: usize) -> &T
    where T: Copy + Ord + Debug  {

    // println!("Input: {:?}::{}th", v, order_nth);
    if v.len() == 1 {
        return &v[0];
    }

    // pick an index at random based on a uniform distribution
    let idx = rand::thread_rng().gen_range(0..(v.len()-1) );
    // find out the nth order of this sample
    let (left_partition, nth, right_partition) = partition_at_index(v, idx);

    let order = left_partition.len()+1;
    // println!("\tAsked:{}ord Picked:{}th, {:?} {:?}ord {:?}", nth_min, idx, left_partition, order, right_partition);

    // is nth order sampled over, equal or above the desired nth_min ?
    match nth_min.cmp(&order) {
        // we've found the item in nth_min order
        Ordering::Equal => nth,
        // the nth_min is below the nth found so recurse on the left partition
        Ordering::Less =>
            r_selection(left_partition, nth_min),
        // the nth_min is above the nth found so recurse on the right partition with adjusted order
        Ordering::Greater =>
            r_selection(right_partition, nth_min - order),
    }
}
// ANCHOR_END: selection_r

/// Find the nth order statistic within an unordered set with O(n) performance
/// using nth_min as 1 will return the smallest item; 2 the second smallest, etc
///
/// The algorithm aims to find the best pivot deterministically rather pick a random value
///
pub fn d_selection<T>(v: &mut [T], nth_min: usize) -> &T
    where T: Copy + Ord + Debug  {

    // println!("DS Input: {:?}::{}th", v, nth_min);
    if v.len() == 1 {
        return &v[0];
    }

    // pick an index deterministically
        // extract the medians vector
    let mut c = medians_of_medians(v);
        // recurse within the vector
        // to zoom into the ultimate median value
    let idx = c.len()/10;
    let p = d_selection(&mut c, idx );
        // I got the median value,
        // but I need the index for partitioning (argh!!)
        // so searching linearly to find the position (argh^2!!)
    // this is the best pivot we could get
    let idx = v.iter_mut().position(|i| *i == *p).unwrap();

    // find out the nth order of this sample
    let (left_partition, nth, right_partition) = partition_at_index(v, idx);

    let order = left_partition.len()+1;
    // println!("\tAsked:{}ord Picked:{}th, {:?} {:?}ord {:?} - {:?}", nth_min, idx, left_partition, order, right_partition, p);

    // is nth order sampled over, equal or above the desired nth_min ?
    match nth_min.cmp(&order) {
        // we've found the item in nth_min order
        Ordering::Equal => nth,
        // the nth_min is below the nth found so recurse on the left partition
        Ordering::Less =>
            d_selection(left_partition, nth_min),
        // the nth_min is above the nth found so recurse on the right partition with adjusted order
        Ordering::Greater =>
            d_selection(right_partition, nth_min - order),
    }
}

// ANCHOR: selection_median
/// Returns a vector of N/5 medians where N = input array length
/// It breaks array into N/5 sub-arrays of length 5 for cheap sorting and picking the median value
///
pub fn medians_of_medians<T>(v:&mut [T]) -> Vec<T>
    where T : Copy + Ord + Debug {

    // extract median of medians array
    // split input slice into n/5 groups of 5
    v.chunks_mut(5)
        .map(|chunk| {
            // sort each group
            mergesort_mut(chunk, merge_mut_adjacent);
            // pull the median out
            chunk[ chunk.len() >> 1]
        })
        // return as vector
        .collect()
}

/// Finds the median value within an array of N elements
pub fn find_median<T>(v:&mut [T]) -> T
    where T : Copy + Ord + Debug {

    if v.len() == 1 {
        return v[0]
    }
    let mut medians: Vec<T> = medians_of_medians(v);
    find_median(&mut medians)
}
// ANCHOR_END: selection_median

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_median() {
        let v =
            &mut [28, 59, -79,  38, -97, -55,  66,  61, -77, -97,
                 -37, 93, -22, 115, -22, -88, 101, 100,  48, -58,
                 100, 55, -49, 104, 110,  35, -86,-118, -10, 115,
                  55, 25,  83, -19,  87, -98, -30, 103, -14,  84,
                 -80,106, -32, -30, 113, 109,  45, -55,  89,  18];

        // Sorted output:
        // [-118, -98, -97, -97, -88, -86, -80, -79, -77, -58,
        //  -55,  -55, -49, -37, -32, -30, -30, -22, -22, -19,
        //  -14,  -10,  18,  25,**28**,35,  38,  45,  48,  55,
        //   55,   59,  61,  66,  83,  84,  87,  89,  93, 100,
        //  100,  101,  103, 104,106, 109, 110, 113, 115, 115]
        assert_eq!(find_median(v), 28);
    }

    #[test]
    fn test_deterministic_selection() {
        let test_data: [(&mut [u32], usize, &u32); 6] = [
            (&mut [23, 43, 8, 22, 15, 11], 1, &8),
            (&mut [23, 43, 8, 22, 15, 11], 2, &11),
            (&mut [23, 43, 8, 22, 15, 11], 3, &15),
            (&mut [23, 43, 8, 22, 15, 11], 4, &22),
            (&mut [23, 43, 8, 22, 15, 11], 5, &23),
            (&mut [23, 43, 8, 22, 15, 11], 6, &43),
        ];

        test_data.into_iter()
            .for_each(|(input, order, item)| {
                let ret_val = d_selection(input, order);
                assert_eq!(item, ret_val);
                assert_eq!(&input[order - 1], item);
            })
    }

    #[test]
    fn test_random_selection() {
        let test_data: [(&mut [u32], usize, &u32); 6] = [
            (&mut [23, 43, 8, 22, 15, 11], 1, &8),
            (&mut [23, 43, 8, 22, 15, 11], 2, &11),
            (&mut [23, 43, 8, 22, 15, 11], 3, &15),
            (&mut [23, 43, 8, 22, 15, 11], 4, &22),
            (&mut [23, 43, 8, 22, 15, 11], 5, &23),
            (&mut [23, 43, 8, 22, 15, 11], 6, &43),
        ];

        test_data.into_iter()
            .for_each(|(input, order, item)| {
                let ret_val = r_selection(input, order);
                assert_eq!(item, ret_val);
                assert_eq!(&input[order - 1], item);
            })
    }
}