#![allow(dead_code)]

use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut s = HashSet::new();

        for n in nums {
            if s.contains(&n) {
                s.remove(&n);
            } else {
                s.insert(n);
            }
        }

        *s.iter().next().unwrap()
    }
}

#[test]
fn do_test() {
    assert_eq!(Solution::single_number(vec![2, 2, 1]), 1);
    assert_eq!(Solution::single_number(vec![4, 1, 2, 1, 2]), 4);
    assert_eq!(Solution::single_number(vec![1]), 1);
}
