// https://leetcode.com/problems/maximum-subarray/

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut current_sum = 0;
        let mut best_sum = i32::MIN;

        for num in nums {
            current_sum = num.max(current_sum + num);
            best_sum = best_sum.max(current_sum);
        }

        best_sum
    }
}

#[test]
fn do_test() {
    assert_eq!(
        Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
        6
    );
    assert_eq!(Solution::max_sub_array(vec![1]), 1);
    assert_eq!(Solution::max_sub_array(vec![5, 4, -1, 7, 8]), 23);
    assert_eq!(Solution::max_sub_array(vec![-2, -1, -2]), -1);
}
