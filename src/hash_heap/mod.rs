use std::collections::{HashMap};
use std::ops::Sub;

pub fn two_sum_all(nums: Vec<i32>, target: i32) -> Vec<(usize,usize)> {
    let map : HashMap<_,_> = nums.as_slice().iter().enumerate().map(|(n,&a)|(a,n)).collect();
    nums.iter()
        .map(|b| target.sub(b))
        .enumerate()
        .filter_map(|(n,a)| match map.get(&a)  {
            Some(&idx) if idx != n => Some((n,idx)),
            _ => None,
        })
        .fold(vec![], |mut out, pair| {
            out.push(pair);
            out
        })
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let s = two_sum_all(nums,target)[0];
    vec![s.0 as i32,s.1 as i32]
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
            (vec![1,6,2,8,7,11,15],9,vec![(0,3), (2,4), (3,0), (4,2)]),
            (vec![3,2,4],6,vec![(1,2),(2,1)]),
            (vec![3,3],6,vec![(0,1)]),
        ];

        for (nums, target, res) in data {
            assert_eq!(two_sum_all(nums, target), res)
        }
    }
}