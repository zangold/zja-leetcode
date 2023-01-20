// Problem description: https://leetcode.com/problems/rotate-array/

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let k = k as usize % nums.len();

        let tmp = &nums[nums.len() - k..].to_vec();

        for i in (0..nums.len() - k).rev() {
            nums[i + k] = nums[i];
        }

        nums[..k].clone_from_slice(&tmp[..k]);
    }
}

#[test]
fn do_test() {
    let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
    Solution::rotate(&mut nums, 3);
    assert_eq!(nums, vec![5, 6, 7, 1, 2, 3, 4]);

    let mut nums = vec![-1, -100, 3, 99];
    Solution::rotate(&mut nums, 2);
    assert_eq!(nums, vec![3, 99, -1, -100]);
}
