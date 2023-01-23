// Problem description: https://leetcode.com/problems/find-the-duplicate-number/

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn find_duplicate(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();

        for i in 0..nums.len() - 1 {
            if nums[i] == nums[i + 1] {
                return nums[i];
            }
        }

        -1
    }
}

#[test]
fn do_test() {
    assert_eq!(Solution::find_duplicate(vec![1, 3, 4, 2, 2]), 2);
    assert_eq!(Solution::find_duplicate(vec![3, 1, 3, 4, 2]), 3);
}
