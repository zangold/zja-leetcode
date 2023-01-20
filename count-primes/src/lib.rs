// Problem description: https://leetcode.com/problems/count-primes/

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        let n = n as usize;
        let mut is_composite = vec![false; n];

        let mut num_primes = 0;
        let mut i = 2;

        while i < n {
            if !is_composite[i] {
                let mut j = 0;
                while i + j * i < n {
                    is_composite[i + j * i] = true;
                    j += 1;
                }
                num_primes += 1;
            } else {
                i += 1;
            }
        }

        num_primes
    }
}

#[test]
fn do_test() {
    assert_eq!(Solution::count_primes(10), 4);
    assert_eq!(Solution::count_primes(0), 0);
    assert_eq!(Solution::count_primes(1), 0);
    assert_eq!(Solution::count_primes(10000), 1229);
}
