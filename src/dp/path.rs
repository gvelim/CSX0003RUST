use std::cmp::max;


fn max_squares_sum(path: &[Vec<usize>]) -> usize {
    let mut dp = vec![ vec![0; path[0].len()+1]; path.len()+1];

    (1..dp.len())
        .for_each(|y|
            (1..dp[0].len())
                .for_each(|x|
                    dp[y][x] = max (dp[y-1][x],dp[y][x-1] ) + path[y-1][x-1]
                )
    );
    dp.iter().for_each(|v| println!("{:2?}",v) );
    *dp.last().unwrap().last().unwrap()
}

#[test]
fn test_max_sum_of_squares() {
    let data = vec![
        (vec![vec![3,7,9,2,7], vec![9,8,3,5,5], vec![1,7,9,8,5], vec![3,8,6,4,10], vec![6,3,9,7,8]], 67),
        (vec![vec![2,3,4],vec![1,5,4],vec![6,1,2]], 16)
    ];
    for (path,res) in data {
        assert_eq!(max_squares_sum(&path), res);
    }
}