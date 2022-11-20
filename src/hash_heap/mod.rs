use std::collections::{HashMap};

/// Finds ALL pairs that have their sum equal to the target value
pub fn two_sum_all(nums: Vec<i32>, target: i32) -> Vec<(i32,i32)> {
    let map : HashMap<_,_> = nums.as_slice().iter().zip(0..).collect();
    nums.iter()
        .map(|b| target-b)
        .zip(0..)
        .filter_map(|(a,n)|
            map.get(&a).and_then(|&idx| if idx != n { Some((n,idx)) } else {None}  )
        )
        .fold(vec![], |mut out, pair| {
            out.push(pair);
            out
        })
}

/// Returns ONLY the first pair that sums up to the target value
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let map : HashMap<_,_> = nums.iter().zip(0..).collect();
    let s = nums.iter()
        .map(|b| target-b)
        .zip(0..)
        .filter_map(|(a,n)|
            map.get(&a).and_then(|&idx| if idx != n { Some((n,idx)) } else {None} )
        )
        .next()
        .unwrap_or_else(||panic!("No two sum results to {target}"));
    vec![s.0, s.1]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2_sum() {
        let data = vec![
            (vec![2,7,11,15],9,vec![0,1]),
            (vec![3,2,4],6,vec![1,2]),
            (vec![3,3],6,vec![0,1]),
        ];

        for (nums, target, res) in data {
            assert_eq!(two_sum(nums, target), res)
        }
    }

    #[test]
    fn test_2_sum_all() {
        let data = vec![
            (vec![1,6,2,8,7,11,15], 9, vec![(0,3), (2,4), (3,0), (4,2)]),
            (vec![3,2,4], 6, vec![(1,2),(2,1)]),
            (vec![3,3], 6, vec![(0,1)]),
        ];

        for (nums, target, res) in data {
            assert_eq!(two_sum_all(nums, target), res)
        }
    }
}