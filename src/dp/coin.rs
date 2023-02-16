use std::collections::{HashMap, HashSet};
use std::time::SystemTime;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_coins() {
        let data = [
            (vec![1, 2, 3, 5], 19, 5)
            , (vec![1, 2, 3, 5], 7, 2)
            , (vec![1, 2, 3], 11, 4)
            , (vec![1, 3, 5], 16, 4)
            , (vec![1, 2, 3, 5, 10, 50], 139, 8)
        ];

        for (set, sum, res) in data {
            let mut coins = Coins::default();
            let mut start = SystemTime::now();
            let n = coins.recursive(sum, &set);
            println!("Recursive: {:?}, {:?}", n,
                     SystemTime::now().duration_since(start)
            );
            assert_eq!(n, res);

            start = SystemTime::now();
            let coins = Coins::iterative(sum, &set);
            println!("Iterative: {sum} = {:?}, {:?}",
                     (coins.get_coins_num(), coins.get_coins()),
                     SystemTime::now().duration_since(start)
            );
            assert_eq!(coins.get_coins_num(), res);
        }
    }

    #[test]
    fn test_combinations() {
        let (set, sum, res) = (vec![1, 3, 5], 8, 5);

        let mut coins = Coins::default();
        coins.combinations(sum, &set);

        println!("Combinations: {}", coins.combos.as_ref().unwrap().len());
        coins.combos.as_ref().unwrap()
            .iter()
            .enumerate()
            .for_each(|(i, combo)| println!("{:2}. {sum} = {:?}", i + 1, combo));

        assert_eq!(res, coins.combos.as_ref().unwrap().len());
    }
}

struct Coins {
    map: Option<HashMap<usize,usize>>,
    dp: Option<Vec<usize>>,
    coins: Option<Vec<usize>>,
    combos: Option<HashSet<Vec<usize>>>
}

impl Default for Coins {
    fn default() -> Self {
        Coins { map: Some(HashMap::new()), dp: None, coins: Some(vec![]), combos: Some(HashSet::new()) }
    }
}

impl Coins {
    fn combinations(&mut self, sum: usize, coins:&[usize]) -> usize {
        if sum == 0 {
            let mut sol = self.coins.as_ref().unwrap().clone();
            sol.sort();
            self.combos.as_mut().unwrap().insert(sol);
            return  1
        }
        let calc = coins
            .iter()
            .filter(|&c| sum >= *c )
            .map(|&c| {
                self.coins.as_mut().unwrap().push(c);
                let sum = self.combinations(sum - c, coins);
                self.coins.as_mut().unwrap().pop();
                sum
            })
            .sum();
        calc
    }

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
            coins: Some(cs),
            combos: None
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
        if sum == 0 { return 0 }
        else {
            if let Some(&best) =
                self.map
                    .as_ref()
                    .expect("recursive(): HashMap not initialised. Use with Coins::default()")
                    .get(&sum) { return best }
        }

        let best = coins.iter()
            .filter(|&c| sum >= *c )
            .map(|&c| self.recursive(sum-c, coins ) + 1 )
            .min()
            .unwrap();

       self.map
            .as_mut()
            .expect("recursive(): HashMap not initialised. Use with Coins::default()")
            .insert(sum,best);
        best
    }
}
