use super::Partition;

// ANCHOR: sort_quick
pub trait QuickSort {
    fn quick_sort(&mut self);
}

impl<T> QuickSort for [T]
    where T: Copy + Clone + Ord {

    /// Sorts a given array using the Quick Sort algorithm.
    /// The function rearranges the array contents rather than returning a new sorted copy of the input array
    /// ```
    /// use csx3::sort::quick::QuickSort;
    ///
    /// let v = &mut [3,5,8,1,2,4,6,0];
    ///
    /// v.quick_sort();
    /// assert_eq!(v, &[0,1,2,3,4,5,6,8]);
    /// ```
    fn quick_sort(&mut self) {

        // have we reached the end of the recursion ?
        if self.len() < 2 {
            return;
        }
        // pick an index at random based on a uniform distribution
        let idx = self.len() >> 1;
        // partition the array into to mutable slices for further sorting
        let (left_partition,_ , right_partition) = self.partition_at_idx(idx);

        // Recurse against left an right partitions
        left_partition.quick_sort();
        right_partition.quick_sort();
    }
}
// ANCHOR_END: sort_quick


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

        test_data.into_iter()
            .for_each( | (input, output) | {
                input.quick_sort();
                assert_eq!(input, output);
            })
    }
    #[test]
    fn test_partition_at_index() {
        let mut v = vec![6,12,5,9,7,8,11,3,1,4,2,10];
        let (l, idx, r) = v.partition_at_idx(4);

        // [2, 5, 6, 3, 1, 4],7,[9, 12, 8, 11, 10]
        // idx = &7 (7th position)
        assert_eq!(l, &[2,5,6,3,1,4]);
        assert_eq!(idx, &7);
        assert_eq!(r, &[9,12,8,11,10]);
    }
}