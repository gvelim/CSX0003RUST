use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::ops::{Index, IndexMut};

/// Constructing a VirtualSlice allowing us to operate over
/// multiple non-adjacent slice segments as a "continuous slice"
/// ```
/// use csx3::utils::VirtualSlice;
///
/// let s1 = &mut [1, 3, 5, 7, 9];
/// let _s3 = &mut [0, 0, 0];
/// let s2 = &mut [2, 4, 6, 8 , 10];
///
/// {
///     let mut v = VirtualSlice::new();
///     v.chain(s1);
///     v.chain(s2);
///     v[0] = 11;
///     v[5] = 9;
/// }
/// assert_eq!(s1, &mut [11, 3, 5, 7, 9]);
/// assert_eq!(s2, &mut [9, 4, 6, 8 , 10]);
/// {
///     let mut v = VirtualSlice::new();
///     v.chain(s1);
///     v.chain(s2);
///     v.swap(0, 5);
/// }
/// assert_eq!(s1, &mut [9, 3, 5, 7, 9]);
/// assert_eq!(s2, &mut [11, 4, 6, 8 , 10]);
/// ```
pub struct VirtualSlice<'a, T> {
    vv: Vec<&'a mut T>
}

impl<'a, T> VirtualSlice<'a, T> {
    pub fn new() -> VirtualSlice<'a, T> {
        VirtualSlice {
            vv : Vec::new(),
        }
    }
    pub fn len(&self) -> usize {
        self.vv.len()
    }
    pub fn is_empty(&self) -> bool {
        self.vv.is_empty()
    }
    /// Append a slice segment onto the VirtualSlice
    pub fn chain(&mut self, s1: &'a mut [T]) {
         s1.iter_mut()
            .for_each(|item| {
                self.vv.push(item);
            });
    }
    /// Get a mutable iterator over the VirtualSlice that return mutable pointers *mut T
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, &'a mut T> {
        self.vv.iter_mut()
    }
    /// Swap two virtual positions that could correspond to between or within underlying slice segments
    pub fn swap(&mut self, a: usize, b:usize) {
        if a == b {
            return;
        }
        // we cannot use vv.swap as this will simply swap the position of the references
        // rather the referred values. Hence we use the pointers to swap the memory contents
        unsafe {
            std::ptr::swap::<T>(
                &mut self[a] as *mut T,
                &mut self[b] as *mut T
            )
        }
    }
    /// Merge Self with another non-adjacent slice using in-place memory swaps
    /// For the algorithm to work we need the following components
    /// - Append virtualslice with given &mut slice so to form a "continous slice"
    /// - Use for slice comparison an "Index Reflector (idx_rfl)" table to "project" (c,j, i') positions upon the "continuous slice" as (c', j', i)
    /// - Swap action at once both (a) continuous slice and (b) Index Reflector
    /// ```
    /// //Slice 1    Slice 2    VirtualSlice                  Index Reflector
    /// //=======    =========  ==========================   ======================
    /// //[5,6,7] <> [1,2,3,4]  [5(c'/i),6,7,1(j'),2,3,4]    [1(c/i'),2,3,4(j),5,6,7]
    /// //[1,6,7] <> [5,2,3,4]  [1,6(i),7,5(c'),2(j'),3,4]   [4(c),2(i'),3,1,5(j),6,7]
    /// //[1,2,7] <> [5,6,3,4]  [1,2,7(i),5(c'),6,3(j'),4]   [4(c),5,3(i'),1,2,6(j),7]
    /// //[1,2,3] <> [5,6,7,4]  [1,2,3,5(c'/i),6,7,4(j')]    [4(c/i'),5,6,1,2,3,7(j)]
    /// //[1,2,3] <> [4,6,7,5]  [1,2,3,4,6(i),7,5(c')](j')   [7(c),5(i'),6,1,2,3,4](j) <-- Main merge finished but still i < c'
    /// //[1,2,3] <> [4,5,7,6]  [1,2,3,4,5,7(i),6(c')](j')   [5,7(c),6(i'),1,2,3,4](j)
    /// //[1,2,3] <> [4,6,7,5]  [1,2,3,4,5,6,7(i/c')](j')    [5,6,7(c/i'),1,2,3,4](j) <-- finished merging (reflects starting position)
    ///
    /// use csx3::utils::VirtualSlice;
    /// let s1 = &mut [5,6,7];
    /// let _s = &[0,0,0,0,0,0]; // wedge to break adjacency
    /// let s2 = &mut [1,2,3,4];
    ///
    /// let mut vs = VirtualSlice::new();
    ///
    /// vs.merge(s1);
    /// vs.merge(s2);
    ///
    /// assert_eq!(s1, &[1,2,3]);
    /// assert_eq!(s2, &[4,5,6,7]);
    /// ```
    pub fn merge(&mut self, s: &'a mut [T]) -> usize
        where T: Ord + Debug {

        if self.is_empty() {
            self.chain(s);
            return 0
        }

        let j = self.len();

        self.chain(s);

        self.reorder_around_pivot(j)
    }
    /// Reorders a slice around a partition point j and given that
    /// - S1{ordered subslice} -> j -> S2{ordered subslice}
    /// - S1 overlaps S2 or not
    /// at completion the virtualslice will be ordered
    pub fn reorder_around_pivot(&mut self, mut j: usize) -> usize
        where T: Ord + Debug
    {
        // j = s2[j] equivalent position within the working slice (j') and index reflector (j)
        // i = partition position in working slice so that ... [merged elements] < ws[i] < [unmerged elements]
        // p = index reflector partition bound where i's position is always upper bounded by p
        // c = s1[c] equivalent position in the index reflector, so that idx_rfl[c] == c' == s1[c] equivalent in ws[c'],
        // used for optimising finding i pos in index array
        let (mut inv_count, mut c, mut i, p) = (0usize, 0usize, 0usize, j);
        // ws_len = working slice's length
        let ws_len = self.len();
        let mut idx_rfl = (0..self.len()).into_iter().collect::<Vec<usize>>();

        println!("-:Merge:{:?} :: {:?} ({:?},{:?},{:?})", self, idx_rfl, i, j, c);

        // j == v.len() => no more comparisons since ws[j] is the rightmost, last and largest of the two slices
        // i == j => no more comparison required, since everything in ws[..i] << ws[j]
        while j < ws_len && i != j {
            match ( self[idx_rfl[c]] ).cmp( &self[idx_rfl[j]] ) {
                Ordering::Less | Ordering::Equal => {

                    // swap left slice's item in the working slice with merged partition edge ws[i]
                    // swap( ws[i] with ws[c'] where c' = index_reflector[c]
                    self.swap(i, idx_rfl[c] );

                    // swap index_reflect[c] with index_reflector[i']
                    // i' == index_reflector[x]; where x == i;
                    // e.g. i = 3rd pos, hence i' = index_reflector[x] where x == 3;
                    let idx = idx_rfl[c..p].iter().position(|x| *x == i).unwrap() + c;
                    // swap( i' with c )
                    idx_rfl.swap(idx, c);
                    print!("l:");
                    // point to the next in order position (left slice)
                    c += 1;
                }
                Ordering::Greater => {
                    // count the equivalent inversions
                    inv_count += j - i;

                    // swap right slice's item in the working slice with merged partition edge ws[i]
                    // swap( ws[i] with ws[j'] where j' = index_reflector[j]
                    self.swap(i, idx_rfl[j]);

                    // swap index_reflect[j] with index_reflector[i']
                    // i' == index_reflector[x]; where x == i;
                    // e.g. i = 3rd pos, hence i' = index_reflector[x] where x == 3;
                    let idx = idx_rfl[c..p].iter().position(|x| *x == i).unwrap() + c;
                    // swap( i' with j )
                    idx_rfl.swap(idx, j);
                    print!("r:");
                    // point to the next in order position (right slice)
                    j += 1;
                }
            }
            // Move partition by one so that [merged partition] < ws[i] < [unmerged partition]
            i += 1;
            println!("Merge:{:?} :: {:?} ({:?},{:?},{:?})",self, idx_rfl, i, j, c);
        }

        // Edge cases: sorting completed with [merged] < ith pos < [unmerged]
        // however [unmerged] are likely not ordered due to swapping
        // example:
        // [5(i/c),6,7] <> [1(j),2,3,4]
        // [1,6(i),7] <> [5(c),2(j),3,4]
        // [1,2,7(i)] <> [5(c),6,3(j),4]
        // [1,2,3] <> [5(c/i),6,7,4(j)]
        // [1,2,3] <> [5(c/i),6,7,4(j)]
        // [1,2,3] <> [4,6(i),7,5(c)] (j) <-- Finished merge however we are left with an unordered rightmost part
        // since i pos << c pos, hence we need to address this segment
        while i < idx_rfl[c] {
            if let Ordering::Less = (self[idx_rfl[c]]).cmp( &self[i] ) {
                // swap i with c' in working slice
                self.swap(i, idx_rfl[c] );

                // extract i' from index_reflector[]
                let idx = idx_rfl[c..p].iter().position(|x| *x == i).unwrap() + c;

                // swap i' with c
                idx_rfl.swap(idx, c);

                // point to the next in order position
                c += 1;
            }
            // Move partition by one so that [merged partition] < ws[i] < [unmerged partition]
            i += 1;
            println!("f:Merge:{:?} :: {:?} ({:?},{:?},{:?})",self, idx_rfl, i, j, c);
        }
        inv_count
    }}


impl<T> Default for VirtualSlice<'_, T> {
    fn default() -> Self {
        VirtualSlice::new()
    }
}

impl<T> Debug for VirtualSlice<'_, T> where T : Debug {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entries(
                self.vv.iter().map( |x| &**x )
            )
            .finish()
    }
}

impl<T> Index<usize> for VirtualSlice<'_, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.vv[index]
    }
}

impl<T> IndexMut<usize> for VirtualSlice<'_, T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.vv[index]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_virtual_slice_merge()
    {
        let s1 = &mut [5,6,7];
        let _x = &[0,0,0,0,0,0]; // wedge to break adjacency
        let s2 = &mut [1,2,3,4];
        let _y = &[0,0,0,0,0,0]; // wedge to break adjacency
        let s3 = &mut [10,12,14];
        let _z = &[0,0,0,0,0,0]; // wedge to break adjacency
        let s4 = &mut [8,9,15,16];

        {
            let mut vs = VirtualSlice::new();
            vs.merge(s1);
            vs.merge(s2);
            vs.merge(s3);
            vs.merge(s4);
        }


        assert_eq!(s1, &mut [1,2,3]);
        assert_eq!(s2, &mut [4,5,6,7]);
        assert_eq!(s3, &mut [8,9,10]);
        assert_eq!(s4, &mut [12,14,15,16]);
    }
    #[test]
    fn test_virtual_slice_new_and_iter() {
        let s1 = &mut [1, 3, 5, 7, 9];
        let _s3 = &mut [0, 0, 0];
        let s2 = &mut [2, 4, 6, 8 , 10];

        {
            let mut v = VirtualSlice::new();
            v.chain(s1);
            v.chain(s2);
            println!("{:?}", v);
            v.iter_mut()
                .for_each(|ptr| {
                    **ptr = 12;
                });
            v[0] = 11;
            v[5] = 9;
            println!("{:?}", v);
        }
        assert_eq!(s1, &mut [11, 12, 12, 12, 12]);
        assert_eq!(s2, &mut [9, 12, 12, 12, 12]);
        {
            let mut v = VirtualSlice::new();
            v.chain(s1);
            v.chain(s2);
            v.swap(0, 5);
        }
        assert_eq!(s1, &mut [9, 12, 12, 12, 12]);
        assert_eq!(s2, &mut [11, 12, 12, 12, 12]);
    }
}