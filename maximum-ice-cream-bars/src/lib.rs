// https://leetcode.com/problems/maximum-ice-cream-bars/

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn max_ice_cream(mut costs: Vec<i32>, mut coins: i32) -> i32 {
        costs.sort_unstable();

        let mut num_bought = 0;

        while num_bought < costs.len() && coins >= costs[num_bought] {
            coins -= costs[num_bought];
            num_bought += 1;
        }

        num_bought as i32
    }
}

#[test]
fn do_test() {
    assert_eq!(Solution::max_ice_cream(vec![1, 3, 2, 4, 1], 7), 4);
    assert_eq!(Solution::max_ice_cream(vec![10, 6, 8, 7, 7, 8], 5), 0);
    assert_eq!(Solution::max_ice_cream(vec![1, 6, 3, 1, 2, 5], 20), 6);
    assert_eq!(Solution::max_ice_cream(vec![4, 6, 3, 1, 2, 5], 20), 5);
}
