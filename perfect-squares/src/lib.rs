// Problem description: https://leetcode.com/problems/perfect-squares/

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![i32::MAX; n + 1];

        dp[0] = 0;

        for i in 0..dp.len() {
            let mut j = 1;

            while i + j * j < dp.len() {
                dp[i + j * j] = dp[i + j * j].min(dp[i] + 1);

                j += 1;
            }
        }

        dp[n]
    }
}

#[test]
fn do_test() {
    assert_eq!(Solution::num_squares(12), 3);
    assert_eq!(Solution::num_squares(13), 2);
}
