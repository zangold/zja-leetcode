// Problem description: https://leetcode.com/problems/non-decreasing-subsequences/

#![allow(dead_code)]

use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn helper(
        nums: &[i32],
        prefix: &mut Vec<i32>,
        sequences: &mut HashSet<Vec<i32>>,
        min: i32,
    ) {
        if nums.is_empty() {
            if prefix.len() >= 2 {
                sequences.insert(prefix.clone());
            }
        } else {
            Self::helper(&nums[1..], prefix, sequences, min);

            if nums[0] >= min {
                prefix.push(nums[0]);
                Self::helper(&nums[1..], prefix, sequences, i32::max(nums[0], min));
                prefix.pop();
            }
        }
    }

    pub fn find_subsequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut subsequences = HashSet::<Vec<i32>>::new();
        let mut prefix = vec![];

        Self::helper(&nums, &mut prefix, &mut subsequences, i32::MIN);
        subsequences.into_iter().collect()
    }
}

#[test]
fn do_test() {
    let nums = vec![4, 6, 7, 7];

    let mut expected = vec![
        vec![4, 6],
        vec![4, 6, 7],
        vec![4, 6, 7, 7],
        vec![4, 7],
        vec![4, 7, 7],
        vec![6, 7],
        vec![6, 7, 7],
        vec![7, 7],
    ];

    expected.sort_unstable();
    let mut output = Solution::find_subsequences(nums);
    output.sort_unstable();

    assert_eq!(expected, output);

    let nums = vec![4, 4, 3, 2, 1];
    let expected = vec![vec![4, 4]];

    let output = Solution::find_subsequences(nums);
    assert_eq!(expected, output);
}
