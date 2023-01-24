use std::str::FromStr;

fn parse_graph(filename: &str) -> Vec<usize> {
    let input = std::fs::read_to_string(filename).unwrap_or_else(|e| panic!("{e}"));

    let mut lines = input.lines();
    let mut n = lines.next().and_then(|num| {
        Some(usize::from_str(num).unwrap_or_else(|e| panic!("{e}")))
    }).unwrap();
    let mut g = Vec::with_capacity(n);
    while n > 0 {
        let val = lines.next().and_then(|num| {
            Some(usize::from_str(num).unwrap_or_else(|e| panic!("{e}")))
        }).unwrap();
        g.push(val);
        n -= 1;
    }
    g
}

fn wis(g:&[usize]) -> Vec<usize> {
    vec![]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_wis() {
        let g = parse_graph("src/dp/txt/input_random_1_10.txt");

        println!("Weight Independent set {:?}", wis(&g))
    }

    #[test]
    fn test_parse() {
        println!("{:?}", parse_graph("src/dp/txt/input_random_1_10.txt"));
    }
}