// https://leetcode.com/problems/permutations/

#![allow(dead_code)]

struct Solution;

impl Solution {
    fn helper(
        nums: &[i32],
        sequence: &mut Vec<i32>,
        sequences: &mut Vec<Vec<i32>>,
        used: &mut Vec<bool>,
    ) {
        let mut all_used = true;

        for i in 0..nums.len() {
            if used[i] {
                continue;
            }

            all_used = false;
            used[i] = true;
            sequence.push(nums[i]);
            Self::helper(nums, sequence, sequences, used);
            sequence.pop();
            used[i] = false;
        }

        if all_used {
            sequences.push(sequence.clone());
        }
    }

    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut sequence = vec![];
        let mut sequences = vec![];
        let mut used = vec![false; nums.len()];

        Self::helper(&nums, &mut sequence, &mut sequences, &mut used);

        sequences
    }
}

#[test]
fn do_test() {
    let nums = vec![1, 2, 3];
    let expected = vec![
        vec![1, 2, 3],
        vec![1, 3, 2],
        vec![2, 1, 3],
        vec![2, 3, 1],
        vec![3, 1, 2],
        vec![3, 2, 1],
    ];

    assert_eq!(Solution::permute(nums), expected);

    let nums = vec![0, 1];
    let expected = vec![vec![0, 1], vec![1, 0]];

    assert_eq!(Solution::permute(nums), expected);

    let nums = vec![1];
    let expected = vec![vec![1]];

    assert_eq!(Solution::permute(nums), expected);
}
