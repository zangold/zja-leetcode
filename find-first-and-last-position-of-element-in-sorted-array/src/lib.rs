// https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/

#![allow(dead_code)]

use std::ops::Range;

struct Solution;

impl Solution {
    fn left_search(nums: &[i32], target: i32, range: Range<usize>) -> i32 {
        if range.is_empty() {
            return -1;
        }

        let mid = (range.start + range.end) / 2;

        if nums[mid] == target && (mid == 0 || nums[mid - 1] != target) {
            mid as i32
        } else if nums[mid] >= target {
            Self::left_search(nums, target, range.start..mid)
        } else {
            Self::left_search(nums, target, mid + 1..range.end)
        }
    }

    fn right_search(nums: &[i32], target: i32, range: Range<usize>) -> i32 {
        if range.is_empty() {
            return -1;
        }

        let mid = (range.start + range.end) / 2;

        if nums[mid] == target && (mid + 1 == nums.len() || nums[mid + 1] != target) {
            mid as i32
        } else if nums[mid] <= target {
            Self::right_search(nums, target, mid + 1..range.end)
        } else {
            Self::right_search(nums, target, range.start..mid)
        }
    }

    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        vec![
            Self::left_search(&nums, target, 0..nums.len()),
            Self::right_search(&nums, target, 0..nums.len()),
        ]
    }
}

#[test]
fn do_test() {
    assert_eq!(
        Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8),
        vec![3, 4]
    );
    assert_eq!(
        Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6),
        vec![-1, -1]
    );
    assert_eq!(Solution::search_range(vec![], 0), vec![-1, -1]);

    assert_eq!(
        Solution::search_range(vec![1, 1, 1, 1, 1, 1], 1),
        vec![0, 5]
    );
    assert_eq!(Solution::search_range(vec![1, 1, 1, 1, 1], 1), vec![0, 4]);

    assert_eq!(Solution::search_range(vec![1, 3], 1), vec![0, 0]);
}
