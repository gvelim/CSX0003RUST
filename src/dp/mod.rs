use std::cmp::max;
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

fn weight_independent_set(set: &[usize]) -> (usize,Vec<bool>) {
    let mut solution = vec![0; set.len()+1];
    solution[0] = 0;
    solution[1] = set[0];

    (2..solution.len())
        .all(|i| {
            solution[i] = max(solution[i-1], solution[i-2]+set[i-1]);
            println!("{:?}",(i,set[i-1],&set,&solution));
            true
        });
    (*solution.last().unwrap(), extract(&solution, set))
}

fn extract(solution: &[usize], set: &[usize]) -> Vec<bool> {
    let mut positions = vec![false;solution.len()-1];
    let mut i = solution.len()-1;
    while i > 0 {
        if solution[i-2] == solution[i]-set[i-1] {
            println!("{:?}",(i,solution[i-2],solution[i],set[i-1]));
            positions[i-1] = true;
            i -= 1;
        }
        i -= 1;
        if i == 1 {
            println!("{:?}",(i,solution[i],set[i-1]));
            positions[i-1] = true;
            i -= 1;
        }
    }
    positions
}

fn wis_to_string(v: &[bool]) -> String {
    v.iter()
        .filter_map(|&d|
            if d { Some('1') } else { Some('0') }
        )
        .collect::<String>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_wis() {
        let data: Vec<(Vec<usize>,usize)> = vec![
            (vec![1,4,5,4],8),
            (vec![10, 280, 618, 762, 908, 409, 34, 59, 277, 246, 779],2626),
            (vec![10, 460, 250, 730, 63, 379, 638, 122, 435, 705, 84],2533),
            (parse_graph("src/dp/txt/input_random_1_10.txt"), 281)
        ];

        for (g,res) in data {
            let set = weight_independent_set(&g);
            println!("Weight Independent set {:?},{:?}\n\n", set.0, wis_to_string(&set.1));
            assert_eq!(set.0,res);
        }
    }

    #[test]
    fn test_parse() {
        println!("{:?}", parse_graph("src/dp/txt/input_random_1_10.txt"));
    }
}