pub mod merge;
pub mod quick;
pub mod count;

// ANCHOR: sort_quick_partition
pub trait Partition<T> {
    fn partition_at_idx(&mut self, idx: usize) -> (&mut [T], &mut T, &mut [T]);
}

impl<T> Partition<T> for [T]
    where T: Copy + Clone + Ord {

    /// Splits an array into two mutable slices/partitions around a pivot location index
    /// so that *[values in left partition] < [pivot] < [values in right partition]*
    /// ```
    /// use csx3::sort::*;
    /// use csx3::sort::Partition;
    /// let mut v = vec![6,12,5,9,7,8,11,3,1,4,2,10];
    /// let (l, idx, r) = v.partition_at_idx(4);
    ///
    /// // [2, 5, 6, 3, 1, 4],7,[9, 12, 8, 11, 10]
    /// // idx = &7 (6th position using zero based index)
    /// assert_eq!(l, &[2,5,6,3,1,4]);
    /// assert_eq!(idx, &7);
    /// assert_eq!(r, &[9,12,8,11,10]);
    /// ```
    fn partition_at_idx(&mut self, idx: usize) -> (&mut [T], &mut T, &mut [T]) {

        let len = self.len();
        assert!(idx < len);

        let mut i = 0usize;

        // swap v[idx] to v[0] before entering the for loop
        self.swap(0, idx);

        // the for_each will own the &mut v anything we need within the loop
        // we'll have to get it before we get in
        let pivot = self[0];
        let ptr = self.as_mut_ptr();

        // v[0] holds the pivot point hence we start comparing from 2nd item v[1]
        // j : points to last element checked
        // i : position in array so that v[1..i] < v[i] < r[i+1..j]
        self.iter_mut()
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
        self.swap(0,i);
        //println!("\tf:{:?}, ({})", v, i+1);

        // split the array into [left part], [pivot + right partition]
        let (l, r) = self.split_at_mut(i);
        // split further into [pivot], [right partition]
        let (p, r) = r.split_at_mut(1);

        (l, &mut p[0], r)
    }
}
// ANCHOR_END: sort_quick_partition

#[cfg(test)]
mod test {
    use super::*;
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