// https://leetcode.com/problems/best-time-to-buy-and-sell-stock/

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let n = prices.len();

        let mut min_before = vec![0; n];
        let mut max_after = vec![0; n];

        let mut min = i32::MAX;
        for i in 0..n {
            if prices[i] < min {
                min = prices[i];
            }
            min_before[i] = min;
        }

        let mut max = i32::MIN;
        for i in (0..n).rev() {
            if prices[i] > max {
                max = prices[i];
            }
            max_after[i] = max;
        }

        let mut max_profit = 0;
        for i in 0..n {
            let profit = max_after[i] - min_before[i];
            if profit > max_profit {
                max_profit = profit;
            }
        }

        max_profit
    }
}

#[test]
fn do_test() {
    assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
    assert_eq!(Solution::max_profit(vec![7, 7, 7, 3, 6, 6]), 3);
    assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
}
