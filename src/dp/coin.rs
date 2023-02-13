use std::cmp::min;
use std::collections::HashMap;
use std::time::SystemTime;

#[test]
fn test_sum_of_coins() {
    let set = &[1,2,3,5,10,50];
    let sum = 197;
    let mut coins = Coins::default();

    let mut start = SystemTime::now();
    println!("Recursive: {:?}, {:?}",
             coins.recursive(sum, set),
             SystemTime::now().duration_since(start)
    );
    start = SystemTime::now();
    println!("Iterative: {:?}, {:?}",
             Coins::iterative(sum, set),
             SystemTime::now().duration_since(start)
    );
}

struct Coins {
    map: HashMap<usize,usize>
}
impl Default for Coins {
    fn default() -> Self {
        Coins { map: HashMap::new() }
    }
}
impl Coins {
    fn iterative(sum: usize, coins:&[usize]) -> usize {
        let mut dp = vec![0;sum+1];
        dp[0] = 0;
        (1..dp.len())
            .for_each(|sum| {
                dp[sum] = coins
                    .iter()
                    .filter(|&c| sum >= *c)
                    .map(|&c| dp[sum - c] + 1 )
                    .min()
                    .unwrap()
            });
        println!("{:?}",dp);
        *dp.last().unwrap()
    }
    fn recursive(&mut self, sum: usize, coins:&[usize]) -> usize {
        if sum == 0 {
            return 0;
        }
        if let Some(&best) = self.map.get(&sum) {
            return best;
        }
        let mut best = usize::MAX;
        for &c in coins {
            if sum < c { continue }
            best = min(best, self.recursive(sum-c, coins ) + 1 )
        }
        // println!("={:?}",(sum,best));
        self.map.insert(sum,best);
        best
    }
}
