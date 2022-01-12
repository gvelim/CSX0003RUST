use std::fmt::Debug;
use std::iter::Peekable;
use std::cmp::Ordering;
use rand::Rng;

/// Takes two iterators as input with each iteration returning
/// the next in order item out of the two, plus its inversions' count
/// ```
/// use csx3::sort::*;
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
    type Item = (u32, I::Item);

    /// Outputs the next in order value out of the two iterators
    /// in the form of Some( tuple ), where
    /// tuple = ( inversions at position, value at position)
    fn next(&mut self) -> Option<Self::Item> {
        match (self.left.peek(), self.right.peek()) {
            // left & right parts remain within their bounds
            (Some(l), Some(r)) => {
                match l.cmp(r) {
                    // left is smaller hence move to output
                    // there are no inversions to count
                    // keep count of current position
                    Ordering::Less | Ordering::Equal=> {
                        self.left_count += 1;
                        Some((0, self.left.next().unwrap()))
                    },
                    // right is smaller hence move to output
                    // inversions are equal to left items remain to iterate over
                    Ordering::Greater => {
                        let inv = self.left_len-self.left_count;
                        Some( (inv, self.right.next().unwrap()) )
                    },
                }
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

// Neve make this fn public !! as it is unsafe outsize this module/context
// we cannot pass a &mut of the array since we will violate rust's borrowing rules
// hence we have to reconstruct the array ourselves here
// GIVEN the slices are adjacent in memory
fn merge_mut<T>(s1: &mut[T], s2:&mut[T]) -> u32
    where T: Ord
{
    // println!("\tInput: {:?},{:?}", s1, s2);

    // We resort to a trick given that slices are adjacent in memory
    // we know they are, hence we reconstruct the parent slice that contains both S1 and S2
    // therefore we operate on the reconstructed slice 
    // Working Slice = (*S1) to (*S1 + s1.len + s2.len)
    let ws: &mut [T];
    unsafe {
        ws = &mut *std::ptr::slice_from_raw_parts_mut::<T>(s1.as_mut_ptr(), s1.len()+s2.len());
    }

    // i = position in working slice so that ... [sorted elements] < ws[i] < [unsorted elements]
    // j = position in working slice representing s2[0]
    // len = working slice length
    let (mut i,mut j, len, mut inv_count)  = (0usize, s1.len(), ws.len(), 0usize);

    //println!("Merge:{:?}<>{:?} ({},{})",s1, s2, i, j);


    // j == v.len() => no more comparisons since v[j] is the rightmost, last and largest of the two slices
    // i == j => no more comparison required, since everything in ws[..i] << ws[j]
    while j < len && i != j {
        if let Ordering::Greater = ws[i].cmp(&ws[j]) {
            // We deploy the rotation trick for now rather than swapping which doesn't work always
            // with rotation we get
            // ws[i],...ws[j-1],ws[j] --> ws[j],ws[i],...ws[j-1]
            // hence the "sets" remain always ordered
            ws[i..=j].rotate_right(1);
            // inversion count is equal to all item between i and j
            inv_count += j - i;
            // pick next element from upper slice since ws[j] moved left
            j += 1;
            // print!("r:");
        }
        // sorted partition (left) increased by 1,
        // pick the next element for sorting
        i += 1;
        //println!("\t{:?} ({},{})({})",ws, i, j, inv_count);
    }

    inv_count as u32
}

/// Sort function based on the merge sort algorithm
/// Sorts the mutable vector with no additional memory by applying in-place merging
/// while it returns the total count of inversions occurred
/// ```
/// use csx3::sort::merge_sort;
///
/// let input = &mut [8, 4, 2, 1];
///
/// assert_eq!( merge_sort(input), 6 );
/// assert_eq!( input, &[1,2,4,8] );
/// ```
pub fn merge_sort<T>(v: &mut [T]) -> u32
    where T: Copy + Clone + Ord {

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
                return 1
            }
            0
        },
        // if slice length longer than 2 then split recursively
        _ => {
            let (left, right) = v.split_at_mut(len >> 1);
            let left_inv = merge_sort(left);
            let right_inv = merge_sort(right);

            // // return a vector of the merged but ordered slices
            // // plus inversions vector; inversion count per position
            // let (merge_vec, _ ):( Vec<u32>, Vec<T>) = MergeIterator::new(left.iter(),right.iter()).unzip();
            // println!("\tInversion Vector: {:?}", &merge_vec);


            // // sum up the inversion count vector
            // let merge_inv: u32 = merge_vec.into_iter().filter(|x| *x > 0).sum();
            //println!("\tInversion Vector: {:?}", &merge_vec);


            let merge_inv = merge_mut(left,right);

            //println!("\tMerged: {:?}{:?} => {}", left, right, left_inv + right_inv + merge_inv);
            left_inv + right_inv + merge_inv
        }
    }
}
/// Splits an array into two mutable slices/partitions around a pivot location index
/// so that *[values in left partition] < [pivot] < [values in right partition]*
/// ```
/// use csx3::sort::*;
/// let mut v = vec![6,12,5,9,7,8,11,3,1,4,2,10];
/// let (l, idx, r) = partition_at_index(&mut v[..], 4);
///
/// // [2, 5, 6, 3, 1, 4],7,[9, 12, 8, 11, 10]
/// // idx = 6 (7th position)
/// assert_eq!(l, &[2,5,6,3,1,4]);
/// assert_eq!(idx, &7);
/// assert_eq!(r, &[9,12,8,11,10]);
/// ```
pub fn partition_at_index<T>(v: &mut [T], idx: usize) -> (&mut [T], &mut T, &mut [T])
    where T: Copy + Clone + Ord  {

    let len = v.len();
    assert!(idx < len);
    //println!("\tInput: {:?}, (@{}{})",v, idx+1, match idx {0=>"st",1=>"nd",2=>"rd",_=>"th"});

    let mut i = 0usize;

    // swap v[idx] to v[0] before entering the for loop
    v.swap(0, idx);

    // the for_each will own the &mut v anything we need within the loop
    // we'll have to get it before we get in
    let pivot = v[0];
    let ptr = v.as_mut_ptr();

    // v[0] holds the pivot point hence we start comparing from 2nd item v[1]
    // j : points to last element checked
    // i : position in array so that v[1..i] < v[i] < r[i+1..j]
    v.into_iter()
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
                        ptr.wrapping_offset(i as isize),
                        ptr.wrapping_offset(j as isize)
                    );
                }
                //print!("\ts:");
            }
            // else {
            //     print!("\t-:");
            // }
            //
            // println!("{:?},({},{})", unsafe{ &*slice_from_raw_parts::<T>(ptr, len) }, i+1, j+1);
        });
    // we found the correct order for pivot
    // hence swap v[i] with v[0]
    v.swap(0,i);
    //println!("\tf:{:?}, ({})", v, i+1);

    // split the array into [left part], [pivot + right partition]
    let (l, r) = v.split_at_mut(i);
    // split further into [pivot], [right partition]
    let (p, r) = r.split_at_mut(1);

    (&mut l[..], &mut p[0], &mut r[..])
}
/// Short a given array using the Quick Sort algorithm.
/// The function rearranges the array contents rather than returning a new sorted copy of the input array
/// ```
/// use csx3::sort::quick_sort;
///
/// let v = &mut [3,5,8,1,2,4,6,0];
///
/// quick_sort(v);
/// assert_eq!(v, &[0,1,2,3,4,5,6,8]);
/// ```
pub fn quick_sort<T>(v: &mut [T])
    where T: Copy + Clone + Ord + Debug {

    // have we reached the end of the recursion ?
    if v.len() < 2 {
        return;
    }
    // pick an index at random based on a uniform distribution
    let idx = rand::thread_rng().gen_range(0..(v.len()-1) );
    // partition the array into to mutable slices for further sorting
    let (left_partition,_ , right_partition) = partition_at_index(v, idx);

    // Recurse against left an right partitions
    quick_sort(left_partition);
    quick_sort(right_partition);
}

/// Find the nth order item within an unordered set with O(n) performance
/// using nth_min as 1 will return the smallest item; 2 the second smallest, etc
/// ```
/// use csx3::sort::rand_selection;
///
/// let v = &mut [23,43,8,22,15,11];
///
/// assert_eq!(rand_selection(v, 1), &8);
/// ```
pub fn rand_selection<T>(v: &mut [T], nth_min: usize) -> &T
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
    // println!("\t{:?} {:?}th {:?}::{}th : {}", left_partition, order, right_partition, order_nth, idx);

    // is nth order sampled over, equal or above the desired nth_min ?
    match nth_min.cmp(&order) {
        // we've found the item in nth_min order
        Ordering::Equal => nth,
        // the nth_min is below the nth found so recurse on the left partition
        Ordering::Less =>
            rand_selection(left_partition, nth_min),
        // the nth_min is above the nth found so recurse on the right partition with adjusted order
        Ordering::Greater =>
            rand_selection(right_partition, nth_min - order),
    }
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_random_selection() {
        let test_data: [(&mut [u32], usize, &u32);6] = [
            (&mut [23,43,8,22,15,11],    1, &8),
            (&mut [23,43,8,22,15,11],    2, &11),
            (&mut [23,43,8,22,15,11],    3, &15),
            (&mut [23,43,8,22,15,11],    4, &22),
            (&mut [23,43,8,22,15,11],    5, &23),
            (&mut [23,43,8,22,15,11],    6, &43),
        ];

        test_data.into_iter()
            .for_each(|(input, order, position)| {
                assert_eq!(rand_selection(input, order), position);
        })
    }
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
                quick_sort(input);
                assert_eq!(input, output);
        })
    }
    #[test]
    fn test_partition_at_index() {
        let mut v = vec![6,12,5,9,7,8,11,3,1,4,2,10];
        let (l, idx, r) = partition_at_index(&mut v[..], 4);

        // [2, 5, 6, 3, 1, 4],7,[9, 12, 8, 11, 10]
        // idx = 6 (7th position)
        assert_eq!(l, &[2,5,6,3,1,4]);
        assert_eq!(idx, &7);
        assert_eq!(r, &[9,12,8,11,10]);
    }
    #[test]
    fn test_merge_sort() {
        let test_data: [(&mut [u32], (u32, &[u32]));6] = [
            (&mut [3,2,1],              (3, &[1,2,3])),
            (&mut [4,1,3,2],            (4, &[1,2,3,4])),
            (&mut [8, 4, 2, 1],         (6, &[1,2,4,8])),
            (&mut [6,2,4,3,5,1],        (10,&[1,2,3,4,5,6])),
            (&mut [7,6,5,4,3,2,1],      (21,&[1,2,3,4,5,6,7])),
            (&mut [8,7,6,5,4,3,2,1],    (28,&[1,2,3,4,5,6,7,8]))
        ];

        test_data.into_iter()
            .for_each(|(input,(inv_count, output))| {
                assert_eq!( merge_sort(input),inv_count );
                assert_eq!( input, output );
        })
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
    fn test_merge_mut() {
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
                merge_mut(s1, s2);
                assert_eq!(input, output);
        })
    }
}
