// https://leetcode.com/problems/domino-and-tromino-tiling/

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn num_tilings(n: i32) -> i32 {
        // start at a -> n = 0, b -> n = 1
        let mut f_a: usize = 1;
        let mut f_b: usize = 1;

        let mut g_a: usize = 0;
        let mut g_b: usize = 1;

        const MOD: usize = 1_000_000_007;

        for _ in 1..n {
            let f_a_next = f_b;
            let f_b_next = f_a + 2 * g_a + f_b;

            let g_a_next = g_b;
            let g_b_next = g_b + f_b;

            f_a = f_a_next % MOD;
            f_b = f_b_next % MOD;
            g_a = g_a_next % MOD;
            g_b = g_b_next % MOD;
        }

        (f_b % MOD) as i32
    }
}

#[test]
fn do_test() {
    assert_eq!(Solution::num_tilings(1), 1);
    assert_eq!(Solution::num_tilings(2), 2);
    assert_eq!(Solution::num_tilings(3), 5);
    assert_eq!(Solution::num_tilings(4), 11);
}
