// Problem description: https://leetcode.com/problems/factorial-trailing-zeroes/

#![allow(dead_code)]

struct Solution;

impl Solution {
    // Computes the power of a given prime in the prime factorization of  n!
    fn prime_power(n: i32, prime: i32) -> i32 {
        let mut exp = 0;
        let mut pow = prime;
        while pow <= n {
            exp += n / pow;
            pow *= prime;
        }

        exp
    }

    pub fn trailing_zeroes(n: i32) -> i32 {
        i32::min(Self::prime_power(n, 2), Self::prime_power(n, 5))
    }
}

#[test]
fn do_test() {
    assert_eq!(Solution::trailing_zeroes(3), 0);
    assert_eq!(Solution::trailing_zeroes(5), 1);
    assert_eq!(Solution::trailing_zeroes(0), 0);

    assert_eq!(Solution::trailing_zeroes(40), 9);
}
