// Problem description: https://leetcode.com/problems/subarray-sums-divisible-by-k/

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn subarrays_div_by_k(mut nums: Vec<i32>, k: i32) -> i32 {
        let mut num_subarrays = 0;
        let mut bins = vec![0; k as usize];

        bins[0] = 1;

        let mut sum = 0;
        for num in nums.iter_mut() {
            sum += *num;
            *num = sum.rem_euclid(k);

            num_subarrays += bins[*num as usize];
            bins[*num as usize] += 1;
        }

        num_subarrays as i32
    }
}

#[test]
fn do_test() {
    assert_eq!(Solution::subarrays_div_by_k(vec![4, 5, 0, -2, -3, 1], 5), 7);
    assert_eq!(Solution::subarrays_div_by_k(vec![5], 9), 0);
    assert_eq!(Solution::subarrays_div_by_k(vec![5], 5), 1);
}
