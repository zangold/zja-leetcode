#![allow(dead_code)]

use std::collections::HashSet;

struct Solution;

impl Solution {
    const fn digits_square_sum(mut n: i32) -> i32 {
        let mut sum = 0;
        while n > 0 {
            let digit = n % 10;
            n /= 10;
            sum += digit * digit;
        }

        sum
    }

    pub fn is_happy(mut n: i32) -> bool {
        let mut found_numbers = HashSet::new();

        loop {
            if n == 1 {
                return true;
            } else if found_numbers.contains(&n) {
                return false;
            }

            found_numbers.insert(n);
            n = Self::digits_square_sum(n);
        }
    }
}

#[test]
fn do_test() {
    assert!(Solution::is_happy(19));
    assert!(!Solution::is_happy(2));
}
