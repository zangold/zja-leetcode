// Problem description: https://leetcode.com/problems/product-of-array-except-self/

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut output = vec![0; nums.len()];

        let mut product = 1;
        for i in 0..nums.len() {
            product *= nums[i];
            output[i] = product;
        }

        product = 1;
        for i in (0..nums.len()).rev() {
            let v = product * if i == 0 { 1 } else { output[i - 1] };

            product *= nums[i];
            output[i] = v;
        }

        output
    }
}

#[test]
fn do_test() {
    assert_eq!(
        Solution::product_except_self(vec![1, 2, 3, 4]),
        vec![24, 12, 8, 6]
    );
    assert_eq!(
        Solution::product_except_self(vec![-1, 1, 0, -3, 3]),
        vec![0, 0, 9, 0, 0]
    );
}
