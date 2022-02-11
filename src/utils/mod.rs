///
///
///

use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::ops::{Index, IndexMut, Range};

/// Constructing a VirtualSlice allowing us to operate over
/// multiple non-adjacent slice segments as a "continuous slice"
/// ```
/// use csx3::utils::VirtualSlice;
///
/// let v = &mut [1, 3, 5, 7, 9, 2, 4, 6, 8 , 10];
/// let (s1, s2) = v.split_at_mut(5);
/// let _s3 = &mut [0, 0, 0, 0,0];   // Wedge this to break stack continuity
/// let s4 = &mut [2, 4, 6, 8, 10];
///
/// {
///     let mut v = VirtualSlice::new();
///     v.attach(s1);
///     v.attach(s4);
///     v[0] = 11;
///     v[5] = 9;
///     v.swap(0, 5);
/// }
/// assert_eq!(s1, &mut [9, 3, 5, 7, 9]);
/// assert_eq!(s4, &mut [11, 4, 6, 8 , 10]);
/// {
///     let mut v = VirtualSlice::new_adjacent(s1);
///     v.attach(s2);
///     v.swap(0, 5);
/// }
/// assert_eq!(s1, &mut [2, 3, 5, 7, 9]);
/// assert_eq!(s2, &mut [9, 4, 6, 8 , 10]);
///
/// ```
pub enum VirtualSlice<'a, T> where T: Ord {
    /// The tuple holds a vector of mutable references and the Index Reflector
    NonAdjacent( Vec<&'a mut T>, Option<Vec<usize>>),
    /// Holds a mutable reference to the reconstructed parent slice out of two memory adjacent slices
    Adjacent( &'a mut[T] ),
}


use VirtualSlice::{NonAdjacent, Adjacent};

impl<'a, T> VirtualSlice<'a, T> where T: Ord {
    /// Create a new VirtualSlice for use with non-adjacent slice segments
    pub fn new() -> VirtualSlice<'a, T> {
        NonAdjacent( Vec::new(), None )
    }
    /// Create a new VirtualSlice for use with adjacent slice segments
    pub fn new_adjacent(s: &'a mut[T]) -> VirtualSlice<'a, T> {
        Adjacent( s )
    }
    /// Current length of the VirtualSlice is equal to sum of all attached slice segments
    pub fn len(&self) -> usize {
        match self {
            NonAdjacent(v,_) => v.len(),
            Adjacent(s) => s.len(),
        }
    }
    pub fn is_empty(&self) -> bool {
        match self {
            NonAdjacent(v,_) => v.is_empty(),
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
            NonAdjacent(v, _) => {
                s.iter_mut()
                    .for_each(|item| {
                        v.push(item);
                    });
            }
            Adjacent(s0) => {
                let fs: &mut [T];
                unsafe {
                    fs = &mut *std::ptr::slice_from_raw_parts_mut::<T>((*s0).as_mut_ptr(), s0.len() + s.len());
                    // checking they are aligned and adjacent,
                    // if not panic! so we prevent unpredictable behaviour
                    assert!(&s[0] == &fs[s0.len()]);
                }
                *self = VirtualSlice::new_adjacent(fs);
            }
        }
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
    /// Perform a deep merge by ordering the referred values hence mutating the slice segments
    pub fn merge(&mut self, s: &'a mut [T]) -> usize
        where T: Ord + Debug {
        // we are not interested in the index reflector here so we don't store it
        let (inversion, _) = self._merge(s, VirtualSlice::swap);
        inversion
    }
    /// Shallow swap; swaps the references of the underlying slice segments. The segments aren't affected
    /// Operates only with non-adjacent slices
    pub fn swap_shallow(&mut self, a: usize, b:usize) {
        if let NonAdjacent(v, _) = self {
            v.swap(a, b);
        } else {
            panic!("Not applicable for Adjacent VirtualSlices; use with VirtualSlice::new() instead");
        }
    }
    /// Perform a shallow merge by ordering the VirtualSlice's references and not the referred values.
    /// The VirtualSlice can be used as sort-mask layer above the slice segments, which later can be superimposed over
    /// In case of non-adjacent slices only.
    pub fn merge_shallow(&mut self, s: &'a mut [T]) -> usize
        where T: Ord + Debug {
        let (inversions, idx_rfl) = self._merge(s, VirtualSlice::swap_shallow);

        match self {
            Adjacent(_) => panic!("merge_shallow(): cannot operate in adjacent mode"),
            NonAdjacent(_, None) => panic!("merge_shallow(): unexpected! _merge() didn't return a index reflector vector"),
            NonAdjacent(_, idx_reflector) => {
                // we need to store index reflector in case we want to mutate the attached slices via the impose method
                *idx_reflector = idx_rfl;
                inversions
            }
        }
    }

    /// Superimposes O(n-1) the derived order onto the attached slice segments.
    /// The stored Index Reflector contains the order per reference
    pub fn superimpose_shallow_merge(&mut self) {
        // total operations must be len()-1 as we use 1 position as temp swap location
        let total_swaps = self.len() - 2;
        // Count total number of swaps occurred
        let mut swap_count = 0usize;
        // holds the current temp swap position
        let mut temp_idx = 0usize;

        // make sure entry conditions are correct
        // prefer to panic as non of those scenarios should be recoverable
        // otherwise, extract internal data and proceed with algorithm
        match self {
            Adjacent(_) => panic!("superimpose_shallow_merge(): call doesn't work over adjacent slice segments"),
            NonAdjacent(_, None) => panic!("superimpose_shallow_merge(): Index Reflector does not exist. Did merge_shallow() run ?"),
            NonAdjacent(vs, Some(idx)) => {

                // Exit conditions are either,
                // - total swaps == total number of elements - 1 OR
                // - current tmp index position has reached the end of VirtualSlice (Case: virtualslice already ordered; zero swaps)
                while swap_count < total_swaps && temp_idx < total_swaps
                {
                    let mut i;
                    // Exit condition
                    // - current swap index == correct ordered position, (item is positioned where it should be)
                    while temp_idx != idx[temp_idx] {
                        i = idx[temp_idx];
                        idx.swap(temp_idx, i);
                        unsafe {
                            // we need to overcome Rust's borrow checking
                            std::ptr::swap::<T>(&mut *vs[temp_idx] as *mut T, &mut *vs[i] as *mut T);
                        }
                        swap_count += 1;
                    }
                    temp_idx += 1;
                }
            }
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
    fn _merge<F>(&mut self, s: &'a mut [T], mut f_swap: F) -> (usize, Option<Vec<usize>>)
        where T: Ord + Debug,
              F: FnMut(&mut Self, usize, usize) {

        if self.is_empty() {
            self.attach(s);
            return (0, None)
        }

        // j = s2[j] equivalent position within the working slice (j') and index reflector (j)
        let mut j = self.len();

        self.attach(s);

        // i = partition position in working slice so that ... [merged elements] < ws[i] < [unmerged elements]
        // p = index reflector partition bound where i's position is always upper bounded by p
        // c = s1[c] equivalent position in the index reflector, so that idx_rfl[c] == c' == s1[c] equivalent in ws[c'],
        // used for optimising finding i pos in index array
        let (mut inv_count, mut c, mut i, p) = (0usize, 0usize, 0usize, j);

        // ws_len = working slice's length
        let ws_len = self.len();

        // Build the index reflector of size [ 0 .. size of left slice] since the following properties apply
        // - c & i' will never exceed size of left slice
        // - j == j' always be the same position
        let mut idx_rfl = (0..ws_len).into_iter().collect::<Vec<usize>>();

        //println!("-:Merge:{:?} :: {:?} ({:?},{:?},{:?})", self, idx_rfl, i, j, c);

        // Phase 1 : Conditions
        // j == v.len() => no more comparisons since ws[j] is the rightmost, last and largest of the two slices
        // i == j => no more comparison required, since everything in ws[..i] << ws[j]
        while j < ws_len && i != j {
            match ( self[idx_rfl[c]] ).cmp( &self[j] ) {
                Ordering::Less | Ordering::Equal => {

                    // swap left slice's item in the working slice with merged partition edge ws[i]
                    // swap( ws[i] with ws[c'] where c' = index_reflector[c]
                    f_swap(self, i, idx_rfl[c] );

                    // swap index_reflect[c] with index_reflector[i']
                    // i' == index_reflector[x]; where x == i;
                    // e.g. i = 3rd pos, hence i' = index_reflector[x] where x == 3;
                    let idx = idx_rfl[c..p].iter().position(|x| *x == i).unwrap() + c;
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
                    // swap( ws[i] with ws[j'] where j' = index_reflector[j], but j' == j so
                    f_swap(self, i, j);

                    // swap index_reflect[j] with index_reflector[i']
                    // i' == index_reflector[x]; where x == i;
                    // e.g. i = 3rd pos, hence i' = index_reflector[x] where x == 3;
                    let idx = idx_rfl[c..p].iter().position(|x| *x == i).unwrap() + c;
                    // swap( i' with j )
                    idx_rfl.swap(idx, j);
                    // or since always j == j' we just copy the value over no need to swap
                    //idx_rfl[idx] = j;
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
        let c_bound = p-1;
        // or if we optimise with idx_len() = s1.len() then
        //let c_bound = idx_rfl.len()-1;
        let i_bound = ws_len-1;
        while i < i_bound && c < c_bound {

            // condition saves cpu-cycles from zero-impact operations when i == c' (no swap)
            // otherwise it has no algorithmic impact
            if i != idx_rfl[c] {
                // swap i with c' in working slice
                f_swap(self, i, idx_rfl[c]);

                // extract i' from index_reflector[]
                let idx = idx_rfl[c..p].iter().position(|x| *x == i).unwrap() + c;

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
        (inv_count, Some(idx_rfl))
    }
}

pub enum VSIter<'b, T> where T: Ord + 'b {
    NonAdjacent( std::slice::Iter<'b, &'b mut T> ),
    Adjacent( std::slice::Iter<'b, T> ),
}
impl<'b, T> VSIter<'b, T> where T: Ord + 'b{
    pub fn new(vs: &'b VirtualSlice<'b, T>) -> VSIter<'b, T> {
        match vs {
            NonAdjacent(v, _) => VSIter::NonAdjacent(v.iter()),
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
pub enum VSIterMut<'b, T> where T: Ord + 'b {
    NonAdjacent( std::slice::IterMut<'b, &'b mut T> ),
    Adjacent( std::slice::IterMut<'b, T> ),
}
impl<'b, T> VSIterMut<'b, T> where T: Ord + 'b {
    pub fn new(vs: &'b mut VirtualSlice<'b, T>) -> VSIterMut<'b, T> {
        match vs {
            NonAdjacent(v, _) => VSIterMut::NonAdjacent(v.iter_mut()),
            Adjacent(s) => VSIterMut::Adjacent(s.iter_mut()),
        }
    }
}
impl<'b, T> Iterator for VSIterMut<'b, T>
    where T: Ord + 'b {
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

impl<T> Default for VirtualSlice<'_, T> where T: Ord {
    fn default() -> Self {
        VirtualSlice::new()
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
impl<T> Index<usize> for VirtualSlice<'_, T> where T: Ord {
    type Output = T;

    /// Index implementation so that VirtualSlice[x] will return a &T to the underlying slice segment
    fn index(&self, index: usize) -> &Self::Output {
        match self {
            // syntactic overkill as rust will automatically dereference the chain of references
            // but it feels good to be explicit!!
            NonAdjacent(vv, _) => &(*vv[index]),
            Adjacent(s) => &s[index],
        }
    }
}
impl<T> IndexMut<usize> for VirtualSlice<'_, T> where T: Ord {

    /// Index implementation so that VirtualSlice[x] will return a &mut T to the underlying slice segment
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        // syntactic overkill as rust will automatically dereference the chain of references
        // but it feels good to be explicit!!
        match self {
            NonAdjacent(vv, _) => &mut (*vv[index]),
            Adjacent(s) => &mut s[index],
        }
    }
}
impl<'a, T> Index<Range<usize>> for VirtualSlice<'a, T> where T: Ord {
    type Output = [&'a mut T];

    fn index(&self, index: Range<usize>) -> &Self::Output {
        if let NonAdjacent(vv, _) = self {
            &vv[index]
        } else {
            panic!()
        }
    }
}
impl<'a, T> IndexMut<Range<usize>> for VirtualSlice<'a, T> where T: Ord {
    fn index_mut(&mut self, index: Range<usize>) -> &mut Self::Output {
        if let NonAdjacent(vv, _) = self {
            &mut vv[index]
        } else {
            panic!()
        }
    }
}
impl<'a, T> PartialOrd for VirtualSlice<'a, T> where T: Ord {
    /// Enable VirtualSlice comparison so we can write things like
    /// ```
    /// use csx3::utils::VirtualSlice;
    /// let s = &mut [1,2,3,4,5];
    /// let vs = VirtualSlice::new_adjacent(s);
    /// assert_eq!( vs, VirtualSlice::Adjacent( &mut [1,2,3,4,5] ) );
    /// ```
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (NonAdjacent(v, _), NonAdjacent(o, _)) => v.partial_cmp(o),
            (Adjacent(s), Adjacent(o)) => s.partial_cmp(o),
            ( _, _ ) => panic!(),
        }
    }
}

impl<'a, T> PartialEq<Self> for VirtualSlice<'a, T> where T: Ord  {
    /// Enable VirtualSlice comparison so we can write things like
    /// ```
    /// use csx3::utils::VirtualSlice;
    /// let s = &mut [1,2,3,4,5];
    /// let vs = VirtualSlice::new_adjacent(s);
    /// assert_eq!( vs, VirtualSlice::Adjacent( &mut [1,2,3,4,5] ) );
    /// ```
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (NonAdjacent(v, _), NonAdjacent(o,_)) => v.eq(o),
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
    fn test_virtual_slice_impose_negative_1() {
        let s1 = &mut [5, 6, 7];
        let mut vs = VirtualSlice::new();
        vs.attach(s1);
        vs.superimpose_shallow_merge();      // there is no index_reflector yet, should do nothing
    }
    #[test]
    #[should_panic]
    fn test_virtual_slice_impose_negative_2() {
        let [(s1, s2)]: [(&mut [i32], &mut [i32]); 1] = [(&mut [5, 6, 7], &mut [1, 2, 3, 4])];
        let mut vs = VirtualSlice::new();

        vs.attach(s1);
        vs.merge(s2);                // deep merge creates a reflector
        vs.superimpose_shallow_merge(); // it should do nothing as the vs is already ordered
    }
    #[test]
    fn test_virtual_slice_impose() {
        let data: [(&mut[i32], &mut[i32], &[i32],&[i32]); 3] = [
            (&mut [-88,-29,4,84], &mut [-127,-113,-71,-54], &[-127, -113, -88, -71], &[-54, -29, 4, 84]),
            (&mut [5,6,7], &mut[1,2,3,4], &[1,2,3], &[4,5,6,7]),
            (&mut [1,2,3,4], &mut[5,6,7], &[1,2,3,4], &[5,6,7]),
        ];

        for (s1,s2, o1, o2) in data {
            let mut vs = VirtualSlice::new();
            vs.attach(s1);
            vs.merge_shallow(s2);
            vs.superimpose_shallow_merge();
            assert_eq!(s1, o1);
            assert_eq!(s2, o2);
        }
    }
    #[test]
    #[should_panic]
    fn test_virtual_slice_adjacent_panic() {
        let s1 = &mut [1, 3, 5, 7, 9];
        let _s = &mut [0,0,0,0];
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
        assert_eq!( s1, &mut [12, 12, 12, 12, 12] );
        assert_eq!( s2, &mut [12, 12, 12, 12, 12] );
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
        assert_eq!( s1, &mut [11, 3, 5, 7, 9] );
        assert_eq!( s2, &mut [9, 4, 6, 8, 10] );
    }
    #[test]
    fn test_virtual_slice_swap_shallow() {
        let s1 = &mut [1, 3, 5, 7, 9];
        let s2 = &mut [2, 4, 6, 8 , 10];

        let mut vs = VirtualSlice::new();
        vs.attach(s1);
        vs.attach(s2);
        vs[0] = 11;
        vs[5] = 22;
        println!("{:?}", vs);
        assert_eq!(vs, NonAdjacent(
            vec![&mut 11, &mut 3, &mut 5, &mut 7, &mut 9, &mut 22, &mut 4, &mut 6, &mut 8, &mut 10],
            None
        ));

        vs.swap_shallow(0,5);
        // references have been swapped
        assert_eq!(vs, NonAdjacent(
            vec![&mut 22, &mut 3, &mut 5, &mut 7, &mut 9, &mut 11, &mut 4, &mut 6, &mut 8, &mut 10],
            None
        ));
        // however segments haven't been affected
        assert_eq!( s1, &mut [11, 3, 5, 7, 9] );
        assert_eq!( s2, &mut [22, 4, 6, 8, 10] );
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

        let mut vs = VirtualSlice::new();
        vs.merge(s1);
        vs.merge(s2);
        vs.merge(s3);
        vs.merge(s4);

        assert_eq!(s1, &mut [1,2,3]);
        assert_eq!(s2, &mut [4,5,6,7]);
        assert_eq!(s3, &mut [8,9,10]);
        assert_eq!(s4, &mut [12,14,15,16]);
    }
    #[test]
    fn test_virtual_slice_merge_shallow() {
        let s1 = &mut [1, 3, 5, 7, 9];
        let s2 = &mut [2, 4, 6, 8, 10];

        let mut vs = VirtualSlice::new();
        vs.merge_shallow(s1);
        vs.merge_shallow(s2);

        assert_eq!( vs, NonAdjacent(
            vec![&mut 1, &mut 2, &mut 3, &mut 4, &mut 5, &mut 6, &mut 7, &mut 8, &mut 9,&mut 10],
            None
        ));
        vs.iter()
            .enumerate()
            .for_each(|(i,x)| assert_eq!(*x,i+1) );

        assert_eq!( s1, &mut [1, 3, 5, 7, 9] );
        assert_eq!( s2, &mut [2, 4, 6, 8, 10] );
    }
    #[test]
    fn test_virtual_slice_new_iter_swap() {
        let s1 = &mut [1, 3, 5, 7, 9];
        let _s3 = &mut [0, 0, 0];
        let s2 = &mut [2, 4, 6, 8 , 10];

        {
            let mut v = VirtualSlice::new();
            v.attach(s1);
            v.attach(s2);

            v.iter_mut()
                .for_each(|ptr| {
                    *ptr = 12;
                });
        }
        {
            let mut v = VirtualSlice::new();
            v.attach(s1);
            v.attach(s2);
            v[0] = 11;
            v[5] = 9;
        }
        assert_eq!(s1, &mut [11, 12, 12, 12, 12]);
        assert_eq!(s2, &mut [9, 12, 12, 12, 12]);
        {
            let mut v = VirtualSlice::new();
            v.attach(s1);
            v.attach(s2);
            v.swap(0, 5);
        }
        assert_eq!(s1, &mut [9, 12, 12, 12, 12]);
        assert_eq!(s2, &mut [11, 12, 12, 12, 12]);
    }
}