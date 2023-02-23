use std::cmp::max;
use std::fmt::{Debug, Formatter};
use std::num::ParseIntError;
use std::str::FromStr;

struct Object {
    value: usize,
    weight: usize
}
impl Debug for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"(v:{},w:{})",self.value,self.weight)
    }
}

struct KnapSack<'a> {
    list: &'a [Object],
    capacity: usize,
    dp: Vec<Vec<usize>>
}

impl KnapSack<'_> {
    fn new(list: &[Object], capacity: usize ) -> KnapSack {
        let mut dp = vec![ vec![ 0; capacity+1 ]; list.len()+1 ];
        (1..dp.len())
            .for_each(|i| {
                for w in 1..=capacity {
                    let item = &list[i-1];
                    dp[i][w] =
                        if w < item.weight {
                            dp[i-1][w]
                        } else {
                            max(dp[i-1][w], dp[i-1][w-item.weight] + item.value )
                        }
                }
            });

        KnapSack { list, capacity, dp }
    }
    fn positions(&self) -> impl Iterator<Item=bool> + '_ {
        let mut w = self.capacity;
        (0..self.list.len())
            .rev()
            .map(move |i|{
                if self.dp[i+1][w] != self.dp[i][w] {
                    w -= self.list[i].weight;
                    true
                } else {
                    false
                }
            })
    }
    fn elements(&self) -> impl Iterator<Item=&'_ Object> {
        self.positions()
            .zip(self.list.iter().rev())
            .filter_map(|(i,v)| if i {Some(v)} else {None})
    }
    fn max_value(&self) -> usize {
        *self.dp.last().unwrap().last().unwrap()
    }
    fn parse(input: &str) -> Result<(usize,Vec<Object>),ParseIntError> {

        fn parse_line(line: &str) -> Result<(usize, usize),ParseIntError> {
            let mut parts = line.split(' ');
            Ok((
                parts.next().and_then(|n| Some(usize::from_str(n)) ).unwrap()?,
                parts.next().and_then(|n| Some(usize::from_str(n)) ).unwrap()?
            ))
        }

        let mut lines = input.lines();
        let (knapsack, items) = parse_line( lines.next().unwrap() )?;

        let mut sack = Vec::with_capacity(items);
        while let Some(line) = lines.next() {
            let (value, weight) = parse_line( line )?;
            sack.push( Object{ value, weight } );
        }
        Ok((knapsack,sack))
    }
}

impl Debug for KnapSack<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let v = (0..self.capacity+1).collect::<Vec<usize>>();
        writeln!(f," Cap :: {:2?}", v)?;
        for (i,item) in self.dp.iter().enumerate().rev() {
            write!(f,"{:2}th :: {:2?}", i, item)?;
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_knapsack() {
        let data = vec![
            (std::fs::read_to_string("src/dp/txt/input_random_1_4_4.txt").unwrap_or_else(|e| panic!("{}",e)), 4)
            ,(std::fs::read_to_string("src/dp/txt/input_random_5_10_10.txt").unwrap_or_else(|e| panic!("{}",e)), 14)
            ,(std::fs::read_to_string("src/dp/txt/input_random_14_100_100.txt").unwrap_or_else(|e| panic!("{}",e)), 478)
            ,(std::fs::read_to_string("src/dp/txt/input_random_24_1000_100.txt").unwrap_or_else(|e| panic!("{}",e)), 6475)
        ];

        for (inp,res) in data {
            let (capacity, list) = KnapSack::parse(inp.as_str()).unwrap_or_else(|e| panic!("{}",e));
            // list.sort_by_key(|obj| obj.weight );
            let ks = KnapSack::new( &list, capacity );
            println!("==========================");
            if list.len() < 20 {
                println!("{:?}",(capacity, &list));
                println!("{:?}",&ks);
            }
            println!("Total value: {} given capacity {}\nSelected items: {:?}\n",
                   ks.max_value(),
                   capacity,
                   ks.elements().collect::<Vec<_>>()
            );
            assert_eq!( ks.max_value(), res );
        }
    }

    #[test]
    fn test_parse() {
        let inp = std::fs::read_to_string("src/dp/txt/input_random_1_4_4.txt").unwrap_or_else(|e| panic!("{}",e));
        println!("{:?}", KnapSack::parse(inp.as_str()).unwrap_or_else(|e| panic!("{}",e)) );
        assert!(true)
    }
}
