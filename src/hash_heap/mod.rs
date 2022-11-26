use std::collections::{HashMap};

/// Finds ALL pairs that have their sum equal to the target value
pub fn two_sum_all(nums: Vec<i64>, target: i64) -> Vec<(i64,i64)> {
    let mut map : HashMap<_,_> = HashMap::new();
    nums.iter()
        .zip(0..)
        .filter_map(|(b,n)|
            match map.get(&(target-b)) {
                Some(&i) => {
                    map.insert(b,n);
                    Some((i, n))
                }
                None => {
                    map.insert(b,n);
                    None
                }
            }
        )
        .fold(vec![], |mut out, pair| {
            out.push(pair);
            out
        })
}

/// Returns ONLY the first pair that sums up to the target value
pub fn two_sum(nums: &Vec<i64>, target: i64) -> Option<Vec<i64>> {
    let mut map : HashMap<_,_> = HashMap::new();
    match nums.iter()
        .zip(0..)
        .filter_map(|(&b,n)|
            match map.get(&(target-b)) {
                Some(&i) => Some((n, i)),
                None => {
                    map.insert(b,n);
                    None
                }
            }
        )
        .next()
    {
        Some(s) => Some(vec![s.1, s.0]),
        None => None
    }
}

#[cfg(test)]
mod test {
    use std::{ fs::File, io::{BufRead, BufReader}, str::FromStr};
    use super::*;

    fn load_file(filename: &str) -> Vec<i64> {

        let fd = File::open(filename).unwrap_or_else(|e| panic!("{e}"));
        let buf = BufReader::new(fd);

        buf.lines()
            .into_iter()
            .filter_map(|ln| ln.ok())
            .fold(vec![], |mut out, line| {
                out.push(
                    i64::from_str(line.as_str()).unwrap_or_else(|e| panic!("{e}"))
                );
                out
            })
    }

    #[test]
    fn test_2_sum_file() {
        let data = vec![
            ("src/hash_heap/txt/input_random_1_10.txt", 2)
            ,("src/hash_heap/txt/input_random_5_20.txt", 4)
            ,("src/hash_heap/txt/input_random_10_40.txt", 11)
        ];
        for (f,result) in data {
            let inp = load_file(f);
            let out = (-10000..=10000)
                .fold( vec![], |mut acc, e| {
                    two_sum(&inp,e)
                    .and_then(|pair| Some(acc.push(pair)));
                    acc
            });
            println!("Expected: {result} => Found: {}, {:?}",out.len(), out);
            assert_eq!(out.len(),result);
        }
    }

    #[test]
    fn test_2_sum() {
        let data = vec![
            (vec![2,7,11,15],9,vec![0,1])
            ,(vec![3,2,4],6,vec![1,2])
            ,(vec![3,3],6,vec![0,1])
        ];

        for (nums, target, res) in data {
            assert_eq!(two_sum(&nums, target), Some(res))
        }
    }

    #[test]
    fn test_2_sum_all() {
        let data = vec![
            (vec![1,6,2,8,1,5,3,7,11,15], 9, vec![(0, 3), (3, 4), (1, 6), (2, 7)]),
            (vec![3,2,4], 6, vec![(1,2)]),
            (vec![3,3], 6, vec![(0,1)]),
        ];

        for (nums, target, res) in data {
            assert_eq!(two_sum_all(nums, target), res)
        }
    }
}