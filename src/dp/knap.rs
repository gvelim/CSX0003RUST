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
    use super::*;

    #[test]
    fn test_knapsack() {
        let inp = std::fs::read_to_string("src/dp/txt/input_random_1_4_4.txt").unwrap_or_else(|e| panic!("{}",e));
        let (knapsack, list) = parse(inp.as_str()).unwrap_or_else(|e| panic!("{e}"));

        let dp = vec![0;list.len()+1];


    }

    #[test]
    fn test_parse() {
        let inp = std::fs::read_to_string("src/dp/txt/input_random_1_4_4.txt").unwrap_or_else(|e| panic!("{}",e));
        println!("{:?}", parse(inp.as_str()).unwrap_or_else(|e| panic!("{e}")) );
        assert!(true)
    }
}
