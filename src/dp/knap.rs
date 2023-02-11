use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
struct Object {
    value: usize,
    weight: usize
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

#[cfg(test)]
mod test {
    use std::cmp::max;
    use super::*;

    fn print_knapsack(ks: &[Vec<usize>]) {
        for (i,item) in ks.iter().enumerate() {
            println!("{i}th :: {:?}", item);
        }
    }

    #[test]
    fn test_knapsack() {
        let inp = std::fs::read_to_string("src/dp/txt/input_random_5_10_10.txt").unwrap_or_else(|e| panic!("{}",e));
        let (capacity, list) = parse(inp.as_str()).unwrap_or_else(|e| panic!("{e}"));

        println!("{:?}",(capacity, &list));

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
        print_knapsack(&dp);

        assert_eq!( dp[list.len()-1][capacity], 14 );
    }

    #[test]
    fn test_parse() {
        let inp = std::fs::read_to_string("src/dp/txt/input_random_1_4_4.txt").unwrap_or_else(|e| panic!("{}",e));
        println!("{:?}", parse(inp.as_str()).unwrap_or_else(|e| panic!("{e}")) );
        assert!(true)
    }
}
