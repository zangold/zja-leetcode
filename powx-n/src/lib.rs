#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        let mut result = 1.0_f64;
        let mut x_binary_power = x;

        let n_abs = (n as i64).abs();

        for i in 0..32 {
            if n_abs & (1 << i) > 0 {
                result *= x_binary_power;
            }

            x_binary_power *= x_binary_power;
        }

        if n < 0 {
            1.0_f64 / result
        } else {
            result
        }
    }
}

#[test]
fn do_test() {
    assert_eq!(Solution::my_pow(2.0_f64, -2147483648), 0.0_f64);
    assert_eq!(Solution::my_pow(2.0_f64, 10), 1024.0_f64);
    assert_eq!(Solution::my_pow(2.0_f64, -2), 0.25_f64);
}
