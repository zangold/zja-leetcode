// Problem description: https://leetcode.com/problems/pascals-triangle-ii/

#![allow(dead_code)]

struct Solution;

const MAX_ROW: usize = 34;

const fn gen_factorial() -> [u128; MAX_ROW] {
    let mut factorial_table = [1u128; MAX_ROW];
    let mut i = 1;

    while i < MAX_ROW {
        factorial_table[i] = factorial_table[i - 1] * i as u128;
        i += 1;
    }

    factorial_table
}

const FACTORIAL: [u128; MAX_ROW] = gen_factorial();

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let row_index = row_index as usize;
        assert!(row_index < MAX_ROW);

        let n_factorial = FACTORIAL[row_index];

        (0..row_index + 1)
            .map(|i| (n_factorial / (FACTORIAL[i] * FACTORIAL[row_index - i])) as i32)
            .collect()
    }
}

#[test]
fn do_test() {
    assert_eq!(Solution::get_row(0), vec![1]);
    assert_eq!(Solution::get_row(1), vec![1, 1]);
    assert_eq!(Solution::get_row(2), vec![1, 2, 1]);
    assert_eq!(Solution::get_row(3), vec![1, 3, 3, 1]);
    assert_eq!(Solution::get_row(4), vec![1, 4, 6, 4, 1]);
    assert_eq!(Solution::get_row(5), vec![1, 5, 10, 10, 5, 1]);
}
