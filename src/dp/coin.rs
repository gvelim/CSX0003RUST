use std::cmp::min;
use std::collections::HashMap;
use std::time::SystemTime;

#[test]
fn test_sum_of_coins() {
    let set = &[1,2,3,5];
    let sum = 19;
    let mut coins = Coins::default();

    let mut start = SystemTime::now();
    println!("Recursive: {:?}, {:?}",
             coins.recursive(sum, set),
             SystemTime::now().duration_since(start)
    );
    start = SystemTime::now();
    let coins = Coins::iterative(sum, set);
    println!("Iterative: {sum} = {:?}, {:?}",
             (coins.get_coins_num(),coins.get_coins()),
             SystemTime::now().duration_since(start)
    );
}

struct Coins {
    map: Option<HashMap<usize,usize>>,
    dp: Option<Vec<usize>>,
    coins: Option<Vec<usize>>
}
impl Default for Coins {
    fn default() -> Self {
        Coins { map: Some(HashMap::new()), dp: None, coins: None }
    }
}
impl Coins {
    fn iterative(sum: usize, coins:&[usize]) -> Coins {
        let mut dp = vec![0;sum+1];
        let mut cs= vec![0;sum+1];
        (1..dp.len())
            .for_each(|sum| {
                (cs[sum], dp[sum]) = coins
                    .iter()
                    .filter(|&c| sum >= *c)
                    .map(|&c| (c,dp[sum - c] + 1) )
                    .min_by_key(|t| t.1 )
                    .unwrap();
            });
        Coins {
            map: None,
            dp: Some(dp),
            coins: Some(cs)
        }
    }
    fn get_coins_num(&self) -> usize {
        if let Some(dp) = &self.dp {
            *dp.last().unwrap()
        } else { 0 }
    }
    fn get_coins(&self) -> Vec<usize> {
        let mut output = vec![];
        if let Some(coins) = &self.coins {
            let mut pos = coins.len() - 1;
            while pos > 0 {
                output.push(coins[pos]);
                pos -= coins[pos];
            }
        }
        output
    }
    fn recursive(&mut self, sum: usize, coins:&[usize]) -> usize {
        if sum == 0 {
            return 0;
        } else {
            let Some(map) = &self.map else { unimplemented!() };
            if let Some(&best) = map.get(&sum) {
                return best;
            }
        }

        let mut best = usize::MAX;
        for &c in coins {
            if sum < c { continue }
            best = min(best, self.recursive(sum-c, coins ) + 1 )
        }
        // println!("={:?}",(sum,best));
        let Some(map) = &mut self.map else { unimplemented!() };
        map.insert(sum,best);
        best
    }
}
