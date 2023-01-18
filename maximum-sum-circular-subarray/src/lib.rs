#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
        let total: i32 = nums.iter().sum();

        let mut highest_inv_sum = i32::MIN;
        let mut current_inv_sum = total;

        let mut highest_sum = i32::MIN;
        let mut current_max_sum = 0i32;
        for n in nums.iter().copied() {
            current_max_sum = n.max(current_max_sum + n);
            highest_sum = highest_sum.max(current_max_sum);

            current_inv_sum = (total - n).max(current_inv_sum - n);
            highest_inv_sum = highest_inv_sum.max(current_inv_sum);
        }

        if highest_inv_sum == 0 {
            return highest_sum;
        }

        highest_sum.max(highest_inv_sum)
    }
}

#[test]
fn do_test() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![1, -2, 3, -2]), 3);
    assert_eq!(Solution::max_subarray_sum_circular(vec![5, -3, 5]), 10);
    assert_eq!(Solution::max_subarray_sum_circular(vec![-3, -2, -3]), -2);
}
