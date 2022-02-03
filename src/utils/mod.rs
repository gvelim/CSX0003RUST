use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::ops::{Index, IndexMut, Range};


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
pub enum VirtualSlice<'a, T> where T: Ord {
    NonAdjacent( Vec<&'a mut T> ),
    Adjacent( &'a mut[T] ),
}

use VirtualSlice::{NonAdjacent, Adjacent};

impl<'a, T> VirtualSlice<'a, T> where T: Ord {
    pub fn new() -> VirtualSlice<'a, T> {
        NonAdjacent( Vec::new() )
    }
    pub fn new_adjacent(s: &'a mut[T]) -> VirtualSlice<'a, T> {
        Adjacent( s )
    }
    pub fn len(&self) -> usize {
        match self {
            NonAdjacent(v) => v.len(),
            Adjacent(s) => s.len(),
        }
    }
    pub fn is_empty(&self) -> bool {
        match self {
            NonAdjacent(v) => v.is_empty(),
            Adjacent(s) => s.is_empty(),
        }
    }
    /// Append a slice segment onto the VirtualSlice
    pub fn chain(&mut self, s: &'a mut [T]) {
        if let NonAdjacent(v) = self {
            s.iter_mut()
                .for_each(|item| {
                    v.push(item);
                });
        }
    }
    pub fn chain_adjacent(&mut self, s: &'a mut [T]) {
        if let Adjacent(s0) = self {
            let fs: &mut [T];
            unsafe {
                fs = &mut *std::ptr::slice_from_raw_parts_mut::<T>(s0.as_mut_ptr(), s0.len() + s.len());
                // checking they are aligned and adjacent,
                // if not panic! so we prevent unpredictable behaviour
                assert!(&s[0] == &fs[s0.len()]);
            }
            *self = VirtualSlice::new_adjacent(fs);
        }
    }
    /// Get a mutable iterator over the VirtualSlice that return mutable references &mut T
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, &'a mut T> {
        if let NonAdjacent(v) = self {
            v.iter_mut()
        } else {
            panic!()
        }
    }
    pub fn iter_mut_adjacent(&mut self) -> std::slice::IterMut<'_, T> {
        if let Adjacent(s) = self {
            s.iter_mut()
        } else {
            panic!()
        }
    }
    /// Swap two referenced positions that could correspond to between or within underlying slice segments
    pub fn swap(&mut self, a: usize, b:usize) {
        if a == b {
            return;
        }
        // we cannot use vv.swap as this will simply swap the position of the pointers
        // rather where the pointers point to. Hence we use the pointers to swap the memory contents
        unsafe {
            std::ptr::swap::<T>(
                &mut self[a] as *mut T,
                &mut self[b] as *mut T
            )
        }
    }
    /// Merge Self with another non-adjacent slice using in-place memory swaps
    /// For the algorithm to work we need the following components
    /// - Append VirtualSlice with given &mut slice so to form a "continuous slice"
    /// - Use for slice comparison an "Index Reflector (idx_rfl)" table to "project" (c,i') positions upon the "continuous slice" as (c', i)
    /// - Swap action at once both (a) continuous slice and (b) Index Reflector
    /// ```
    /// //Slice 1    Slice 2    VirtualSlice                Index Reflector
    /// //=======    =========  =========================   =============
    /// //[5,6,7] <> [1,2,3,4]  [5(c'/i),6,7,1(j),2,3,4]    [1(c/i'),2,3,]
    /// //[1,6,7] <> [5,2,3,4]  [1,6(i),7,5(c'),2(j),3,4]   [4(c),2(i'),3]
    /// //[1,2,7] <> [5,6,3,4]  [1,2,7(i),5(c'),6,3(j),4]   [4(c),5,3(i')]
    /// //[1,2,3] <> [5,6,7,4]  [1,2,3,5(c'/i),6,7,4(j)]    [4(c/i'),5,6,]
    /// //[1,2,3] <> [4,6,7,5]  [1,2,3,4,6(i),7,5(c')](j)   [7(c),5(i'),6] <-- Phase 1: Main merge finished but still i < c'
    /// //[1,2,3] <> [4,5,7,6]  [1,2,3,4,5,7(i),6(c')](j)   [5,7(c),6(i')]     Trick: reflector knows the right order remaining
    /// //[1,2,3] <> [4,6,7,5]  [1,2,3,4,5,6,7(i/c')](j)    [5,6,7(c/i') ] <-- Phase 2: finished merging (reflects starting position)
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

        // j = s2[j] equivalent position within the working slice (j') and index reflector (j)
        let mut j = self.len();

        match self {
            NonAdjacent(_) => self.chain(s),
            Adjacent(_) => self.chain_adjacent(s),
        };

        // i = partition position in working slice so that ... [merged elements] < ws[i] < [unmerged elements]
        // p = index reflector partition bound where i's position is always upper bounded by p
        // c = s1[c] equivalent position in the index reflector, so that idx_rfl[c] == c' == s1[c] equivalent in ws[c'],
        // used for optimising finding i pos in index array
        let (mut inv_count, mut c, mut i) = (0usize, 0usize, 0usize);

        // ws_len = working slice's length
        let ws_len = self.len();

        // Build the index reflector of size [ 0 .. size of left slice] since the following properties apply
        // - c & i' will never exceed size of left slice
        // - j == j' always be the same position
        let mut idx_rfl = (0..j).into_iter().collect::<Vec<usize>>();

        //println!("-:Merge:{:?} :: {:?} ({:?},{:?},{:?})", self, idx_rfl, i, j, c);

        // Phase 1 : Conditions
        // j == v.len() => no more comparisons since ws[j] is the rightmost, last and largest of the two slices
        // i == j => no more comparison required, since everything in ws[..i] << ws[j]
        while j < ws_len && i != j {
            match ( self[idx_rfl[c]] ).cmp( &self[j] ) {
                Ordering::Less | Ordering::Equal => {

                    // swap left slice's item in the working slice with merged partition edge ws[i]
                    // swap( ws[i] with ws[c'] where c' = index_reflector[c]
                    self.swap(i, idx_rfl[c] );

                    // swap index_reflect[c] with index_reflector[i']
                    // i' == index_reflector[x]; where x == i;
                    // e.g. i = 3rd pos, hence i' = index_reflector[x] where x == 3;
                    let idx = idx_rfl[c..].iter().position(|x| *x == i).unwrap() + c;
                    //swap( i' with c )
                    idx_rfl.swap(idx, c);
                    //print!("\tl:");
                    // point to the next in order position (left slice)
                    c += 1;
                }
                Ordering::Greater => {
                    // count the equivalent inversions
                    inv_count += j - i;

                    // swap right slice's item in the working slice with merged partition edge ws[i]
                    // swap( ws[i] with ws[j'] where j' = index_reflector[j]
                    self.swap(i, j);

                    // swap index_reflect[j] with index_reflector[i']
                    // i' == index_reflector[x]; where x == i;
                    // e.g. i = 3rd pos, hence i' = index_reflector[x] where x == 3;
                    let idx = idx_rfl[c..].iter().position(|x| *x == i).unwrap() + c;
                    // swap( i' with j )
                    // since always j == j' we just copy the value over no need to swap
                    idx_rfl[idx] = j;
                    //print!("\tr:");
                    // point to the next in order position (right slice)
                    j += 1;
                }
            }
            // Move partition by one so that [merged partition] < ws[i] < [unmerged partition]
            i += 1;
            //println!("Phase 1: Merge:{:?} :: {:?} ({},{},{}={})",self, idx_rfl, i, j, c, idx_rfl[c]);
        }

        // Phase 2 : Finalise the trailing ends remaining after rightmost part has been exhausted,
        // Conditions: i == [c], i == ws_len-1, c == p-1
        //
        // Here is an example:
        // [5(i/c),6,7] <> [1(j),2,3,4]
        // [1,6(i),7] <> [5(c),2(j),3,4]
        // [1,2,7(i)] <> [5(c),6,3(j),4]
        // [1,2,3] <> [5(c/i),6,7,4(j)]
        // [1,2,3] <> [5(c/i),6,7,4(j)]
        // [1,2,3] <> [4,6(i),7,5(c)] (j) <-- Finished merge however we are left with an unordered rightmost part
        //                                    [1,2,3,4,6(i),7,5(c')] = VirtualSlice needs to be ordered between i..c'
        // Tip: the index_reflector already stores the correct order of the trailing items
        // all we have to do is to let it guide the remaining swapping
        let c_bound = idx_rfl.len()-1;
        let i_bound = ws_len-1;
        while i < i_bound && c < c_bound {

            // condition saves cpu-cycles from zero-impact operations when i == c' (no swap)
            // otherwise it has no algorithmic impact
            if i != idx_rfl[c] {
                // swap i with c' in working slice
                self.swap(i, idx_rfl[c]);

                // extract i' from index_reflector[]
                let idx = idx_rfl[c..].iter().position(|x| *x == i).unwrap() + c;

                // swap i' with c
                idx_rfl.swap(idx, c);

                //println!("\ts:Merge:{:?} :: {:?} ({i},{j},{c}={},{p})", self, idx_rfl, idx_rfl[c]);
            }
            // point to the next in order position,
            // so that idx_rfl[c] point to the right ws['c] item to be swapped
            c += 1;
            // Move partition by one so that [merged partition] < ws[i] < [unmerged partition]
            i += 1;
        }

        //println!("Merge Done");
        inv_count
    }
}

impl<T> Default for VirtualSlice<'_, T> where T: Ord {
    fn default() -> Self {
        VirtualSlice::new()
    }
}

impl<T> Debug for VirtualSlice<'_, T> where T : Ord + Debug {

    /// extract and display the slice subsegments attached to the virtualslice
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NonAdjacent(v) =>{
                f.debug_list()
                    .entries(
                        // get to the actual element referenced
                        // since we got a *pointer -> virtual slice *pointer -> slice segment item
                        v.iter().map(|x| &**x)
                    )
                    .finish()
            }
            Adjacent(s) => {
                f.debug_list()
                    .entries(
                        // get to the actual element referenced
                        // since we got a *pointer -> virtual slice *pointer -> slice segment item
                        s.iter()
                    )
                    .finish()
            }
        }
    }
}

impl<T> Index<usize> for VirtualSlice<'_, T> where T: Ord {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        // syntactic overkill as rust will automatically dereference the chain of references
        // but it feels good to be explicit!!
        match self {
            NonAdjacent(vv) => &(*vv[index]),
            Adjacent(s) => &s[index],
        }
    }
}

impl<'a, T> Index<Range<usize>> for VirtualSlice<'a, T> where T: Ord {
    type Output = [&'a mut T];

    fn index(&self, index: Range<usize>) -> &Self::Output {
        if let NonAdjacent(vv) = self {
            &vv[index]
        } else {
            panic!()
        }
    }
}

impl<T> IndexMut<usize> for VirtualSlice<'_, T> where T: Ord {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        // syntactic overkill as rust will automatically dereference the chain of references
        // but it feels good to be explicit!!
        match self {
            NonAdjacent(vv) => &mut (*vv[index]),
            Adjacent(s) => &mut s[index],
        }
    }
}

impl<'a, T> IndexMut<Range<usize>> for VirtualSlice<'a, T> where T: Ord {
    fn index_mut(&mut self, index: Range<usize>) -> &mut Self::Output {
        if let NonAdjacent(vv) = self {
            &mut vv[index]
        } else {
            panic!()
        }
    }
}

impl<'a, T> PartialOrd for VirtualSlice<'a, T> where T: Ord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (NonAdjacent(v), NonAdjacent(o)) => v.partial_cmp(o),
            (Adjacent(s), Adjacent(o)) => s.partial_cmp(o),
            ( _, _ ) => panic!(),
        }
    }
}

impl<'a, T> PartialEq<Self> for VirtualSlice<'a, T> where T: Ord  {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (NonAdjacent(v), NonAdjacent(o)) => v.eq(o),
            (Adjacent(s), Adjacent(o)) => s.eq(o),
            ( _, _ ) => panic!(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    #[should_panic]
    fn test_virtual_slice_adjacent_panic() {
        let s1 = &mut [1, 3, 5, 7, 9];
        let _s = &mut [0,0,0,0];
        let s2 = &mut [2, 4, 6, 8, 10];
        let mut vs = VirtualSlice::new_adjacent(s1);
        vs.chain_adjacent(s2);
    }
    #[test]
    fn test_virtual_slice_merge_adjacent() {
        let s1 = &mut [1, 3, 5, 7, 9];
        let s2 = &mut [2, 4, 6, 8, 10];
        let mut vs = VirtualSlice::new_adjacent(s1);
        vs.chain_adjacent(s2);
        println!("{:?}",vs);
        assert_eq!(vs, Adjacent( &mut [1,3,5,7,9,2,4,6,8,10] ) );
        vs.iter_mut_adjacent()
            .for_each(|x| {
                *x = 12;
            });
        vs[0] = 11;
        vs[5] = 9;
        println!("{:?}",vs);
        assert_eq!(vs, Adjacent( &mut [11,12,12,12,12,9,12,12,12,12] ) );
    }
    #[test]
    fn test_virtual_slice_merge() {
        let test_data: [(&mut[i32], &mut[i32], &[i32],&[i32]); 6] = [
            (&mut[-88,-29,4,84],                             &mut[-127,-113,-71,-54],
                &[-127,-113,-88,-71],                           &[-54,-29,4,84]),
            (&mut[5,6,7],                                    &mut[1,2,3,4],
                &[1,2,3],                                       &[4,5,6,7]),
            (&mut[-127, -81, -55, -38, 40, 78, 122, 124],    &mut[-126, -123, -102, -78, -51, -44, -29, 17],
                &[-127, -126, -123, -102, -81, -78, -55, -51],  &[-44, -38, -29, 17, 40, 78, 122, 124]),
            (&mut[-69, -18, -8, 3, 38, 68, 69, 74],          &mut[-119, -83, -81, -76, -37, -13, 40, 77],
                &[-119, -83, -81, -76, -69, -37, -18, -13],     &[-8, 3, 38, 40, 68, 69, 74, 77]),
            (&mut[-106, -82, -64, -57, 5, 23, 67, 79],       &mut[-103, -85, -85, -49, -42, -38, -37, 86],
                &[-106, -103, -85, -85, -82, -64, -57, -49],    &[-42, -38, -37, 5, 23, 67, 79, 86]),
            (&mut[-122, -19, 3, 51, 69, 77, 78, 115],        &mut[-118, -99, 23, 23, 35, 59, 63, 75],
                &[-122, -118, -99, -19, 3, 23, 23, 35],         &[51, 59, 63, 69, 75, 77, 78, 115])
        ];

        for (s1,s2, c1, c2) in test_data {
            let mut vs = VirtualSlice::new();
            vs.merge(s1);
            vs.merge(s2);
            assert_eq!(s1, c1);
            assert_eq!(s2, c2);
        }

    }
    #[test]
    fn test_virtual_slice_merge_multiple()
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
    fn test_virtual_slice_range() {
        let s1 = &mut [1, 3, 5, 7, 9];
        let _s3 = &mut [0, 0, 0];
        let s2 = &mut [2, 4, 6, 8, 10];

        let mut vs = VirtualSlice::new();
        vs.chain(s1);
        vs.chain(s2);

        vs[3..6]
            .iter_mut()
            .for_each(|x| **x = 12);

        assert_eq!( &vs[2..7], &[&5,&12,&12,&12,&4]);
        assert_eq!( s1, &mut [1, 3, 5, 12, 12] );
        assert_eq!( s2, &mut [12, 4, 6, 8, 10] );
    }
    #[test]
    fn test_virtual_slice_new_iter_swap() {
        let s1 = &mut [1, 3, 5, 7, 9];
        let _s3 = &mut [0, 0, 0];
        let s2 = &mut [2, 4, 6, 8 , 10];

        {
            let mut v = VirtualSlice::new();
            v.chain(s1);
            v.chain(s2);

            v.iter_mut()
                .for_each(|ptr| {
                    **ptr = 12;
                });
            v[0] = 11;
            v[5] = 9;
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