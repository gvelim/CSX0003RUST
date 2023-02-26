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
            println!("===================\nSum: {}\nPath: {:?}",ss.sum(),ss.path().collect::<Vec<_>>());
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
    fn path(&self) -> impl Iterator<Item=usize> + '_ {
        SquareSumIter::new(self)
    }
    fn sum(&self) -> usize {
        *self.dp.last().unwrap().last().unwrap()
    }
}

struct SquareSumIter<'a> {
    ss: &'a SquareSum<'a>,
    sum: usize,
    pos: (usize,usize)
}

impl<'a> SquareSumIter<'a> {
    fn new(ss:&'a SquareSum<'a>) -> SquareSumIter<'a> {
        SquareSumIter { ss, sum:ss.sum(), pos: (ss.dp[0].len()-1, ss.dp.len()-1) }
    }
}

impl Iterator for SquareSumIter<'_> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.sum > 0 {
            let ss = self.ss;
            let (x,  y) = self.pos;        // get current x,y position
            let square = ss.path[y-1][x-1];      // extract square value at (x,y)
            self.sum = ss.dp[y][x] - square;           // previous position = sum(x,y) - square value
            if self.sum == ss.dp[y-1][x] {             // does it match square above ?
                self.pos.1 -= 1;                       // yes -> then go one up
            } else {
                self.pos.0 -= 1;                       // no -> then go one left
            }
            Some(square)
        } else {
            None
        }
    }
}


impl Debug for SquareSum<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let piter = self.path.iter();
        self.dp.iter()
            .skip(1)
            .zip(piter)
            .all(|(dp,path)| {
                dp.iter()
                    .skip(1)
                    .zip(path)
                    .all(|p|
                        write!(f, "{:2}/{:2} ", p.1, p.0).is_ok()
                    )
                    .then_some( writeln!(f) )
                    .is_some()
            });
        Ok(())
    }
}

