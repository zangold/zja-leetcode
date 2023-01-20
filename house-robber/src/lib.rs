// Problem description: https://leetcode.com/problems/house-robber/

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut dp = vec![0; nums.len() + 1];

        dp[1] = nums[0];

        for i in 2..nums.len() + 1 {
            dp[i] = i32::max(dp[i - 2] + nums[i - 1], dp[i - 1]);
        }

        dp[nums.len()]
    }
}

#[test]
fn do_test() {
    assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
    assert_eq!(Solution::rob(vec![2, 7, 9, 3, 1]), 12);
    assert_eq!(Solution::rob(vec![2, 7, 9, 6, 1]), 13);
    assert_eq!(Solution::rob(vec![10, 2, 3, 10]), 20);
}
