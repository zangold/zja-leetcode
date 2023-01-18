// https://leetcode.com/problems/power-of-three/

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub const fn is_power_of_three(mut n: i32) -> bool {
        while n > 1 && n % 3 == 0 {
            n /= 3;
        }

        n == 1
    }
}

#[test]
fn do_test() {
    assert!(Solution::is_power_of_three(27));
    assert!(Solution::is_power_of_three(81));
    assert!(!Solution::is_power_of_three(14));
    assert!(!Solution::is_power_of_three(72));
    assert!(!Solution::is_power_of_three(0));
    assert!(!Solution::is_power_of_three(-1));
}
