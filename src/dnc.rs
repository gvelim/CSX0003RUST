use std::fmt::Debug;
use std::iter::Peekable;
use std::cmp::Ordering;
use std::ops::Range;


/// Takes two iterators as input with each iteration returning
/// the next in order item out of the two, plus its inversions' count
/// ```
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
struct MergeIterator<I: Iterator> {
    right: Peekable<I>,
    left: Peekable<I>,
    left_count: u32,
    left_len: u32,
}
impl<I: Iterator> MergeIterator<I> {
    /// Constructs a new MergeIterator given two iterators
    fn new(left: I, right: I) -> Self {
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
          I::Item: Ord, {
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

/// Sort function based on the merge sort algorithm
/// returning a sorted vector plus the count of inversions
pub fn merge_sort<T>(v: &[T]) -> (u32, Vec<T>)
    where T: Copy + Clone + Ord + Debug {

    let len = v.len();

    println!("Input: ({}){:?} =>", len, v);
    match len {
        // unity slice, just return it
        0..=1 => (0, v.to_vec()),
        // sort the binary slice and exit
        // use a local variable to eliminate the need for &mut as input
        // and given we output a new vector
        2 => {
            let mut sorted_vec = Vec::from(v);
            let mut inv = 0;
            if sorted_vec[0] > sorted_vec[1] {
                sorted_vec.swap(0, 1);
                inv += 1;
            }
            (inv, sorted_vec)
        },
        // if slice length longer than 2 then split recursively
        _ => {
            let (left, right) = v.split_at(len >> 1);
            let (left_inv, left) = merge_sort(left);
            let (right_inv, right) = merge_sort(right);

            // return a vector of the merged but ordered slices
            // plus inversions vector; inversion count per position
            let (merge_vec, sorted_vec): (Vec<u32>, Vec<T>) = MergeIterator::new(left.iter(),right.iter()).unzip();

            println!("Inversion Vector: {:?}", &merge_vec);

            // sum up the inversion count vector
            let merge_inv: u32 = merge_vec.into_iter().filter(|x| *x > 0).sum();

            println!("Merged: {}:{:?} <> {}:{:?} => {}:{:?}", left_inv, left, right_inv, right, left_inv + right_inv + merge_inv, sorted_vec);
            (left_inv + right_inv + merge_inv, sorted_vec)
        }
    }
}

fn partition_at_index<T>(v: &mut [T], idx: usize) -> usize
    where T: Copy + Clone + Ord + Debug  {

    let mut i = 1usize;
    v.swap(0, idx);
    for j in 1..(v.len()) {
        if v[0] > v[j] {
            v.swap(i,j);
            i+=1;
            print!("s:");
        } else {
            print!("-:");
        }
        println!("{:?}, ({},{})", v, i,j);
    }
    v.swap(0,i-1);
    println!("f:{:?}, ({})", v, i);
    (i-1)
}

pub fn quick_sort<T>(v: &mut [T])
    where T: Copy + Clone + Ord + Debug {

    let idx= partition_at_index(v, 3);

    //quick_sort(&mut v[..left_partition.len()]);
    //quick_sort(&mut v[left_partition.len() + 1..]);
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_partition_at_index() {
        let mut v = vec![6,12,5,9,7,8,11,3,1,4,2,10];
        let idx = partition_at_index(&mut v[..], 4);

        // [2, 5, 6, 3, 1, 4],7,[9, 12, 8, 11, 10]
        // idx = 6 (7th position)
        assert_eq!(v[..idx], [2,5,6,3,1,4]);
        assert_eq!(v[idx], 7);
        assert_eq!(v[idx+1..v.len()], [9,12,8,11,10]);
    }
    #[test]
    fn test_sort() {
        let v1 = vec![3,2,1];
        let v2 = vec![4,1,3,2];
        let v3 = vec![8, 4, 2, 1];
        let v4 = vec![6,2,4,3,5,1];
        let v5 = vec![7,6,5,4,3,2,1];
        let v6 = vec![8,7,6,5,4,3,2,1];
        assert_eq!(merge_sort(&v1), (3, vec![1,2,3]));
        assert_eq!(merge_sort(&v2), (4, vec![1,2,3,4]));
        assert_eq!(merge_sort(&v3), (6, vec![1,2,4,8]));
        assert_eq!(merge_sort(&v4), (10, vec![1,2,3,4,5,6]));
        assert_eq!(merge_sort(&v5), (21, vec![1,2,3,4,5,6,7]));
        assert_eq!(merge_sort(&v6), (28, vec![1,2,3,4,5,6,7,8]));
    }
    #[test]
    fn test_merge() {
        let s1 = &[2, 4, 6];
        let s2 = &[1, 3, 5];

        let mut iter = MergeIterator::new(s1.iter(), s2.iter());

        assert_eq!(iter.next(), Some( (3,&1) ));
        assert_eq!(iter.next(), Some( (0,&2) ));
        assert_eq!(iter.next(), Some( (2,&3) ));
        assert_eq!(iter.next(), Some( (0,&4) ));
        assert_eq!(iter.next(), Some( (1,&5) ));
        assert_eq!(iter.next(), Some( (0,&6) ));
        assert_eq!(iter.next(), None);
    }
}
