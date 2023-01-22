// Problem description: https://leetcode.com/problems/unique-paths/

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut num_paths = 1usize;
        let m = m as usize;
        let n = n as usize;

        for i in 0..m - 1 {
            num_paths *= i + n;
            num_paths /= i + 1;
        }

        num_paths as i32
    }
}

#[test]
fn do_test() {
    assert_eq!(Solution::unique_paths(3, 7), 28);
    assert_eq!(Solution::unique_paths(7, 3), 28);
    assert_eq!(Solution::unique_paths(3, 2), 3);

    assert_eq!(Solution::unique_paths(23, 12), 193536720);
}
