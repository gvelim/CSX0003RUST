use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::ops::{Index, IndexMut, Range};

/// Constructing a VirtualSlice allowing us to operate over
/// multiple non-adjacent slice segments as a "continuous slice"
/// ```
/// use csx3::merge::vs::VirtualSlice;
///
/// let v = &mut [1, 3, 5, 7, 9, 2, 4, 6, 8, 10];
/// let (s1, s2) = v.split_at_mut(5);
/// let _s3 = &mut [0, 0, 0, 0,0];   // Wedge this to break stack continuity
/// let s4 = &mut [2, 4, 6, 8, 10];
///
/// let mut v = VirtualSlice::new_adjacent(s1);
/// v.merge(s2);
/// v.iter()
///     .enumerate()
///     .for_each(|(i,x)| assert_eq!(*x,i+1) );
///
/// assert_eq!(s1, &mut [1, 2, 3, 4, 5]);
/// assert_eq!(s2, &mut [6, 7, 8, 9, 10]);
///
/// let mut v = VirtualSlice::new( s1.len() + s2.len() );
/// v.attach(s1);
/// v.attach(s4);
/// v[0] = 11;
/// v[5] = 9;
/// v.swap(0, 5);
///
/// assert_eq!(s1, &mut [9, 2, 3, 4, 5]);
/// assert_eq!(s4, &mut [11, 4, 6, 8 , 10]);
///
/// ```
pub enum VirtualSlice<'a, T> where T: Ord + Debug {
    /// The tuple holds a vector of mutable references and the Index Reflector
    NonAdjacent( Vec<&'a mut T> ),
    /// Holds a mutable reference to the reconstructed parent slice out of two memory adjacent slices
    Adjacent( &'a mut[T] ),
}

use VirtualSlice::{NonAdjacent, Adjacent};

impl<'a, T> VirtualSlice<'a, T> where T: Ord + Debug {
    /// Create a new VirtualSlice for use with non-adjacent slice segments
    pub fn new(length :usize) -> VirtualSlice<'a, T> {
        NonAdjacent( Vec::with_capacity(length) )
    }
    /// Create a new VirtualSlice for use with adjacent slice segments
    pub fn new_adjacent(s: &'a mut[T]) -> VirtualSlice<'a, T> {
        Adjacent( s )
    }
    /// Current length of the VirtualSlice is equal to sum of all attached slice segments
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
    /// Get a mutable iterator over the VirtualSlice that return mutable references &mut T
    pub fn iter_mut<'b: 'a>(&'b mut self) -> VSIterMut<'b, T> where T: 'b {
        VSIterMut::new(self)
    }
    /// Get an immutable iterator over the VirtualSlice that return mutable references &mut T
    pub fn iter(&self) -> VSIter<'_, T> {
        VSIter::new(self)
    }
    /// Attach a slice segment onto the VirtualSlice
    pub fn attach(&mut self, s: &'a mut [T]) {
        match self {
            NonAdjacent(v) => {
                v.extend(s.iter_mut());
            }
            Adjacent(s0) => {
                let fs: &mut [T];
                unsafe {
                    fs = &mut *std::ptr::slice_from_raw_parts_mut::<T>((*s0).as_mut_ptr(), s0.len() + s.len());
                    // checking they are aligned and adjacent,
                    // if not panic! so we prevent unpredictable behaviour
                    assert!(s[0] == fs[s0.len()]);
                }
                *self = VirtualSlice::new_adjacent(fs);
            }
        }
    }
    /// Perform a deep merge by ordering the referred values hence mutating the slice segments
    pub fn merge(&mut self, s: &'a mut [T]) -> usize
        where T: Ord  {
        self._merge(s, VirtualSlice::swap)
    }
    /// Shallow swap; swaps the references of the underlying slice segments. The segments aren't affected
    /// Operates only with non-adjacent slices
    pub fn swap_shallow(&mut self, a: usize, b:usize) {
        if let NonAdjacent(v) = self {
            v.swap(a, b);
        } else {
            panic!("swap_shallow(): Not applicable for Adjacent VirtualSlices; use with VirtualSlice::new() instead");
        }
    }
    /// Perform a shallow merge by ordering the VirtualSlice's references and not the referred values.
    /// The VirtualSlice can be used as sort-mask layer above the slice segments, which later can be superimposed over
    /// In case of non-adjacent slices only.
    pub fn merge_lazy(&mut self, s: &'a mut [T]) -> usize
        where T: Ord  {
        self._merge(s, VirtualSlice::swap_shallow)
    }
    /// Deep swap; swaps the two references to the positions of the underlying slice segments
    /// Operates at both adjacent and non-adjacent slices
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
    /// use csx3::merge::vs::VirtualSlice;
    /// let s1 = &mut [5,6,7];
    /// let _s = &[0,0,0,0,0,0]; // wedge to break adjacency
    /// let s2 = &mut [1,2,3,4];
    ///
    /// let mut vs = VirtualSlice::new( s1.len() + s2.len());
    ///
    /// vs.merge(s1);
    /// vs.merge(s2);
    ///
    /// assert_eq!(s1, &[1,2,3]);
    /// assert_eq!(s2, &[4,5,6,7]);
    /// ```
    fn _merge<F>(&mut self, s: &'a mut [T], mut f_swap: F) -> usize
        where T: Ord ,
              F: FnMut(&mut Self, usize, usize) {

        if self.is_empty() {
            self.attach(s);
            return 0
        }

        // j = s2[j] equivalent position within the working slice (j') and index reflector (j)
        let mut j = self.len();

        // attach slice to be merged with self
        self.attach(s);

        // i = partition position in working slice so that ... [merged elements] < ws[i] < [unmerged elements]
        // p = index reflector partition bound where i's position is always upper bounded by p
        // c = s1[c] equivalent position in the index reflector, so that idx_rfl[c] == c' == s1[c] equivalent in ws[c'],
        // used for optimising finding i pos in index array
        let (mut inv_count, mut c, mut i, p) = (0usize, 0usize, 0usize, j);

        // ws_len = working slice's length = self.len + s.len
        let ws_len = self.len();

        // Memory Optimisation: we could build the index reflector of size [ 0 .. size of left slice] since the following properties apply
        // - c & i' will never exceed size of left slice
        // - j == j' always be the same position
        let mut idx_rfl : Vec::<usize> = (0..ws_len).collect();

        //println!("Merge:{self:?} :: {idx_rfl:?} (i:{i},j:{j},c:{c})");

        let mut cc;
        let mut ii;
        let base : *mut usize = idx_rfl.as_mut_ptr();
        loop {
            // Flattening/de-normalising the workflow logic
            // ============================================
            // A: (i != j or j < ws_len ) => Any more comparisons required ? is everything in ws[..i] << ws[j] ?
            // B: ( i != [c] where i < ws_len-1, c < p-1 ) => Have all left slice elements been processed ? Have we reached the end where i == [c] ?
            // +------+-------+----------+---------------------------------------
            // |   A  |   B   | if Guard | Action
            // +------+-------+----------+---------------------------------------
            // | true |  true |   l > r  | Phase 1: swap right with pivot
            // | true |  true |    N/A   | Phase 1: l<=r implied; swap left with pivot
            // |false |  true |    ANY   | Phase 2: finishing moving/reordering remaining items
            // | true | false |    N/A   | Exit: Merge completed; finished left part, right part remaining is ordered
            // |false | false |    N/A   | Exit: Merge completed
            // +------+-------+----------+---------------------------------------
            unsafe {// translate index_reflector[c] --> vs[c']
                // where index_reflector[c] predicts position of c' in ws[] given current iteration
                cc = *base.add(c);
                // translate index_reflector[i] --> index_reflector[i']
                // where index_reflector[i] predicts position of i' in index_reflector[] given current iteration
                ii = *base.add(i);
            }
            match (j < ws_len && i != j, i < ws_len-1 && c < p-1) {
                (true, _) if self[cc].cmp(&self[j]) == Ordering::Greater => {
                    // count the equivalent number of inversions
                    inv_count += j - i;

                    // swap data, right slice's item in the working slice with merged partition edge ws[i]
                    // swap( ws[i] with ws[j'] where j' = index_reflector[j], but j' == j so
                    f_swap(self, i, j);

                    unsafe {
                        // swap indexes, index_reflect[j] with index_reflector[i']
                        // since we don't use the index reflector property for superimposing just copy j over to i'
                        base.add(ii).replace(j);
                        // Store i' at position [j] to use when i reaches j value
                        // e.g. if j = 10, [i'] = 4, then when i becomes 10, index_reflector[10] will move i' to 4
                        base.add(j).replace(ii);
                    }
                    //print!("\tr:");
                    // point to the next in order position (right slice)
                    j += 1;
                },
                (_, true) => {
                    // condition saves cpu-cycles from zero-impact operations when i == c' (no swap)
                    // otherwise it has no algorithmic impact
                    if i != cc {
                        // swap left slice's item in the working slice with merged partition edge ws[i]
                        // swap( ws[i] with ws[c'] where c' = index_reflector[c]
                        f_swap(self, i, cc);

                        unsafe {
                        // Store i' at position [c'] to use when i == c'
                        // for example, with c' = [c] and i' = [i]
                        // given [c=2]=9, [i=5]=2, then we store at idx_rfl[9] the value 5 as i is at position 5
                        // that means when i becomes 9, the i' will have position 5
                            base.add(cc).replace(ii);
                        // swap index_reflect[c] with index_reflector[i']
                            std::ptr::swap( base.add(ii), base.add(c) );
                        }
                        //print!("\tl:");
                    }
                    // point to the next in order position (left slice)
                    c += 1;
                },
                (_, _) => break,
            };
            // Move partition by one so that [merged partition] < ws[i] < [unmerged partition]
            i += 1;
            //println!("Merge:{self:?} :: {idx_rfl:?} (i:{i},j:{j},c:{c})");
        };

        //println!("Merge Done");
        inv_count
    }
}

pub enum VSIter<'b, T> where T: Ord + 'b {
    NonAdjacent( std::slice::Iter<'b, &'b mut T> ),
    Adjacent( std::slice::Iter<'b, T> ),
}
impl<'b, T> VSIter<'b, T> where T: Ord + 'b + Debug {
    pub fn new(vs: &'b VirtualSlice<'b, T>) -> VSIter<'b, T> {
        match vs {
            NonAdjacent(v) => VSIter::NonAdjacent(v.iter()),
            Adjacent(s) => VSIter::Adjacent(s.iter()),
        }
    }
}
impl<'b, T> Iterator for VSIter<'b, T> where T: Ord + 'b {
    type Item = &'b T;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            VSIter::NonAdjacent( vi) => {
                if let Some(val) = vi.next() {
                    Some(*val)
                } else {
                    None
                }
            },
            VSIter::Adjacent( si) => si.next(),
        }
    }
}
#[derive(Debug)]
pub enum VSIterMut<'b, T> where T: Ord + 'b + Debug {
    NonAdjacent( std::slice::IterMut<'b, &'b mut T> ),
    Adjacent( std::slice::IterMut<'b, T> ),
}
impl<'b, T> VSIterMut<'b, T> where T: Ord + 'b + Debug {
    pub fn new(vs: &'b mut VirtualSlice<'b, T>) -> VSIterMut<'b, T> {
        match vs {
            NonAdjacent(v) => VSIterMut::NonAdjacent(v.iter_mut()),
            Adjacent(s) => VSIterMut::Adjacent(s.iter_mut()),
        }
    }
}
impl<'b, T> Iterator for VSIterMut<'b, T>
    where T: Ord + 'b + Debug {
    type Item = &'b mut T;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            VSIterMut::NonAdjacent( vi) => {
                if let Some(val) = vi.next() {
                    Some(*val)
                } else {
                    None
                }
            },
            VSIterMut::Adjacent( si) => si.next(),
        }
    }
}

impl<T> Default for VirtualSlice<'_, T> where T: Ord + Debug {
    fn default() -> Self {
        VirtualSlice::new(50)
    }
}
impl<T> Debug for VirtualSlice<'_, T> where T : Ord + Debug {

    /// extract and display the slice subsegments attached to the virtualslice
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entries(
                // returns a VSIter that serves &'a T
                self.iter()
            )
            .finish()
    }
}
impl<T> Index<usize> for VirtualSlice<'_, T> where T: Ord + Debug {
    type Output = T;

    /// Index implementation so that VirtualSlice[x] will return a &T to the underlying slice segment
    fn index(&self, index: usize) -> &Self::Output {
        unsafe {
            match self {
                // syntactic overkill as rust will automatically dereference the chain of references
                // but it feels good to be explicit!!
                NonAdjacent(vv) => &(*vv.as_ptr().add(index)),
                Adjacent(s) => &*s.as_ptr().add(index),
            }
        }
    }
}
impl<T> IndexMut<usize> for VirtualSlice<'_, T> where T: Ord + Debug {

    /// Index implementation so that VirtualSlice[x] will return a &mut T to the underlying slice segment
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        // syntactic overkill as rust will automatically dereference the chain of references
        // but it feels good to be explicit!!
        unsafe {
            match self {
                NonAdjacent(vv) => *vv.as_mut_ptr().add(index) as &mut T,
                Adjacent(s) => &mut *s.as_mut_ptr().add(index) as &mut T,
            }
        }
    }
}
impl<'a, T> Index<Range<usize>> for VirtualSlice<'a, T> where T: Ord + Debug {
    type Output = [&'a mut T];

    fn index(&self, index: Range<usize>) -> &Self::Output {
        if let NonAdjacent(vv) = self {
            &vv[index]
        } else {
            panic!()
        }
    }
}
impl<'a, T> IndexMut<Range<usize>> for VirtualSlice<'a, T> where T: Ord + Debug {
    fn index_mut(&mut self, index: Range<usize>) -> &mut Self::Output {
        if let NonAdjacent(vv) = self {
            &mut vv[index]
        } else {
            panic!()
        }
    }
}
impl<'a, T> PartialOrd for VirtualSlice<'a, T> where T: Ord + Debug {
    /// Enable VirtualSlice comparison so we can write things like
    /// ```
    /// use csx3::merge::vs::VirtualSlice;
    /// let s = &mut [1,2,3,4,5];
    /// let vs = VirtualSlice::new_adjacent(s);
    /// assert_eq!( vs, VirtualSlice::Adjacent( &mut [1,2,3,4,5] ) );
    /// ```
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (NonAdjacent(v), NonAdjacent(o)) => v.partial_cmp(o),
            (Adjacent(s), Adjacent(o)) => s.partial_cmp(o),
            ( _, _ ) => panic!(),
        }
    }
}

impl<'a, T> PartialEq<Self> for VirtualSlice<'a, T> where T: Ord  + Debug {
    /// Enable VirtualSlice comparison so we can write things like
    /// ```
    /// use csx3::merge::vs::VirtualSlice;
    /// let s = &mut [1,2,3,4,5];
    /// let vs = VirtualSlice::new_adjacent(s);
    /// assert_eq!( vs, VirtualSlice::Adjacent( &mut [1,2,3,4,5] ) );
    /// ```
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
        let _s = &mut [0, 0, 0, 0];
        let s2 = &mut [2, 4, 6, 8, 10];
        let mut vs = VirtualSlice::new_adjacent(s1);
        vs.attach(s2);
    }

    #[test]
    fn test_virtual_slice_iter_mut_adjacent() {
        let mut input = vec![1, 3, 5, 7, 9, 2, 4, 6, 8, 10];
        let (s1, s2) = input.split_at_mut(5);

        let mut vs = VirtualSlice::new_adjacent(s1);
        vs.attach(s2);
        println!("{:?}", vs);
        assert_eq!(vs, Adjacent(&mut [1, 3, 5, 7, 9, 2, 4, 6, 8, 10]));
        vs.iter_mut()
            .for_each(|x| {
                *x = 12;
            });
        assert_eq!(s1, &mut [12, 12, 12, 12, 12]);
        assert_eq!(s2, &mut [12, 12, 12, 12, 12]);
    }

    #[test]
    fn test_virtual_slice_index_adjacent() {
        let mut input = vec![1, 3, 5, 7, 9, 2, 4, 6, 8, 10];
        let (s1, s2) = input.split_at_mut(5);

        let mut vs = VirtualSlice::new_adjacent(s1);
        vs.attach(s2);
        vs[0] = 11;
        vs[5] = 9;
        println!("{:?}", vs);

        assert_eq!(vs, Adjacent(&mut [11, 3, 5, 7, 9, 9, 4, 6, 8, 10]));
        assert_eq!(s1, &mut [11, 3, 5, 7, 9]);
        assert_eq!(s2, &mut [9, 4, 6, 8, 10]);
    }
    #[test]
    fn test_virtual_slice_swap_shallow() {
        let s1 = &mut [1, 3, 5, 7, 9];
        let s2 = &mut [2, 4, 6, 8, 10];

        let mut vs = VirtualSlice::new(s1.len()+s2.len());
        vs.attach(s1);
        vs.attach(s2);
        vs[0] = 11;
        vs[5] = 22;
        println!("{:?}", vs);
        assert_eq!(vs, NonAdjacent(
            vec![&mut 11, &mut 3, &mut 5, &mut 7, &mut 9, &mut 22, &mut 4, &mut 6, &mut 8, &mut 10]
        ));

        vs.swap_shallow(0, 5);
        // references have been swapped
        assert_eq!(vs, NonAdjacent(
            vec![&mut 22, &mut 3, &mut 5, &mut 7, &mut 9, &mut 11, &mut 4, &mut 6, &mut 8, &mut 10]
        ));
        // however segments haven't been affected
        assert_eq!(s1, &mut [11, 3, 5, 7, 9]);
        assert_eq!(s2, &mut [22, 4, 6, 8, 10]);
    }
    #[test]
    fn test_virtual_slice_merge() {
        let test_data: [(&mut [i32], &mut [i32], &[i32], &[i32]); 8] = [
            (&mut [-88, -29, 4, 84], &mut [-127, -113, -71, -54],
             &[-127, -113, -88, -71], &[-54, -29, 4, 84]),
            (&mut [5, 6, 7], &mut [1, 2, 3, 4],
             &[1, 2, 3], &[4, 5, 6, 7]),
            (&mut [-127, -81, -55, -38, 40, 78, 122, 124], &mut [-126, -123, -102, -78, -51, -44, -29, 17],
             &[-127, -126, -123, -102, -81, -78, -55, -51], &[-44, -38, -29, 17, 40, 78, 122, 124]),
            (&mut [-69, -18, -8, 3, 38, 68, 69, 74], &mut [-119, -83, -81, -76, -37, -13, 40, 77],
             &[-119, -83, -81, -76, -69, -37, -18, -13], &[-8, 3, 38, 40, 68, 69, 74, 77]),
            (&mut [-106, -82, -64, -57, 5, 23, 67, 79], &mut [-103, -85, -85, -49, -42, -38, -37, 86],
             &[-106, -103, -85, -85, -82, -64, -57, -49], &[-42, -38, -37, 5, 23, 67, 79, 86]),
            (&mut [-122, -19, 3, 51, 69, 77, 78, 115], &mut [-118, -99, 23, 23, 35, 59, 63, 75],
             &[-122, -118, -99, -19, 3, 23, 23, 35], &[51, 59, 63, 69, 75, 77, 78, 115]),
            (&mut [-79, -39, -5, 69, 87, 117, 118, 126], &mut [-59, -39, -39, -14, 40, 86, 97, 113],
             &[-79, -59, -39, -39, -39, -14, -5, 40], &[69, 86, 87, 97, 113, 117, 118, 126]),
            (&mut [-108, -84, 6, 49, 74, 96, 100, 112], &mut [-118, -91, -86, -81, -43, 16, 45, 52],
             &[-118, -108, -91, -86, -84, -81, -43, 6], &[16, 45, 49, 52, 74, 96, 100, 112])
        ];

        for (s1, s2, c1, c2) in test_data {
            let mut vs = VirtualSlice::new(s1.len()+s2.len());
            vs.merge(s1);
            vs.merge(s2);
            assert_eq!(s1, c1);
            assert_eq!(s2, c2);
        }
    }

    #[test]
    fn test_virtual_slice_merge_multiple()
    {
        let s1 = &mut [5, 6, 7];
        let _x = &[0, 0, 0, 0, 0, 0]; // wedge to break adjacency
        let s2 = &mut [1, 2, 3, 4];
        let _y = &[0, 0, 0, 0, 0, 0]; // wedge to break adjacency
        let s3 = &mut [10, 12, 14];
        let _z = &[0, 0, 0, 0, 0, 0]; // wedge to break adjacency
        let s4 = &mut [8, 9, 15, 16];

        let mut vs = VirtualSlice::new(s1.len()+s2.len()+s3.len()+s4.len());
        vs.merge(s1);
        vs.merge(s2);
        vs.merge(s3);
        vs.merge(s4);

        assert_eq!(s1, &mut [1, 2, 3]);
        assert_eq!(s2, &mut [4, 5, 6, 7]);
        assert_eq!(s3, &mut [8, 9, 10]);
        assert_eq!(s4, &mut [12, 14, 15, 16]);
    }
    #[test]
    fn test_virtual_slice_new_iter_swap() {
        let s1 = &mut [1, 3, 5, 7, 9];
        let _s3 = &mut [0, 0, 0];
        let s2 = &mut [2, 4, 6, 8 , 10];

        {
            let mut v = VirtualSlice::new(s1.len()+s2.len());
            v.attach(s1);
            v.attach(s2);

            v.iter_mut()
                .for_each(|ptr| {
                    *ptr = 12;
                });
        }
        {
            let mut v = VirtualSlice::new(s1.len()+s2.len());
            v.attach(s1);
            v.attach(s2);
            v[0] = 11;
            v[5] = 9;
        }
        assert_eq!(s1, &mut [11, 12, 12, 12, 12]);
        assert_eq!(s2, &mut [9, 12, 12, 12, 12]);
        {
            let mut v = VirtualSlice::new(s1.len()+s2.len());
            v.attach(s1);
            v.attach(s2);
            v.swap(0, 5);
        }
        assert_eq!(s1, &mut [9, 12, 12, 12, 12]);
        assert_eq!(s2, &mut [11, 12, 12, 12, 12]);
    }
}