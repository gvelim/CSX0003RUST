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

struct WIS<'a> {
    set: &'a [usize],
    dp: Vec<usize>
}

impl WIS<'_> {
    fn new(set: &[usize]) -> WIS {
        let mut dp = vec![0; set.len()+1];
        dp[0] = 0;
        dp[1] = set[0];

        (2..dp.len())
            .all(|i| {
                dp[i] = max(dp[i - 1], dp[i - 2] + set[i - 1]);
                println!("{:?}", (i, set[i - 1], &set, &dp));
                true
            });

        WIS { set, dp }
    }

    fn weight(&self) -> usize {
        *self.dp.last().unwrap()
    }
    fn positions_in_set(&self) -> Vec<bool> {
        let mut positions = vec![false; self.dp.len() - 1];
        let mut i = self.dp.len() - 1;
        while i > 0 {
            if self.dp[i - 2] == self.dp[i] - self.set[i - 1] {
                println!("{:?}", (i, self.dp[i - 2], self.dp[i], self.set[i - 1]));
                positions[i - 1] = true;
                i -= 1;
            }
            i -= 1;
            if i == 1 {
                println!("{:?}", (i, self.dp[i], self.set[i - 1]));
                positions[i - 1] = true;
                i -= 1;
            }
        }
        positions
    }
    fn to_binary_string(&self) -> String {
        self.positions_in_set().iter()
            .filter_map(|&d|
                if d { Some('1') } else { Some('0') }
            )
            .collect::<String>()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_wis() {
        let data: Vec<(Vec<usize>,usize)> = vec![
            (vec![1,4,5,4],8)
            ,(vec![10, 280, 618, 762, 908, 409, 34, 59, 277, 246, 779],2626)
            ,(vec![10, 460, 250, 730, 63, 379, 638, 122, 435, 705, 84],2533)
            ,(parse_graph("src/dp/txt/input_random_1_10.txt"), 281)
            ,(parse_graph("src/dp/txt/input_random_10_40.txt"), 19639)
            // ,(parse_graph("src/dp/txt/input_random_30_1000.txt"), 288082919)
        ];

        for (g,res) in data {
            let wis = WIS::new(&g);
            let n = wis.weight();
            println!("Weight Independent set: {:?},{:?}\n", n, wis.to_binary_string());
            assert_eq!(n,res);
        }
    }

    #[test]
    fn test_parse() {
        println!("{:?}", parse_graph("src/dp/txt/input_random_1_10.txt"));
    }
}