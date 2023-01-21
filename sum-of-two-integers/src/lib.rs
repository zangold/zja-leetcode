// Problem description: https://leetcode.com/problems/sum-of-two-integers/

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn get_sum(a: i32, b: i32) -> i32 {
        let mut carry = 0;
        let mut sum = 0;
        for i in 0..32 {
            let bit_sum = ((a >> i) & 1) + ((b >> i) & 1) + carry;
            match bit_sum {
                1 => {
                    carry = 0;
                    sum |= 1 << i;
                }
                2 => {
                    carry = 1;
                }
                3 => {
                    carry = 1;
                    sum |= 1 << i;
                }
                _ => {}
            }
        }
        sum
    }
}

#[test]
fn do_test() {
    for a in -10..=10 {
        for b in -10..=10 {
            assert_eq!(Solution::get_sum(a, b), a + b);
        }
    }
}
