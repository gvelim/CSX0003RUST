use std::fmt::Debug;

// ANCHOR: sort_count
/// Sorts a given array using the Count Sort algorithm.
/// Input array NuType shouldn't exceed u16 to avoid memory issues
/// ```
/// use csx3::sort::count::CountSort;
///
/// let v = &mut [3i8,5,8,1,2,4,6,0];
///
/// v.count_sort();
/// assert_eq!(v, &[0,1,2,3,4,5,6,8]);
/// ```
pub trait CountSort {

    fn count_sort(&mut self);
}

// CountSort macro implementation for singed and unsigned types
impl<T> CountSort for [T]
    where T: Distance<T> + Copy + Ord + Debug{

    fn count_sort(&mut self) {

        if self.len() < 2 {
            return;
        }
        // find min and max elements
        // so we can construct the boundaries of the counting array
        // i.e. if (min,max) = (13232, 13233) then we need only an array with capacity(2)
        let (min, max) = min_max(self);

        // construct a counting array with length = Max - Min + 1
        let len: usize = max.dist_from(min);
        // initialise it with zero counts
        let mut count = vec![0usize; len + 1];
        // and finally measure counts per item
        self.iter()
            .for_each(|x| {
                // construct index offset based on Min value, such as, Min is at [0] position
                let idx: usize = x.dist_from(min);
                count[idx] += 1;
            });

        // play back onto the input slice the counts collected with Sum of all counts == slice.len()
        let iter = &mut self.iter_mut();
        count.into_iter()
            .enumerate()
            .filter(|(_, x)| *x > 0)
            .for_each(|(i, x)| {
                // place value at `x` positions
                iter.take(x)
                    // translate index -> value
                    // given value = Min + index
                    .for_each(|n| { *n = min.add_index(i ) });
            });
    }
}
// ANCHOR: sort_count_diff
/// Distance calculation between two types that are either both signed or unsigned
/// Returns the distance as unsigned type
pub trait Distance<T> {
    fn dist_from(&self, min: T) -> usize;
    fn add_index(&self, idx: usize) -> T;
}

/// Macro implementation of Distance trait for all signed types
macro_rules! impl_dist_signed {
    ( $($x:ty),*) => {
        $( impl Distance<$x> for $x {
            #[inline]
            fn dist_from(&self, min: $x) -> usize {
                if *self > min {
                    (*self as usize).wrapping_sub(min as usize)
                } else {
                    (min as usize).wrapping_sub(*self as usize)
                }
            }
            #[inline]
            fn add_index(&self, idx: usize) -> $x { self.wrapping_add(idx as $x) }
        } )*
    }
}
impl_dist_signed!(i8,i16,i32,isize,u8,u16,u32,usize);
// ANCHOR_END: sort_count_diff
// ANCHOR_END: sort_count

#[inline]
fn min_max<T>(s: &[T]) -> (T, T) where T: Copy + Ord {
    let (mut min, mut max) = (s[0], s[0]);
    s.iter()
        .skip(1)
        .for_each(|x| {
            if *x > max { max = *x; } else if *x < min { min = *x; }
        });
    (min, max)
}


#[cfg(test)]
mod test {
    use crate::random_sequence;
    use crate::{sort::merge::*,merge::Merge};
    use super::*;

    #[test]
    fn test_countsort_head_to_head()
    {
        let runs = 255usize;
        let size = random_sequence::<u8, Vec::<u8>>(runs);
        for i in 0..runs {
            let mut v1: Vec<i8> = random_sequence( size[i].into() );
            let mut v2 = v1.clone();

            v2.as_mut_slice().count_sort();
            let _ = v1.mergesort_mut(Merge::merge_mut_adjacent);
            assert_eq!( &v1, &v2 );
        }
    }
    #[test]
    fn test_count_sort() {
        let test_data: [(&mut [isize], &[isize]);6] = [
            (&mut [13,12,11],              &[11,12,13]),
            (&mut [14,11,13,12],           &[11,12,13,14]),
            (&mut [28, 24, 22, 21],        &[21,22,24,28]),
            (&mut [36,32,34,33,35,31],     &[31,32,33,34,35,36]),
            (&mut [7,6,5,4,3,2,1],         &[1,2,3,4,5,6,7]),
            (&mut [113, 82, 127, 58, 16, -128, -58, -110],   &[-128, -110, -58, 16, 58, 82, 113, 127])
        ];

        test_data.into_iter()
            .for_each( | (input, output) | {
                input.count_sort();
                assert_eq!( input, output);
            });
    }
}
