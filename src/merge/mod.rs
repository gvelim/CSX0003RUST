pub mod vs;

use std::cmp::Ordering;
use std::iter::Peekable;
use vs::VirtualSlice;
use std::fmt::Debug;
use std::slice::from_raw_parts_mut;

/// Takes two iterators as input with each iteration returning
/// the next in order item out of the two, plus its inversions' count
/// ```
/// use csx3::merge::MergeIterator;
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
    left_count: u32,
    left_len: u32,
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
        mi.left_len = mi.left.size_hint().0 as u32;
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
            // slices within bounds; right > left
            (Some(l),
                Some(r)) if l.cmp(r) == Ordering::Greater => {
                    let inv = self.left_len - self.left_count;
                    Some( (inv as usize, self.right.next().unwrap()) )
                },
            // slices within bounds; right <= left
            (Some(_), Some(_)) => {
                self.left_count += 1;
                Some((0, self.left.next().unwrap()))
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

/// Merge capabilities for generic type arrays
pub trait Merge<T> where T: Ord + Debug {
    fn merge_lazy<'a>(&'a mut self, s:&'a mut[T]) -> (usize, VirtualSlice<T>);
    fn merge_mut_adjacent(&mut self, s:&mut[T]) -> usize;
    fn merge_mut(&mut self, s:&mut[T]) -> usize;
}

impl<T> Merge<T> for [T]
    where T: Ord + Debug  {
    /// Returns a merged representation (virtual slice) that attaches onto `self` with another slice without mutating their contents
    /// The virtual slice can then be used for further ordered operations across the attached slices,
    /// Using its `superimpose_state()` method you can mutate the order back to the attached slices.
    /// ```
    /// use csx3::merge::Merge;
    ///
    /// let s1 = &mut [5,6,7];
    /// let s2 = &mut [1,2,3,4];
    ///
    /// let (_inversions, mut mask) = s1.merge_lazy(s2);    // mask mutably borrows s1 & s2
    ///
    /// mask.iter()                                         // iterate over merged contents
    ///     .enumerate()                                    // while s1 and s2 are unaffected
    ///     .for_each(|(i,x)| assert_eq!(*x,i+1) );
    ///
    /// assert_eq!(s1, &[5,6,7]);
    /// assert_eq!(s2, &[1,2,3,4]);
    /// ```
    fn merge_lazy<'a>(&'a mut self, s: &'a mut [T]) -> (usize, VirtualSlice<T>) {
        let mut vs = VirtualSlice::new(self.len()+s.len());
        vs.attach(self);
        (vs.merge_lazy(s), vs)
    }
    /// Applies memory efficient in-place merging when two slices are adjacent to each other.
    /// ```
    /// use csx3::merge::Merge;
    ///
    /// let mut input = vec![1, 3, 5, 7, 9, 2, 4, 6, 8, 10];
    /// let (s1,s2) = input.split_at_mut(5);
    ///
    /// s1.merge_mut_adjacent(s2);
    /// assert_eq!(input, vec![1,2,3,4,5,6,7,8,9,10]);
    /// ```
    /// Panics in case the two slices are found not to be adjacent. For safety, always use *ONLY* against slices that have been mutable split from an existing slice
    /// #[should_panic]
    /// let s1 = &mut [3, 5, 7];
    /// let s2 = &mut [1, 3, 5];   // wedge this between the two
    /// let s3 = &mut [2, 4, 6];
    ///
    /// s1.merge_mut_adjacent(s3); // this should throw a panic
    ///
    /// There is no warranty that Rust will maintain two slice adjacent in a case like this.
    /// let s1 = &mut [3, 5, 7];
    /// let s3 = &mut [2, 4, 6];
    ///
    /// s1.merge_mut_adjacent(s3); // this may not always work
    ///
    fn merge_mut_adjacent(&mut self, s:&mut[T]) -> usize {
        //let mut ws = VirtualSlice::new_adjacent(self);
        //ws.merge(s)

        merge_mut_fast(self, s)
    }

    /// Merge two non-adjacent slices using in-place memory swaps and without use of rotations
    /// ```
    /// use csx3::merge::Merge;
    ///
    /// let s1 = &mut [5,6,7];
    /// let _s = &[0,0,0,0,0,0]; // wedge to break adjacency
    /// let s2 = &mut [1,2,3,4];
    ///
    /// let inv = s1.merge_mut(s2);
    ///
    /// assert_eq!(s1, &[1,2,3]);
    /// assert_eq!(s2, &[4,5,6,7]);
    /// ```
    fn merge_mut(&mut self, s:&mut[T]) -> usize {
        let mut ws = VirtualSlice::new(self.len()+s.len());
        ws.attach(self);
        ws.merge(s)
    }
}
// ANCHOR: merge_adjacent
/// Applies memory efficient in-place merging when two slices are adjacent to each other.
/// ```
/// use csx3::merge::merge_mut_fast;
///
/// let mut input = vec![1, 3, 5, 7, 9, 2, 4, 6, 8, 10];
/// let (s1,s2) = input.split_at_mut(5);
///
/// merge_mut_fast(s1,s2);
/// assert_eq!(input, vec![1,2,3,4,5,6,7,8,9,10]);
/// ```
/// Panics in case the two slices are found not to be adjacent. For safety, always use *ONLY* against slices that have been mutable split from an existing slice
pub fn merge_mut_fast<T>(s1: &mut [T], s2: &mut [T]) -> usize where T: Ord + Debug {
    let ws: &mut [T];

    unsafe {
        ws = from_raw_parts_mut(s1.as_mut_ptr(), s1.len() + s2.len());
        assert!(s2[0] == ws[s1.len()]);
    }

    let (mut p, mut c, mut j, llen, mut inversion ) = (0usize, 0usize, s1.len(), s1.len(), 0usize);
    let mut idx_rfl: Vec<usize> = Vec::from_iter(0..ws.len());
    let len = idx_rfl.len();

    //println!("{ws:?}::{idx_rfl:?}, ({i},{c},{j})");

    unsafe {
        let idxp = idx_rfl.as_mut_ptr();
        let wsp = ws.as_mut_ptr();

        let mut cc; // c' definition
        let mut pp; // p' definition

        loop {
            cc = *idxp.add(c);
            pp = *idxp.add(p);
            match (j < len && j != p, p < len-1 && c < llen -1) {
                (true, _) if (*wsp.add(cc)).cmp(&(*wsp.add(j))) == Ordering::Greater => {
                    inversion += j - p;
                    wsp.add(p).swap( wsp.add(j));
                    //idx_rfl.swap(ii, j);
                    idxp.add(pp).write(j);
                    idxp.add(j).write(pp);
                    j += 1;
                },
                (_, true) => {
                    wsp.add(p).swap( wsp.add(cc));
                    //idx_rfl.swap(pp, c);
                    idxp.add(cc).write(pp);
                    idxp.add(pp).write( cc);
                    c += 1;
                },
                (_,_) => break,
            };
            p += 1;
            //println!("{ws:?}::{idx_rfl:?}, ({i},{c},{j})");
        }
    }
    //println!("Merge Done!");
    inversion
}
// ANCHOR_END: merge_adjacent
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_merge_shallow() {
        let s1 = &mut [1, 3, 5, 7, 9];
        let s2 = &mut [2, 4, 6, 8, 10];

        let (_,vs) = s1.merge_lazy(s2);

        assert_eq!( vs, VirtualSlice::NonAdjacent(
            vec![&mut 1, &mut 2, &mut 3, &mut 4, &mut 5, &mut 6, &mut 7, &mut 8, &mut 9,&mut 10]
        ));
        vs.iter()
            .enumerate()
            .for_each(|(i,x)| assert_eq!(*x,i+1) );

        assert_eq!( s1, &mut [1, 3, 5, 7, 9] );
        assert_eq!( s2, &mut [2, 4, 6, 8, 10] );
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
                s1.merge_mut_adjacent(s2);
                assert_eq!(input, output);
            })
    }
    #[test]
    fn test_merge_mut_adjacent_fast() {
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
                merge_mut_fast(s1,s2);
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
        s1.merge_mut_adjacent(s3);
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
                s1.merge_mut(s2);
                assert_eq!(input, output);
            })
    }
}