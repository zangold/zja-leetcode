// Problem description: https://leetcode.com/problems/coin-change/

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut dp = vec![-1i32; amount as usize + 1];

        dp[0] = 0;

        for i in 0..dp.len() {
            let mut best = i32::MAX;
            for c in coins.iter().copied() {
                let c = c as usize;
                if i >= c && dp[i - c] != -1 {
                    best = best.min(dp[i - c] + 1);
                }
            }

            if best != i32::MAX {
                dp[i] = best;
            }
        }

        dp[amount as usize] as i32
    }
}

#[test]
fn do_test() {
    assert_eq!(Solution::coin_change(vec![1, 2, 5], 11), 3);
    assert_eq!(Solution::coin_change(vec![2], 3), -1);
    assert_eq!(Solution::coin_change(vec![1], 0), 0);
}
