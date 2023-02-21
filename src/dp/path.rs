use std::cmp::max;
use std::fmt::{Debug, Formatter};

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_max_sum_of_squares() {
        let data = vec![
            (vec![vec![2,3,4],vec![1,5,4],vec![6,1,2]], 16),
            (vec![vec![2,3,4,1],vec![1,4,5,4],vec![6,4,1,2],vec![3,1,3,2]], 22),
            (vec![vec![3,7,9,2,7], vec![9,8,3,5,5], vec![1,7,9,8,5], vec![3,8,6,4,10], vec![6,3,9,7,8]], 67),
        ];
        for (path,res) in data {
            let ss = SquareSum::new(&path);
            println!("===================\nSum: {}\nPath: {:?}",ss.sum(),ss.path());
            println!("(square:sum)\n{:?}",ss);
            assert_eq!(ss.sum(), res);
        }
    }
}

struct SquareSum<'a> {
    path: &'a [Vec<usize>],
    dp: Vec<Vec<usize>>
}

impl SquareSum<'_> {
    fn new(path: &[Vec<usize>]) -> SquareSum {
        let mut dp = vec![ vec![0; path[0].len()+1]; path.len()+1];
        (1..dp.len())
            .for_each(|y|
                (1..dp[0].len())
                    .for_each(|x|
                        dp[y][x] = max (dp[y-1][x],dp[y][x-1] ) + path[y-1][x-1]
                    )
            );
        SquareSum { path, dp }
    }
    fn path(&self) -> Vec<usize> {
        let mut path = Vec::new();
        let mut sum = self.sum();
        let (mut x, mut y) = (self.dp[0].len()-1, self.dp.len()-1);

        while sum > 0 {
            sum = self.dp[y][x] - self.path[y-1][x-1];
            path.push(self.path[y-1][x-1]);
            if sum == self.dp[y-1][x] {
                y -= 1;
            } else {
                x -= 1;
            }
        }
        path.reverse();
        path
    }
    fn sum(&self) -> usize {
        *self.dp.last().unwrap().last().unwrap()
    }
}

impl Debug for SquareSum<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let piter = self.path.iter();
        self.dp.iter()
            .skip(1)
            .zip(piter)
            .for_each(|(dp,path)| {
                dp.iter()
                    .skip(1)
                    .zip(path)
                    .for_each(|p|
                        write!(f, "{:2}/{:2} ", p.1, p.0).expect("")
                    );
                writeln!(f).expect("");
            });
        Ok(())
    }
}

