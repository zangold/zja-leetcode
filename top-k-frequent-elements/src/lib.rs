// Problem description: https://leetcode.com/problems/top-k-frequent-elements/

#![allow(dead_code)]

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut binned = HashMap::<i32, i32>::new();

        for num in nums {
            binned.entry(num).and_modify(|n| *n += 1).or_insert(1);
        }

        let mut v = binned.into_iter().collect::<Vec<(i32, i32)>>();

        v.sort_unstable_by_key(|(_num, num_occurrences)| *num_occurrences);

        let n = v.len() as i32;
        v.into_iter()
            .skip((n - k) as usize)
            .map(|(n, _)| n)
            .collect()
    }
}

#[test]
fn do_test() {
    assert_eq!(
        Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2),
        vec![2, 1]
    );
    assert_eq!(Solution::top_k_frequent(vec![1], 1), vec![1]);
}
