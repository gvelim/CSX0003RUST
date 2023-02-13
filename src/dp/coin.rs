use std::cmp::min;
use std::collections::HashMap;

#[test]
fn test_sum_of_coins() {

    struct Coins {
        map: HashMap<usize,usize>
    }
    impl Default for Coins {
        fn default() -> Self {
            Coins { map: HashMap::new() }
        }
    }
    impl Coins {
        fn recursive(&mut self, sum: usize, coins:&[usize]) -> usize {
            if sum == 0 {
                return 0;
            }
            if let Some(&best) = self.map.get(&sum) {
                return best;
            }
            let mut best = usize::MAX;
            print!("{:?}",(sum));
            for &c in coins {
                if sum < c { continue }
                best = min(best, self.recursive(sum-c, coins ) + 1 )
            }
            println!("={:?}",(sum,best));
            self.map.insert(sum,best);
            best
        }
    }

    let mut coins = Coins::default();
    let x = coins.recursive(9, &[1,2,3,5,10,50]);
    println!("{x}");
}